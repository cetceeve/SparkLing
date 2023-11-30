use chrono::prelude::*;
use csv::Writer;
use flate2::write::GzEncoder;
use flate2::Compression;
use redis::{self, PubSubCommands};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::sync::Arc;
use tokio::sync::mpsc::{self, UnboundedReceiver, UnboundedSender};
use tokio::sync::Mutex;
use tokio::time::{self, Duration, Instant};
use google_cloud_storage::client::{ClientConfig, Client};
use google_cloud_storage::http::objects::upload::{Media, UploadObjectRequest, UploadType};


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

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CsvSerializableVehicle {
    pub id: String,
    pub lng: f32,
    pub lat: f32,
    pub timestamp: u64,
    pub trip_id: Option<String>,
    pub route_type: Option<u64>,
    pub agency_name: Option<String>,
    pub route_short_name: Option<String>,
    pub route_long_name: Option<String>,
    pub trip_headsign: Option<String>,
}

#[tokio::main]
async fn main() {
    // oneshot channel to get data to the writer thread
    let (sender, receiver) = mpsc::unbounded_channel::<String>();
    tokio::task::spawn(async move { redis_subscriber(sender).await });

    let guarded_recv = Arc::new(Mutex::new(receiver));
    loop {
        let date = Utc::now().format("%Y-%m-%dT%H_%M_%S").to_string();
        let file_name = format!("{date}-rt-data.csv.gz");
        match collect_as_csv_rows(guarded_recv.clone(), 1000000).await {
            Err(_) => {}
            Ok(data) => {
                println!("Finished collecting rows for: {}", file_name);

                // spawning an async task to upload the csv data to google cloud storage
                tokio::task::spawn( async move {
                    // authenticate using Application Default Credentials
                    let config = ClientConfig::default().with_auth().await.unwrap();
                    let client = Client::new(config);

                    println!("Uploading to Google Cloud Storage...");
                    let upload_type = UploadType::Simple(Media::new(file_name));
                    let uploaded = client.upload_object(&UploadObjectRequest {
                        bucket: "test-rt-data-archive".to_string(),
                        ..Default::default()
                    }, data, &upload_type).await;

                    
                    match uploaded {
                        Ok(storage_object) => {
                            println!("Upload successfull!");
                            println!("{:?}", storage_object)
                        },
                        Err(err) => {
                            println!("Upload failed.");
                            println!("{:?}", err);
                        }
                    };
                });
        },
        }
    }
}

async fn collect_as_csv_rows(
    guarded_receiver: Arc<Mutex<UnboundedReceiver<String>>>,
    min_rows: u64,
) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut receiver = guarded_receiver.lock().await;
    // using tokio sleep with select to create a timeout function
    const TIMEOUT: u64 = 200;
    let sleep = time::sleep(Duration::from_millis(TIMEOUT));
    tokio::pin!(sleep); 

    let encoder = GzEncoder::new(Vec::new(), Compression::default());
    let mut wtr = Writer::from_writer(encoder);

    let mut rows_written = 0;
    loop {
        tokio::select! {
            Some(msg) = receiver.recv() => {
                if let Ok(vehicle) = serde_json::from_str::<Vehicle>(&msg) {
                    let csv_vehicle = make_vehicle_serializable(vehicle);

                    match wtr.serialize(csv_vehicle) {
                        Err(_) => println!("CSV Writer: Error serializing vehicle to csv."),
                        Ok(()) => {
                            sleep.as_mut().reset(Instant::now() + Duration::from_millis(TIMEOUT));
                            rows_written += 1;
                        },
                    };
                } else {
                    println!("Serde: Error deserializing vehicle.")
                }
            },
            // called only when receiver has not received anything for a while
            () = &mut sleep => {
                sleep.as_mut().reset(Instant::now() + Duration::from_millis(2000));
                println!("Currently {}rows.", rows_written);

                if rows_written > min_rows {
                    // returning the underlying writers according to documentation for csv and flate2
                    // automatically flushes to the underlying writers
                    return Ok(wtr.into_inner()?.finish()?);
                }
            },
        }
    }
}

async fn redis_subscriber(sender: UnboundedSender<String>) {
    // start redis client
    let redis_client = redis::Client::open("redis://sparkling-redis/").unwrap();
    let mut redis_conn = redis_client.get_connection().unwrap();

    let _: () = redis_conn
        .subscribe(&["realtime-with-metadata"], |msg| {
            match msg.get_payload() {
                Ok(data) => sender.send(data).expect("Internal channel broken"),
                Err(_) => println!("Redis: Error retrieving payload from pubsub."),
            }
            redis::ControlFlow::Continue
        })
        .unwrap();
}

fn make_vehicle_serializable(vehicle: Vehicle) -> CsvSerializableVehicle {
    match vehicle.metadata {
        Some(metadata) => CsvSerializableVehicle {
            id: vehicle.id,
            lng: vehicle.lng,
            lat: vehicle.lat,
            timestamp: vehicle.timestamp,
            trip_id: vehicle.trip_id,
            route_type: metadata.route_type,
            agency_name: metadata.agency_name,
            route_short_name: metadata.route_short_name,
            route_long_name: metadata.route_long_name,
            trip_headsign: metadata.trip_headsign,
        },
        None => CsvSerializableVehicle {
            id: vehicle.id,
            lng: vehicle.lng,
            lat: vehicle.lat,
            timestamp: vehicle.timestamp,
            trip_id: vehicle.trip_id,
            route_type: None,
            agency_name: None,
            route_short_name: None,
            route_long_name: None,
            trip_headsign: None,
        },
    }
}
