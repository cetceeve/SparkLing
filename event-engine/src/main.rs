use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use tokio::sync::mpsc;
use redis::AsyncCommands;

mod rt_gtfs_client;
mod stream_processor;
mod training_data_client;

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

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VehicleMetadata {
    pub trip_id: String,
    pub route_type: Option<u64>,
    pub agency_name: Option<String>,
    pub route_short_name: Option<String>,
    pub route_long_name: Option<String>,
    pub trip_headsign: Option<String>,
    pub shape_id: Option<u64>,
    pub route_id: Option<String>, // TODO: make some of these not optional
    pub direction_id: Option<u8>,
    pub stops: Option<Vec<Stop>>, // sorted by stop_sequence
    pub real_stop_times: Option<Vec<Option<u64>>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Stop {
    pub stop_sequence: u16,
    pub stop_id: String,
    pub stop_name: String,
    pub arrival_time: String,
    pub departure_time: String,
    pub stop_lat: f32,
    pub stop_lon: f32,
    pub shape_dist_traveled: f32,
}

/// Used for serialization to send to the frontend
impl From<Stop> for Value {
    fn from(value: Stop) -> Self {
        let mut map = Map::new();
        map.insert("stop_sequence".to_string(), Value::from(value.stop_sequence));
        map.insert("stop_name".to_string(), Value::from(value.stop_name));
        map.insert("stop_lon".to_string(), Value::from(value.stop_lon));
        map.insert("stop_lat".to_string(), Value::from(value.stop_lat));
        map.insert("arrival_time".to_string(), Value::from(value.arrival_time));
        Value::Object(map)
    }
}

#[tokio::main]
async fn main() {
    let (input_sender, input_receiver) = mpsc::unbounded_channel::<Vehicle>();
    let (output_sender, mut output_receiver) = mpsc::unbounded_channel::<Vehicle>();
    let mut processor = stream_processor::StreamProcessor::default().await;

    // let mut training_client = training_data_client::TrainingDataClient::default().await;
    tokio::task::spawn(async move {
        processor.run(input_receiver, output_sender).await
    });
    rt_gtfs_client::start_vehicle_position_clients(input_sender);

    // Uncomment here to run training
    // let mut training_client = training_data_client::TrainingDataClient::default().await;
    // tokio::task::spawn(async move {
    //     training_client.run(input_sender).await;
    // });

    // start redis client
    let redis_client = redis::Client::open("redis://sparkling-redis/").unwrap();
    let mut redis_conn = redis_client.get_tokio_connection().await.unwrap();

    loop {
        let vehicle = output_receiver.recv().await.unwrap();
        let serialized = vehicle.serialize_for_frontend();
        // TODO: can we not block the loop on each item?
        redis_conn.publish::<_,_,()>("realtime-with-metadata", serialized).await.unwrap();
    }
}
