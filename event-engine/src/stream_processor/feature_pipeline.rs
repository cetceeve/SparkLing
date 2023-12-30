use crate::{Vehicle, VehicleMetadata};
use anyhow::{Result, bail};
use chrono::{NaiveDateTime, Timelike, Datelike};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use std::f32::consts::PI;

static STOP_DETECT_DISTANCE: f32 = 100.0;

/// Features for predicting the real time it will take to get from stop A to stop B.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Features {
    // maybe include these
    pub lat: f32,
    pub lng: f32,
    pub route_id: String,
    pub stop_to_predict: String,

    // the following are features we can compute that seem good to me at first glance

    /// the day of the week
    pub weekday: u32,
    /// the hour of day
    pub hour: u32,
    /// the time it takes according to the schedule to get from stop A to B
    pub scheduled_duration: u64,
    /// the time it took the previous vehicle to get from stop A to B
    pub prev_vehicle_duration: u64,
    /// the amount of time already passed since the last stop
    pub passed_duration: u64,
    /// the delay the vehicle had at stop A
    pub delay_at_last_stop: u64,
    /// the delay the previous vehicle had at stop A
    pub prev_vehicle_delay_at_last_stop: u64,
}

/// Features for predicting the speed relative to the schedule for the next stops.
/// This is a sequence to sequence problem.
/// Input features: ratio of (real time between past stops / scheduled time)
/// example: [1.2, 1.0, 1.12, 0.78, 1.1]
/// Output (labels): same ratio, but for future stops
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Features2 {
    // maybe include these
    pub lat: f32,
    pub lng: f32,
    pub route_id: String,
    pub direction_id: u8,
    /// the day of the week
    pub weekday: u32,
    /// the hour of day
    pub hour: u32,
    // TODO: sequence
}

pub struct FeatureExtractor {
    // metadata_table: HashMap<String, VehicleMetadata>,

    /// Mapping of vehicle_id -> vehicle
    vehicles: HashMap<String, Rc<RefCell<Vehicle>>>,
    /// Mapping of (stop_id, route_id, direction_id) -> (vehicle, timestamp)
    last_vehicle_at_stop: HashMap<(String, String, u8), (Rc<RefCell<Vehicle>>, u64)>, // update only on arrival
    next_vehicle: HashMap<String, Rc<RefCell<Vehicle>>>,
    prev_vehicle: HashMap<String, Rc<RefCell<Vehicle>>>,
}

// data structures:
// route_id,direction_id -> [vehicles]
// vehicle.id,route_id,direction_id -> (last_stop, time)

// computations:
// vehicle.trip_id -> metadata.trip_id
// vehicle.latlng -> current_stop   // update last_stop, next_stop

// feature: 
// prev vehicle real arrival time at the stop we want to predict minus
// the arrival time of the previous vehicle at the stop before


// we want a column in the metadata table that contains a list of the stop times for that trip
// we also want to keep in memory, updated stop times for past stops,
// we can use these, or something from it, like the current real delay as a feature
// also, it's just cool to have by itself in the app as information to display

impl FeatureExtractor {
    pub async fn init() -> Self {
        Self {
            // metadata_table: download_metadata_table().await.unwrap(),
            vehicles: Default::default(),
            last_vehicle_at_stop: Default::default(),
            next_vehicle: Default::default(),
            prev_vehicle: Default::default(),
        }
    }

