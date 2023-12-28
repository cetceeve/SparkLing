use crate::{Vehicle, VehicleMetadata};
use anyhow::Result;
use csv;
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use std::f32::consts::PI;

static STOP_DETECT_DISTANCE: f32 = 100.0;

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

    pub fn extract_features(&mut self, vehicle: &mut Vehicle) {
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
            // check if next stop is reached
            if let Some(ref mut stops) = vehicle_metadata.stops {
                for stop in stops {
                    if stop.real_time == None {
                        if measure_distance(stop.stop_lat, stop.stop_lon, vehicle.lat, vehicle.lng) < STOP_DETECT_DISTANCE {
                            // TODO: attach previous stops' real_time before
                            stop.real_time = Some(vehicle.timestamp);
                            self.last_vehicle_at_stop.insert(
                                (stop.stop_id.clone(), vehicle_metadata.route_id.clone().unwrap(), vehicle_metadata.direction_id.clone().unwrap()),
                                (Rc::clone(&vehicle_rc), vehicle.timestamp)
                            );
                        }
                    }
                }
            }
        }

        // in this function, we used the Rc<RefCell<Vehicle>>> we created in the beginning
        // now we need to update the vehicle in the stream from it
        *vehicle = vehicle_rc.borrow().clone();
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
