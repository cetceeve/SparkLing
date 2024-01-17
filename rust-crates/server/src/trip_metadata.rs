use axum::{extract::Path, Json, http::StatusCode};
use types::{VehicleMetadata, get_trip_metadata_async};

pub async fn metadata_handler(Path(trip_id): Path<String>) -> Result<Json<VehicleMetadata>, StatusCode> {
    if let Some(metadata) = get_trip_metadata_async(&trip_id).await {
        Ok(Json(metadata))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}
