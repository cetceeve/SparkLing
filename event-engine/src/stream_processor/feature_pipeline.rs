use std::fs::{OpenOptions, File};

use crate::Vehicle;
use chrono::{NaiveDateTime, Timelike, Datelike, NaiveTime, Duration};
use serde::{Deserialize, Serialize};

use super::ProcessingStep;

/// Features for predicting the speed relative to the schedule for the next stops.
/// This is a sequence to sequence problem.
/// Input features: ratio of (real time between past stops / scheduled time)
/// example: [1.2, 1.0, 1.12, 0.78, 1.1]
/// Output (labels): same ratio, but for future stops
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TrainingFeatures {
    // maybe include these
    // pub lat: f64,
    // pub lng: f64,
    pub trip_id: String,
    pub route_id: String,
    pub direction_id: u8,
    pub route_type: u64,
    /// the day of the week
    pub weekday: u32,
    /// the hour of day
    pub hour: u32,
    /// the stop_sequence of the startingn stop of the section
    pub stop_sequence: u32,
    /// the scheduled section length in seconds
    pub scheduled_duration: u32,
    /// the same, but for the next timestep
    pub next_scheduled_duration: u32,
    /// the ratio of true duration to scheduled duration
    pub ratio: f64,
}

pub struct TrainingFeatureExtractor {
    writer: csv::Writer<File>,
}

impl TrainingFeatureExtractor {
    pub fn init() -> Self {
        let file = OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open("/training-data/training_data.csv")
            .unwrap();
        let writer = csv::Writer::from_writer(file);
        Self { writer }
    }
}

impl ProcessingStep for TrainingFeatureExtractor {
    fn apply(&mut self, vehicle: &mut Vehicle, _low_watermark: u64) -> bool {
        // get metadata if there
        if let Some(ref vehicle_metadata) = vehicle.metadata {
            if let (Some(real_stop_times), Some(stops)) = (&vehicle_metadata.real_stop_times, &vehicle_metadata.stops) {
                if real_stop_times.len() == 0 || *real_stop_times.last().unwrap() == None {
                    return true // we only generate training features once we reached the last stop
                }

                // convert scheduled stop times to useful timestamps
                let end_date = NaiveDateTime::from_timestamp_millis(vehicle.timestamp as i64 * 1000).unwrap().date();
                let mut scheduled_times = stops.iter().map(|x| end_date.and_time(NaiveTime::parse_from_str(&x.arrival_time, "%H:%M:%S").unwrap())).collect::<Vec<NaiveDateTime>>();
                // if the trip ended past midnight, we need to fix the scheduled dates for stops before midnight
                for i in (1..scheduled_times.len()).rev() {
                    if scheduled_times[i] < scheduled_times[i-1] {
                        scheduled_times[i-1] -= Duration::days(1);
                    }
                }
                let scheduled_timestamps = scheduled_times.iter().map(|x| x.timestamp()).collect::<Vec<i64>>();

                // calculate ratios
                let mut ratios = vec![];
                let mut scheduled_durations = vec![];
                let real_stop_times = real_stop_times.iter().map(|x| x.unwrap_or(0)).collect::<Vec<u64>>();
                let mut i = 1;
                while i < stops.len() {
                    if real_stop_times[i-1] == 0 {
                        ratios.push(1.0);
                        scheduled_durations.push(scheduled_timestamps[i] - scheduled_timestamps[i-1]);
                        i += 1;
                    } else {
                        let mut j = 0;
                        while real_stop_times[i+j] == 0 {
                            j += 1;
                        }
                        let mut ratio = (real_stop_times[i+j] as f64 - real_stop_times[i-1] as f64) / (scheduled_timestamps[i+j] as f64 - scheduled_timestamps[i-1] as f64);
                        if ratio == 0.0 {
                            ratio = 1.0;
                        }
                        if ratio > 50.0 || ratio < 0.0 {
                            println!("DEBUG: bad ratio: {:?} -> {:?}, {:?} - {:?}, {:?}", ratio, real_stop_times[i+j], real_stop_times[i-1], scheduled_timestamps[i+j], scheduled_timestamps[i-1])
                        }
                        let p = i;
                        for q in 0..=j {
                            ratios.push(ratio);
                            scheduled_durations.push(scheduled_timestamps[p+q] - scheduled_timestamps[p+q-1]);
                            i += 1;
                        }
                    }
                }
                scheduled_durations.push(0);
                
                // create multiple training samples from one trip
                for i in 0..(stops.len()-1) {
                    let datetime = scheduled_times[i+1].clone();
                    let features = TrainingFeatures {
                        // lat: vehicle.lat,
                        // lng: vehicle.lng,
                        trip_id: vehicle.trip_id.clone().unwrap(),
                        route_id: vehicle_metadata.route_id.clone().unwrap(),
                        direction_id: vehicle_metadata.direction_id.unwrap(),
                        route_type: vehicle_metadata.route_type.unwrap(),
                        weekday: datetime.date().weekday().num_days_from_monday(),
                        hour: datetime.time().hour(),
                        stop_sequence: i as u32 + 1,
                        ratio: ratios[i],
                        scheduled_duration: scheduled_durations[i] as u32,
                        next_scheduled_duration: scheduled_durations[i+1] as u32,
                    };
                    self.writer.serialize(features).unwrap();
                }
                self.writer.flush().unwrap();
            }
        }
        true
    }
}
