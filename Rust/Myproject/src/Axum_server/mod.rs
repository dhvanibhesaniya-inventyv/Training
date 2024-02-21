
use tower::ServiceBuilder;

use crate::Axum_server::middleware::cors::get_cors_middleware;
use crate::Axum_server::Routes::get_routes;
use crate::Axum_server::middleware::get_middlewares;

pub mod Users;
pub mod Routes;
pub mod health_check;
pub mod middleware;
pub mod utils;

#[tokio::main]
pub async fn main() {

let app = get_routes();
let app =get_middlewares(app);
 let app =app.layer(ServiceBuilder::new().layer(get_cors_middleware())); 

    println!("Running on http://localhost:3000");
    axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}