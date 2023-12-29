use crate::{Vehicle, VehicleMetadata};
use anyhow::Result;
use csv;
use reqwest;
use std::collections::HashMap;

use super::ProcessingStep;

pub struct MetadataJoiner {
    table: HashMap<String, VehicleMetadata>,
}

impl MetadataJoiner {
    pub async fn init() -> Self {
        Self {
            table: download_table().await.unwrap(),
        }
    }
}

impl ProcessingStep for MetadataJoiner {
    fn apply(&mut self, vehicle: &mut Vehicle, _low_watermark: u64) -> bool {
        if let Some(ref trip_id) = vehicle.trip_id {
            vehicle.metadata = self.table.get(trip_id).map(|x| x.to_owned());
            true
        } else {
            false
        }
    }
}

async fn download_table() -> Result<HashMap<String, VehicleMetadata>> {
    let text =
        reqwest::get("https://storage.googleapis.com/gtfs_static/sweden_aggregated_metadata.csv")
            .await?
            .text()
            .await?;
    let mut table = HashMap::<String, VehicleMetadata>::default();
    for result in csv::Reader::from_reader(text.as_bytes()).deserialize() {
        let item: VehicleMetadata = result?;
        table.insert(item.trip_id.clone(), item);
    }
    println!("Downloaded metadata table");
    Ok(table)
}
