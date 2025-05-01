mod index;

use index::index;

pub fn create_route() -> axum::Router {
    axum::Router::new().route("/", axum::routing::get(index))
}
