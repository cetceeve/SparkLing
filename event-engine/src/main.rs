use serde::{Deserialize, Serialize};
use tokio::sync::mpsc;
use std::fmt::Write;
use std::time::Duration;
use kafka::producer::{Producer, Record, RequiredAcks};

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

    // send json serialized messages to kafka
    let mut producer =
        Producer::from_hosts(vec!("sparkling-kafka-bootstrap:9092".to_owned()))
            .with_ack_timeout(Duration::from_secs(1))
            .with_required_acks(RequiredAcks::One)
            .create()
            .unwrap();

    loop {
        let item = output_receiver.recv().await.unwrap();
        let serialized = serde_json::to_vec(&item).expect("Serialisation failed.");
        producer.send(&Record::from_value("realtime-with-metadata", serialized)).unwrap();
        println!("{:?}", item)
    }
}
