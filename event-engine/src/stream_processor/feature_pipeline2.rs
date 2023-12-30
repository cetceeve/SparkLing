use crate::Vehicle;
use chrono::{NaiveDateTime, Timelike, Datelike};
use serde::{Deserialize, Serialize};

use super::ProcessingStep;

static STOP_DETECT_DISTANCE: f32 = 100.0;

/// Features for predicting the speed relative to the schedule for the next stops.
/// This is a sequence to sequence problem.
/// Input features: ratio of (real time between past stops / scheduled time)
/// example: [1.2, 1.0, 1.12, 0.78, 1.1]
/// Output (labels): same ratio, but for future stops
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TrainingFeatures {
    // maybe include these
    // pub lat: f32,
    // pub lng: f32,
    pub route_id: String,
    pub direction_id: u8,
    /// the day of the week
    pub weekday: u32,
    /// the hour of day
    pub hour: u32,
    /// the sequence
    pub past_time_ratios: Vec<f32>,
    /// labels
    pub future_time_ratios: Vec<f32>,
}

pub struct TrainingFeatureExtractor {}

impl TrainingFeatureExtractor {
    pub async fn init() -> Self {
        Self {}
    }
}

impl ProcessingStep for TrainingFeatureExtractor {
    fn apply(&mut self, vehicle: &mut Vehicle, _low_watermark: u64) -> bool {
        // get metadata if there
        if let Some(ref vehicle_metadata) = vehicle.metadata {
            if let Some(ref real_stop_times) = vehicle_metadata.real_stop_times {
                let stops = &vehicle_metadata.stops;
                if real_stop_times.len() == 0 || *real_stop_times.last().unwrap() == None {
                    return true // we only generate training features once we reached the last stop
                }

                // calculate ratios
                let mut ratios = vec![];
                let mut real_stop_times = real_stop_times.iter().map(|x| x.unwrap_or(0)).collect::<Vec<u64>>();
                let mut i = 1;
                while i < stops.len() {
                    if real_stop_times[i-1] == 0 {
                        ratios.push(1.0);
                    } else {
                        let mut j = 0;
                        while real_stop_times[i+j] == 0 {
                            j += 1;
                        }
                        let ratio = (real_stop_times[i+j] as f32 - real_stop_times[i-1] as f32) / (stops[i+j].scheduled_time as f32 - stops[i-1].scheduled_time as f32);
                        for _ in 0..j {
                            ratios.push(ratio);
                        }
                    }
                    i += 1;
                }
                
                // create multiple training samples from one trip
                for i in 0..(stops.len()-1) {
                    let datetime = NaiveDateTime::from_timestamp_millis(stops[i+1].scheduled_time as i64 * 1000).unwrap();
                    let features = TrainingFeatures {
                        // lat: vehicle.lat,
                        // lng: vehicle.lng,
                        route_id: vehicle_metadata.route_id.clone().unwrap(),
                        direction_id: vehicle_metadata.direction_id.unwrap(),
                        weekday: datetime.date().weekday().num_days_from_monday(),
                        hour: datetime.time().hour(),
                        past_time_ratios: ratios[..i].to_vec(),
                        future_time_ratios: ratios[i..].to_vec(),
                    };
                    // TODO: write features
                }
            }
        }
        true
    }
}
