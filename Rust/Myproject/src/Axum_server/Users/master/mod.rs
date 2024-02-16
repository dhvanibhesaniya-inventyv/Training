use axum::{routing::post, Router};
pub mod services;
use services::{create_master, delete_master_by_id, get_all_master, get_master_by_id, update_master};

// get master Routes .
pub fn get_master_route() -> Router{
    let routees = Router::new().route("/master_get_all", post(get_all_master))
    .route("/master/:id", post(get_master_by_id))
    .route("/master_delete/:id", post(delete_master_by_id))
    .route("/master_update",post( update_master))
    .route("/master_create", post(create_master));
    routees

}