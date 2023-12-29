use crate::{Vehicle, VehicleMetadata, Stop};
use anyhow::Result;
use csv;
use flate2::read::GzDecoder;
use reqwest;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, io::Read};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct IntermediateVehicleMetadata {
    pub trip_id: String,
    pub route_id: Option<String>,
    pub route_type: Option<u64>,
    pub agency_name: Option<String>,
    pub route_short_name: Option<String>,
    pub route_long_name: Option<String>,
    pub trip_headsign: Option<String>,
    pub shape_id: Option<u64>,
    pub direction_id: Option<u32>,
    pub stop_id: Option<String>,
    pub stop_name: Option<String>,
    pub stop_sequence: Option<String>,
    pub stop_lat: Option<String>,
    pub stop_lon: Option<String>,
    pub arrival_time: Option<String>,
    pub departure_time: Option<String>,
    pub shape_dist_traveled: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DeserializedVehicleMetadata {
    pub trip_id: String,
    pub route_id: Option<String>,
    pub route_type: Option<u64>,
    pub agency_name: Option<String>,
    pub route_short_name: Option<String>,
    pub route_long_name: Option<String>,
    pub trip_headsign: Option<String>,
    pub shape_id: Option<u64>,
    pub direction_id: Option<u32>,
    pub stop_id: Option<Vec<u64>>,
    pub stop_name: Option<Vec<String>>,
    pub stop_sequence: Option<Vec<u8>>,
    pub stop_lat: Option<Vec<f64>>,
    pub stop_lon: Option<Vec<f64>>,
    pub arrival_time: Option<Vec<String>>,
    pub departure_time: Option<Vec<String>>,
    pub shape_dist_traveled: Option<Vec<f64>>,
}

pub struct MetadataJoiner {
    table: HashMap<String, VehicleMetadata>,
}

impl MetadataJoiner {
    pub async fn init() -> Self {
        Self {
            table: download_table().await.unwrap(),
        }
    }

    pub fn join_metadata(&self, vehicle: &mut Vehicle) {
        if let Some(ref trip_id) = vehicle.trip_id {
            vehicle.metadata = self.table.get(trip_id).map(|x| x.to_owned());
        }
    }
}

fn deserialize_stops(raw_item: IntermediateVehicleMetadata) -> VehicleMetadata {
    let stop_ids: Vec<u64> = if let Some(series) = raw_item.stop_id {
        series.split("|").map(|x| x.parse::<u64>().unwrap()).collect()
    } else { Vec::default() };

    let stop_names: Vec<String> = if let Some(series) = raw_item.stop_name {
        series.split("|").map(|x| x.to_owned()).collect()
    } else { Vec::default() };
    
    let stop_lats: Vec<f64> = if let Some(series) = raw_item.stop_lat {
        series.split("|").map(|x| x.parse::<f64>().unwrap()).collect()
    } else { Vec::default() };
    
    let stop_lons: Vec<f64> = if let Some(series) = raw_item.stop_lon {
        series.split("|").map(|x| x.parse::<f64>().unwrap()).collect()
    } else { Vec::default() }; 
    
    let stop_sequences: Vec<u8> = if let Some(series) = raw_item.stop_sequence {
        series.split("|").map(|x| x.parse::<u8>().unwrap()).collect()
    } else { Vec::default() };
    
    let arrival_times: Vec<Option<String>> = if let Some(series) = raw_item.arrival_time {
        series.split("|").map(|x| if x == "nan" { None } else { Some (x.to_owned()) }).collect()
    } else { Vec::default() };
    
    let departure_times: Vec<Option<String>> = if let Some(series) = raw_item.departure_time {
        series.split("|").map(|x| if x == "nan" { None } else { Some (x.to_owned()) }).collect()
    } else { Vec::default() };
    
    let shape_dist_traveleds: Vec<f64> = if let Some(series) = raw_item.shape_dist_traveled {
        series.split("|").map(|x| x.parse::<f64>().unwrap()).collect()
    } else { Vec::default() };

    let mut stops: Vec<Stop> = Vec::default();
    let mut i = 0;
    while i < stop_ids.len() {
       stops.insert(i, Stop {
        stop_id: stop_ids[i],
        stop_name: stop_names[i].to_owned(),
        stop_lat: stop_lats[i],
        stop_lon: stop_lons[i],
        stop_sequence: stop_sequences[i],
        arrival_time: arrival_times[i].to_owned(),
        departure_time: departure_times[i].to_owned(),
        shape_dist_traveled: shape_dist_traveleds[i],
       });
       i = i + 1;
    };

    VehicleMetadata {
        trip_id: raw_item.trip_id,
        shape_id: raw_item.shape_id,
        direction_id: raw_item.direction_id,
        route_id: raw_item.route_id,
        route_short_name: raw_item.route_short_name,
        route_long_name: raw_item.route_long_name,
        route_type: raw_item.route_type,
        agency_name: raw_item.agency_name,
        trip_headsign: raw_item.trip_headsign,
        stops: if stop_ids.len() > 0 { Some(stops) } else { None } 
    }
}

async fn download_table() -> Result<HashMap<String, VehicleMetadata>> {
    let body =
        reqwest::get("https://storage.googleapis.com/gtfs_static/sweden_aggregated_metadata.csv.gz")
            .await?
            .bytes()
            .await?
            .to_vec();

    // Explisitly decompress the gziped body because there is no content-encoding: gzip header on the response
    let mut buffer: Vec<u8> = Vec::default();
    let mut decoder = GzDecoder::new(body.as_slice());
    // Keep it as bytes since the csv reader deserializes from bytes anyway
    decoder.read_to_end(&mut buffer)?;

    let mut table = HashMap::<String, VehicleMetadata>::default();
    for result in csv::Reader::from_reader(buffer.as_slice()).deserialize() {
        let raw_item: IntermediateVehicleMetadata = result?;
        // println!("{:?}", raw_item);
        let item = deserialize_stops(raw_item);
        // print!("{:?}", item);
        table.insert(item.trip_id.clone(), item);
    }
    println!("Downloaded metadata table");
    Ok(table)
}
