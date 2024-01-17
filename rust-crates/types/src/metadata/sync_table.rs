use std::{collections::HashMap, sync::{RwLock, Mutex}, time::{SystemTime, Duration}};

use super::{VehicleMetadata, download_metadata_table_blocking};

static mut TABLE: Option<RwLock<HashMap<String, VehicleMetadata>>> = None;
static mut LAST_UPDATED: Option<Mutex<SystemTime>> = None;

fn ensure_table() {
    unsafe {
        if let None = TABLE {
            LAST_UPDATED = Some(Mutex::new(SystemTime::now()));
            TABLE = Some(RwLock::new(Default::default()));
            if let Some(ref table) = TABLE {
                let mut lock = table.write().unwrap();
                *lock = download_metadata_table_blocking().unwrap();
            }
        } else {
            if let Some(ref last_updated) = LAST_UPDATED {
                let mut ts = last_updated.lock().unwrap();
                if ts.elapsed().unwrap() > Duration::from_secs(24*60*60) {
                    *ts = SystemTime::now();
                    if let Some(ref table) = TABLE {
                        *table.write().unwrap() = download_metadata_table_blocking().unwrap();
                    }
                }
            }
        }
    }
}

pub fn init_blocking_metadata_table() {
    ensure_table();
}

pub fn get_trip_metadata_blocking(trip_id: &str) -> Option<VehicleMetadata> {
    ensure_table();
    unsafe {
        if let Some(ref table) = TABLE {
            table.read().unwrap().get(trip_id).cloned()
        } else {
            None
        }
    }
}
