mod api;
mod index;

use api::post_api;
use index::index;

//  set post or get for each route
pub fn create_route() -> axum::Router {
    axum::Router::new()
        .merge(route_static())
        .route("/api", axum::routing::post(post_api))
}

//  stack static files in one fn
fn route_static() -> axum::Router {
    axum::Router::new().route("/", axum::routing::get(index))
}
