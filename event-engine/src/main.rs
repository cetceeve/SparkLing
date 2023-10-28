use serde::{Deserialize, Serialize};
use tokio::sync::mpsc;

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
}

#[tokio::main]
async fn main() {
    let (input_sender, input_receiver) = mpsc::unbounded_channel::<Vehicle>();
    let (output_sender, mut output_receiver) = mpsc::unbounded_channel::<Vehicle>();
    tokio::task::spawn(async move {
        stream_processor::process_vehicle_stream(input_receiver, output_sender).await
    });
    rt_gtfs_client::start_vehicle_position_clients(input_sender);

    loop {
        let item = output_receiver.recv().await.unwrap();
        println!("{:?}", item)
    }
}
