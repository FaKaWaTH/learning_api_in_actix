use axum::{routing::post, Extension, Json, Router};
use serde::Deserialize;
use serde_json::{json, Value};
use tower_cookies::{Cookie, Cookies};

use crate::{web, Error, Result};

pub fn route_api_login() -> Router {
    Router::new().route("/api/login", post(api_login))
}

//  check post data to login

#[axum::debug_handler]
async fn api_login(
    Extension(cookies): Extension<Cookies>,
    Json(payload): Json<LoginPayload>,
) -> Result<Json<Value>> {
    println!("->> {:<8} - api_login", "HANDLER");

    //  TODO!(implement db login)
    if payload.user != "demo" || payload.pass != "run" {
        return Err(Error::LoginFail);
    }

    cookies.add(Cookie::new(web::AUTH_TOKE, "user-1.exp.sing"));

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
