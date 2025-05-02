mod api;
mod index;

use api::post_api;
use index::index;

//  set post or get for each route
pub fn create_route() -> axum::Router {
    axum::Router::new()
        .route("/", axum::routing::get(index))
        .route("/api", axum::routing::post(post_api))
}
