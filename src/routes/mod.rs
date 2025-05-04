mod index;
pub(crate) mod model;

use axum::{
    middleware,
    response::Response,
    routing::{get, Router},
};
use index::index;
use model::ModelController;
use tower_cookies::CookieManagerLayer;

use crate::web;

//  set post or get for each route
pub fn create_route(mc: ModelController) -> Router {
    Router::new()
        .merge(route_static())
        .merge(web::routes_login::route_api_login())
        //  /api/tickets -> (get post) /api/tickets/{id}} -> (delete)
        .nest("/api", web::routes_tickets::routes(mc.clone()))
        .layer(middleware::map_response(response_mapper))
        .layer(CookieManagerLayer::new())
}

//  stack static files in one fn
fn route_static() -> Router {
    Router::new().route("/", get(index))
}

async fn response_mapper(res: Response) -> Response {
    println!("->> {:<8} - response_mapper", "RES_MAPPER");

    println!();
    res
}
