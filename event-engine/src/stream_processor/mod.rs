use crate::Vehicle;
use tokio::sync::mpsc::{Receiver, Sender};

mod feature_pipeline;
mod inference_pipeline;
mod metadata_join;

use metadata_join::MetadataJoiner;

pub trait ProcessingStep: Send {
    /// May mutate the vehicle, or remove it from the stream, by returning `false`.
    /// low_watermark is the lowest event timestamp a following vehicle may have.
    fn apply(&mut self, vehicle: &mut Vehicle, low_watermark: u64) -> bool;
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
        let metadata_joiner = MetadataJoiner::init().await;
        processor.register_step(Box::new(metadata_joiner));
        processor
    }

    pub async fn run(
        &mut self,
        mut receiver: Receiver<Vehicle>,
        sender: Sender<Vehicle>,
    ) {
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
                let keep_vehicle = step.apply(&mut vehicle, self.low_watermark);
                if !keep_vehicle {
                    continue 'EVENT_LOOP
                }
            }

            sender.send(vehicle).await.expect("broken internal channel");
        }
    }
}
