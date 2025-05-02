mod index;

use axum::routing::{get, Router};
use index::index;

use crate::web;

//  set post or get for each route
pub fn create_route() -> Router {
    Router::new()
        .merge(route_static())
        .merge(web::routes_login::route_api_login())
}

//  stack static files in one fn
fn route_static() -> Router {
    Router::new().route("/", get(index))
}
