use crate::users::student::model::student_data_loading;




pub mod users;
pub mod utils;
pub mod configration;
use crate::utils::db_config::get_client;
use crate::utils::logger;





#[tokio::main]
async fn main() {
  
  get_client().await;
      // Initialize Logger
      logger::startLogger();

student_data_loading().await;

    println!("running on http://127.0.0.1:5000");
    let app = users::student::api();
    axum::Server::bind(&"127.0.0.1:5000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
