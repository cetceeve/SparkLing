use types::{Vehicle, VehicleMetadata, download_metadata_table_async};
use std::collections::HashMap;

use super::ProcessingStep;

pub struct MetadataJoiner {
    table: HashMap<String, VehicleMetadata>,
}

impl MetadataJoiner {
    pub async fn init() -> Self {
        Self {
            table: download_metadata_table_async().await.unwrap(),
        }
    }
}

impl ProcessingStep for MetadataJoiner {
    fn apply(&mut self, vehicle: &mut Vehicle) -> (bool, Option<(String, Vec<u8>)>) {
        if let Some(ref trip_id) = vehicle.trip_id {
            vehicle.metadata = self.table.get(trip_id).map(|x| x.to_owned());
            (true, None)
        } else {
            (false, None)
        }
    }
}
