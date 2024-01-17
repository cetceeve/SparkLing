use types::{Vehicle, VehicleMetadata};
use tokio::sync::mpsc;
use redis::AsyncCommands;

mod rt_gtfs_client;
mod stream_processor;
mod training_data_client;

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
