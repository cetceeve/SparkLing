use std::collections::HashMap;
use std::f32::consts::PI;

use crate::Vehicle;
use super::ProcessingStep;

static STOP_DETECT_DISTANCE: f32 = 100.0;

pub struct StopDetector {
    /// mapping of: vehicle.id -> (trip_id, real_stop_times)
    real_stop_times: HashMap<String, (String, Vec<Option<u64>>)>,
}

impl StopDetector {
    pub fn init() -> Self {
        Self {
            real_stop_times: Default::default(),
        }
    }
}

impl ProcessingStep for StopDetector {
    fn apply(&mut self, vehicle: &mut Vehicle, _low_watermark: u64) -> bool {
        // we can only do anything if metadata is there
        if let Some(vehicle_metadata) = &mut vehicle.metadata {
            // get mutable reference to known real stop times for the vehicle
            let (_, real_stop_times) = if let Some(times) = self.real_stop_times.get_mut(&vehicle.id) {
                if times.0 != vehicle.trip_id.unwrap() {
                    self.real_stop_times.insert(vehicle.id.clone(), (vehicle.trip_id.clone().unwrap(), vec![None; vehicle_metadata.stops.len()]));
                    self.real_stop_times.get_mut(&vehicle.id).unwrap()
                } else {
                    times
                }
            } else {
                self.real_stop_times.insert(vehicle.id.clone(), (vehicle.trip_id.clone().unwrap(), vec![None; vehicle_metadata.stops.len()]));
                self.real_stop_times.get_mut(&vehicle.id).unwrap()
            };

            // record time if a new stop is reached
            for (i, stop) in vehicle_metadata.stops.iter().enumerate() {
                if real_stop_times[i] == None {
                    if measure_distance(stop.stop_lat, stop.stop_lon, vehicle.lat, vehicle.lng) < STOP_DETECT_DISTANCE {
                        real_stop_times[i] = Some(vehicle.timestamp);
                    }
                }
            }
            // attach real times to vehicle
            vehicle_metadata.real_stop_times = Some(real_stop_times.clone());
        }
        true // we don't filter here
    }
}

/// Returns the distance in meters between two lat lon coordinates.
fn measure_distance(lat1: f32, lon1: f32, lat2: f32, lon2: f32) -> f32 {
    const R: f32 = 6378.137; // Radius of earth in KM
    let d_lat = lat2 * PI / 180.0 - lat1 * PI / 180.0;
    let d_lon = lon2 * PI / 180.0 - lon1 * PI / 180.0;
    let a = (d_lat/2.0).sin() * (d_lat/2.0).sin() +
    (lat1 * PI / 180.0).cos() * (lat2 * PI / 180.0).cos() *
    (d_lon/2.0).sin() * (d_lon/2.0).sin();
    let c = 2.0 * a.sqrt().atan2((1.0-a).sqrt());
    let d = R * c;
    return d * 1000.0; // meters
}
