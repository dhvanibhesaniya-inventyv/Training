use std::net::SocketAddr;

use axum::{routing::get, Router};


use crate::Axum_server::Routes::get_routes;
pub mod Users;
pub mod Routes;
pub mod health_check;

#[tokio::main]
pub async fn main() {

let app = get_routes();

    println!("Running on http://localhost:3000");
    axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}