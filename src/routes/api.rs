use axum::Json;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct DataIn {
    message: String,
}

pub async fn post_api(Json(body): Json<DataIn>) -> Json<DataIn> {
    Json(body)
}
