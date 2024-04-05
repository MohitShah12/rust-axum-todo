mod api_data;
mod mirror_body_json;
mod mirror_body_string;
mod things_id;

use axum::{
    routing::{get, post},
    Router,
};
use mirror_body_json::mirror_body_json;
use mirror_body_string::mirror_body_string;
use things_id::things_id;

pub fn create_routes() -> Router {
    Router::new()
        .route("/", get(api_data::hello_world))
        .route("/mirror_body_string", post(mirror_body_string))
        .route("/mirror_body_json", post(mirror_body_json))
        .route("/things/:id", get(things_id))
}
