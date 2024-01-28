use types::{Vehicle, get_trip_metadata_blocking};

use super::ProcessingStep;

pub struct MetadataJoiner {}

impl MetadataJoiner {
    pub fn init() -> Self {
        Self {}
    }
}

impl ProcessingStep for MetadataJoiner {
    fn apply(&mut self, vehicle: &mut Vehicle) -> (bool, Option<(String, Vec<u8>)>) {
        if let Some(ref trip_id) = vehicle.trip_id {
            vehicle.metadata = get_trip_metadata_blocking(trip_id);
            (true, None)
        } else {
            (false, None)
        }
    }
}
