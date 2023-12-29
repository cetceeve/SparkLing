use std::time::Duration;
use crate::Vehicle;
use tokio::{sync::mpsc::{UnboundedReceiver, UnboundedSender}, time::Instant};

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
    /// used together with processing_time_widow and old_ts_pairs to calculate low_watermark
    processing_event_times: Vec<(Instant, u64)>,
    old_ts_pairs: usize,
    processing_time_widow: Duration,
    /// registered processing steps
    processing_steps: Vec<Box<dyn ProcessingStep>>,
}

impl StreamProcessor {
    pub fn init(processing_time_widow: Duration) -> Self {
        Self {
            processing_time_widow,
            low_watermark: 0,
            processing_event_times: vec![],
            old_ts_pairs: 0,
            processing_steps: vec![],
        }
    }

    pub fn register_step(&mut self, step: Box<dyn ProcessingStep>) {
        self.processing_steps.push(step);
    }

    pub async fn default() -> Self {
        let mut processor = Self::init(Duration::from_secs(4));
        let metadata_joiner = MetadataJoiner::init().await;
        processor.register_step(Box::new(metadata_joiner));
        processor
    }

    pub async fn run(
        &mut self,
        mut receiver: UnboundedReceiver<Vehicle>,
        sender: UnboundedSender<Vehicle>,
    ) {
        'EVENT_LOOP: loop {
            let mut vehicle = receiver.recv().await.expect("broken internal channel");

            // update low_watermark
            self.low_watermark = {
                let now = Instant::now();
                let window_barrier = now - self.processing_time_widow;
                self.processing_event_times.push((now, vehicle.timestamp));
                let mut recompute = false;
                while self.processing_event_times[self.old_ts_pairs].0 < window_barrier {
                    if self.processing_event_times[self.old_ts_pairs].1 == self.low_watermark {
                        recompute = true;
                    }
                    self.old_ts_pairs += 1;
                }
                if recompute {
                    self.processing_event_times.drain(0..self.old_ts_pairs);
                    let mut new_low = u64::MAX;
                    for (_, event_time) in &self.processing_event_times {
                        if *event_time < new_low {
                            new_low = *event_time;
                        }
                    }
                    new_low
                } else {
                    self.low_watermark
                }
            };

            // precessing steps
            for step in &mut self.processing_steps {
                let keep_vehicle = step.apply(&mut vehicle, self.low_watermark);
                if !keep_vehicle {
                    continue 'EVENT_LOOP
                }
            }

            sender.send(vehicle).expect("broken internal channel");
        }
    }
}
