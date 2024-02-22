use lazy_static::lazy_static;
use std::{sync::Arc, time::{SystemTime, Duration}};
use tokio::sync::Mutex;

lazy_static!{
    static ref SITE_VISITS: Arc<Mutex<Vec<SystemTime>>> = Default::default();
}

pub async fn inc_site_visits() {
    SITE_VISITS.lock().await.push(SystemTime::now());
}

pub async fn stats_handler() -> String {
    let visits = SITE_VISITS.lock().await;
    format!(
        "Site visits in the last hour: {}\nSite visits in the last 24h: {}\nSite visits since last system reboot: {}",
        visits.iter().filter(|t| t.elapsed().unwrap() < Duration::from_secs(60*60)).count(),
        visits.iter().filter(|t| t.elapsed().unwrap() < Duration::from_secs(60*60*24)).count(),
        visits.len(),
    )
}
