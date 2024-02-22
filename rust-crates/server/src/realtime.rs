use axum::response::{Sse, sse::{Event, KeepAlive}};
use futures_util::{Stream, StreamExt};
use tokio_stream::wrappers::BroadcastStream;
use types::redis_util::subscribe;

use crate::stats::inc_site_visits;

pub async fn realtime_handler() -> Sse<impl Stream<Item = anyhow::Result<Event>>> {
    inc_site_visits().await;
    let receiver = subscribe("realtime-with-metadata").await;
    let stream = BroadcastStream::new(receiver)
        .map(|x| { x.map(|s| Event::default().data(s)) })
        .map(|x| { x.map_err(|e| e.into()) });
    Sse::new(stream).keep_alive(KeepAlive::default())
}