    pub fn extract_features(&mut self, vehicle: &mut Vehicle) -> Result<Features> {
        // update vehicle cache
        let vehicle_rc = if let Some(vehicle_rc) = self.vehicles.get_mut(&vehicle.id) {
            *vehicle_rc.borrow_mut() = vehicle.clone();
            Rc::clone(vehicle_rc)
        } else {
            let vehicle_rc = Rc::new(RefCell::new(vehicle.clone()));
            self.vehicles.insert(vehicle.id.clone(), Rc::clone(&vehicle_rc));
            vehicle_rc
        };

        // get metadata if there
        if let Some(ref mut vehicle_metadata) = vehicle_rc.borrow_mut().metadata {
            if let Some(stops) = &mut vehicle_metadata.stops {
                // check if next stop is reached
                for stop in stops.iter_mut() {
                    if stop.real_time == None {
                        if measure_distance(stop.stop_lat, stop.stop_lon, vehicle.lat, vehicle.lng) < STOP_DETECT_DISTANCE {
                            // TODO: attach previous stops' real_time before
                            stop.real_time = Some(vehicle.timestamp);
                            if let Some((prev, _ts)) = self.last_vehicle_at_stop.get(&(stop.stop_id.clone(), vehicle_metadata.route_id.clone().unwrap(), vehicle_metadata.direction_id.clone().unwrap())) {
                                self.prev_vehicle.insert(vehicle.id.clone(), Rc::clone(prev));
                                self.next_vehicle.insert(prev.borrow().id.clone(), Rc::clone(&vehicle_rc));
                            }
                            self.last_vehicle_at_stop.insert(
                                (stop.stop_id.clone(), vehicle_metadata.route_id.clone().unwrap(), vehicle_metadata.direction_id.clone().unwrap()),
                                (Rc::clone(&vehicle_rc), vehicle.timestamp)
                            );
                        }
                    }
                }

                // now we begin constructing the actual features
                let mut stop_to_predict = None;
                let mut prev_stop = None;
                let mut schedule_time_a: u64 = 0;
                let mut schedule_time_b: u64 = 0;
                let mut real_time_a: u64 = 0;
                for stop in stops.iter_mut() {
                    if let Some(real_time) = stop.real_time {
                        prev_stop = Some(stop.stop_id.clone());
                        stop_to_predict = None;
                        schedule_time_a = stop.scheduled_time;
                        real_time_a = real_time;
                    } else if prev_stop != None && stop_to_predict == None {
                        stop_to_predict = Some(stop.stop_id.clone());
                        schedule_time_b = stop.scheduled_time;
                    }
                }
                if stop_to_predict == None || prev_stop == None {
                    bail!("missing previous stop or stop to predict");
                }
                let stop_to_predict = stop_to_predict.unwrap();
                let prev_stop = prev_stop.unwrap();

                // find the times for the previous vehicle
                let mut prev_vehicle_schedule_time_a: u64 = 0;
                let mut prev_vehicle_real_time_a: u64 = 0;
                let mut prev_vehicle_real_time_b: u64 = 0;
                if let Some(prev_vehicle) = self.prev_vehicle.get(&vehicle.id) {
                    if let Some(ref prev_vehicle_meta) = prev_vehicle.borrow().metadata {
                        if let Some(ref prev_vehicle_stops) = prev_vehicle_meta.stops {
                            for stop in prev_vehicle_stops {
                                if stop.stop_id == prev_stop {
                                    if let Some(real_time) = stop.real_time {
                                        prev_vehicle_schedule_time_a = stop.scheduled_time;
                                        prev_vehicle_real_time_a = real_time;
                                    }
                                }
                                if stop.stop_id == stop_to_predict {
                                    if let Some(real_time) = stop.real_time {
                                        prev_vehicle_real_time_b = real_time;
                                    }
                                }
                            }
                        }
                    }
                }
                if prev_vehicle_real_time_a == 0 || prev_vehicle_real_time_b == 0 {
                    bail!("missing previous vehicle data");
                }

                let datetime = NaiveDateTime::from_timestamp_millis(vehicle.timestamp as i64 * 1000).unwrap();
                let features = Features {
                    lat: vehicle.lat,
                    lng: vehicle.lng,
                    route_id: vehicle_metadata.route_id.clone().unwrap(),
                    stop_to_predict,
                    scheduled_duration: schedule_time_b - schedule_time_a,
                    prev_vehicle_duration: prev_vehicle_real_time_b - prev_vehicle_real_time_a,
                    passed_duration: vehicle.timestamp - real_time_a,
                    delay_at_last_stop: real_time_a - schedule_time_a,
                    prev_vehicle_delay_at_last_stop: prev_vehicle_real_time_a - prev_vehicle_schedule_time_a,
                    weekday: datetime.date().weekday().num_days_from_monday(),
                    hour: datetime.time().hour(),
                };
                return Ok(features);
            }
        }

        // in this function, we used the Rc<RefCell<Vehicle>>> we created in the beginning
        // now we need to update the vehicle in the stream from it
        *vehicle = vehicle_rc.borrow().clone();
        bail!("could not create features");
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
