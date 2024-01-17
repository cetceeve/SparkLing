use std::collections::HashMap;

use futures_util::StreamExt;
use tokio::sync::{broadcast::{Sender, Receiver, self}, Mutex};
use redis;

static mut REDIS_BROADCASTS: Option<Mutex<HashMap<String, Sender<String>>>> = None;

async fn ensure_table() {
    unsafe {
        if let None = REDIS_BROADCASTS {
            REDIS_BROADCASTS = Some(Mutex::new(HashMap::default()));
        }
    }
}

pub async fn subscribe(topic: &str) -> Receiver<String> {
    ensure_table().await;
    unsafe {
        if let Some(ref broadcasts) = REDIS_BROADCASTS {
            let mut locked_broadcasts = broadcasts.lock().await;
            if let Some(broadcast) = locked_broadcasts.get(topic) {
                // we have an active subscriber for this topic
                return broadcast.subscribe();
            } else {
                // create a new subscriber for this topic
                let (sender, receiver) = broadcast::channel::<String>(5000);
                locked_broadcasts.insert(topic.to_string(), sender.clone());
                let topic = topic.to_string();
                tokio::task::spawn(async move { run_subscriber(&topic, sender).await });
                return receiver;
            }
        }
        unreachable!("shared broadcast table should be there");
    }
}

async fn run_subscriber(topic: &str, sender: Sender<String>) {
    // start redis client
    let redis_client = redis::Client::open("redis://sparkling-redis/").unwrap();
    let redis_conn = redis_client.get_async_connection().await.unwrap();

    let mut pubsub = redis_conn.into_pubsub();
    pubsub.subscribe(&[topic]).await.unwrap();
    let mut stream = pubsub.on_message();
    while let Some(msg) = stream.next().await {
        match msg.get_payload() {
            Ok(data) => { let _ = sender.send(data); },
            Err(_) => println!("Redis: Error retrieving payload from pubsub."),
        }
    }
}
