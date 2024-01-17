use std::{collections::HashMap, time::{SystemTime, Duration}};
use tokio::sync::{RwLock, Mutex};

use super::{VehicleMetadata, download_metadata_table_async};

static mut TABLE: Option<RwLock<HashMap<String, VehicleMetadata>>> = None;
static mut LAST_UPDATED: Option<Mutex<SystemTime>> = None;

async fn ensure_table() {
    unsafe {
        if let None = TABLE {
            LAST_UPDATED = Some(Mutex::new(SystemTime::now()));
            TABLE = Some(RwLock::new(Default::default()));
            if let Some(ref table) = TABLE {
                let mut lock = table.write().await;
                *lock = download_metadata_table_async().await.unwrap();
            }
        } else {
            if let Some(ref last_updated) = LAST_UPDATED {
                let mut ts = last_updated.lock().await;
                if ts.elapsed().unwrap() > Duration::from_secs(24*60*60) {
                    *ts = SystemTime::now();
                    if let Some(ref table) = TABLE {
                        *table.write().await = download_metadata_table_async().await.unwrap();
                    }
                }
            }
        }
    }
}

pub async fn init_async_metadata_table() {
    ensure_table().await;
}

pub async fn get_trip_metadata_async(trip_id: &str) -> Option<VehicleMetadata> {
    ensure_table().await;
    unsafe {
        if let Some(ref table) = TABLE {
            table.read().await.get(trip_id).cloned()
        } else {
            None
        }
    }
}
