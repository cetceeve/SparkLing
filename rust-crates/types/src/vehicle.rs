use crate::VehicleMetadata;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Vehicle {
    pub id: String,
    pub lng: f32,
    pub lat: f32,
    pub timestamp: u64,
    pub trip_id: Option<String>,
    pub metadata: Option<VehicleMetadata>,
}

impl Vehicle {
    /// The serialization used for communication with the frontend
    pub fn serialize_for_frontend(&self) -> Vec<u8> {
        let mut map = Map::new();
        map.insert("id".to_string(), Value::String(self.id.to_string()));
        map.insert("lat".to_string(), Value::from(self.lat));
        map.insert("lng".to_string(), Value::from(self.lng));
        map.insert("trip_id".to_string(), Value::from(self.trip_id.clone()));
        if let Some(m) = self.metadata.clone() {
            if let Some(route_type) = m.route_type {
                map.insert("route_type".to_string(), Value::from(route_type));
            }
            if let Some(agency_name) = m.agency_name {
                map.insert("agency_name".to_string(), Value::from(agency_name));
            }
            if let Some(route_short_name) = m.route_short_name {
                map.insert("route_short_name".to_string(), Value::from(route_short_name));
            }
            if let Some(route_long_name) = m.route_long_name {
                map.insert("route_long_name".to_string(), Value::from(route_long_name));
            }
            if let Some(trip_headsign) = m.trip_headsign {
                map.insert("trip_headsign".to_string(), Value::from(trip_headsign));
            }
            if let Some(stops) = m.stops {
                map.insert("stops".to_string(), Value::from(stops));
            }
        }
        serde_json::to_vec(&Value::Object(map)).unwrap()
    }
}
