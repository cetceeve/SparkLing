use crate::Vehicle;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};

mod feature_pipeline;
mod inference_pipeline;
mod metadata_join;
use metadata_join::MetadataJoiner;

pub async fn process_vehicle_stream(
    mut receiver: UnboundedReceiver<Vehicle>,
    sender: UnboundedSender<Vehicle>,
) {
    let metadata_joiner = MetadataJoiner::init().await;
    loop {
        let mut vehicle = receiver.recv().await.expect("broken internal channel");

        // precessing steps
        metadata_joiner.join_metadata(&mut vehicle);

        sender.send(vehicle).expect("broken internal channel");
    }
}
