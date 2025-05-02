use axum::{routing::post, Json, Router};
use serde::Deserialize;
use serde_json::{json, Value};

use crate::{Error, Result};

pub fn route_api_login() -> Router {
    Router::new().route("/api/login", post(api_login))
}

//  check post data to login
async fn api_login(payload: Json<LoginPayload>) -> Result<Json<Value>> {
    println!("->> {:<8} - api_login", "HANDLER");

    //  TODO!(implement db login)
    if payload.user != "demo" || payload.pass != "run" {
        return Err(Error::LoginFail);
    }

    let body = Json(json!({
        "result": {
            "success": true
        }
    }));

    Ok(body)
}

#[derive(Debug, Deserialize)]
struct LoginPayload {
    user: String,
    pass: String,
}
