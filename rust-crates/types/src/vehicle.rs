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
        }
        serde_json::to_vec(&Value::Object(map)).unwrap()
    }
}
