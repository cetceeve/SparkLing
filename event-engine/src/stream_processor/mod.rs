use crate::Vehicle;
use tokio::sync::mpsc::{Receiver, Sender};
use redis::AsyncCommands;

mod metadata_join;
mod stop_detector;
mod feature_pipeline;
mod inference_pipeline;

use self::feature_pipeline::{TrainingFeatureExtractor, InferenceFeatureExtractor};
use self::metadata_join::MetadataJoiner;
use self::stop_detector::StopDetector;

pub trait ProcessingStep: Send {
    /// May mutate the vehicle, or remove it from the stream, by returning `(false, _)`.
    /// May return an arbitrary binary message to publish to redis on a topic by returning `(_, Some(topic_name, msg))`
    ///
    /// low_watermark is the lowest event timestamp a following vehicle may have.
    fn apply(&mut self, vehicle: &mut Vehicle, low_watermark: u64) -> (bool, Option<(String, Vec<u8>)>);
}

/// Processes a steam of Vehicle events.
pub struct StreamProcessor {
    /// the lowest event timestamp that may appear in following events
    low_watermark: u64,
    /// used to calculate low_watermark
    processing_time_widow: u64,
    /// registered processing steps
    processing_steps: Vec<Box<dyn ProcessingStep>>,
}

impl StreamProcessor {
    pub fn init(processing_time_widow: u64) -> Self {
        Self {
            processing_time_widow,
            low_watermark: 0,
            processing_steps: vec![],
        }
    }

    pub fn register_step(&mut self, step: Box<dyn ProcessingStep>) {
        self.processing_steps.push(step);
    }

    pub async fn default() -> Self {
        let mut processor = Self::init(5);
        processor.register_step(Box::new(MetadataJoiner::init().await));
        processor.register_step(Box::new(StopDetector::init()));
        processor.register_step(Box::new(TrainingFeatureExtractor::init()));
        processor.register_step(Box::new(InferenceFeatureExtractor::init()));
        processor
    }

    pub async fn run(
        &mut self,
        mut receiver: Receiver<Vehicle>,
        sender: Sender<Vehicle>,
    ) {
        let redis_client = redis::Client::open("redis://sparkling-redis/").unwrap();
        let mut redis_conn = redis_client.get_tokio_connection().await.unwrap();
        'EVENT_LOOP: loop {
            let mut vehicle = receiver.recv().await.expect("broken internal channel");

            // filter old events
            if vehicle.timestamp < self.low_watermark {
                continue
            }
            // update low_watermark
            self.low_watermark = self.low_watermark.max(vehicle.timestamp - self.processing_time_widow);

            // precessing steps
            for step in &mut self.processing_steps {
                let (keep_vehicle, message) = step.apply(&mut vehicle, self.low_watermark);
                if let Some((topic_name, data)) = message {
                    redis_conn.publish::<_,_,()>(topic_name, data).await.unwrap();
                }
                if !keep_vehicle {
                    continue 'EVENT_LOOP
                }
            }

            sender.send(vehicle).await.expect("broken internal channel");
        }
    }
}
