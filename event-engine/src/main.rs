use serde::{Deserialize, Serialize};
use tokio::sync::mpsc;
use std::time::Duration;
use redis::AsyncCommands;

mod rt_gtfs_client;
mod stream_processor;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Vehicle {
    pub id: String,
    pub lng: f32,
    pub lat: f32,
    pub timestamp: u64,
    pub trip_id: Option<String>,
    pub metadata: Option<VehicleMetadata>,
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
    pub direction_id: Option<u32>,
}

#[tokio::main]
async fn main() {
    let (input_sender, input_receiver) = mpsc::unbounded_channel::<Vehicle>();
    let (output_sender, mut output_receiver) = mpsc::unbounded_channel::<Vehicle>();
    tokio::task::spawn(async move {
        stream_processor::process_vehicle_stream(input_receiver, output_sender).await
    });
    rt_gtfs_client::start_vehicle_position_clients(input_sender);

    // start redis client
    let redis_client = redis::Client::open("redis://sparkling-redis/").unwrap();
    let mut redis_conn = redis_client.get_tokio_connection().await.unwrap();

    loop {
        let item = output_receiver.recv().await.unwrap();
        let serialized = serde_json::to_vec(&item).expect("Serialisation failed.");
        // TODO: can we not block the loop on each item?
        redis_conn.publish::<_,_,()>("realtime-with-metadata", serialized).await.unwrap();
    }
}
