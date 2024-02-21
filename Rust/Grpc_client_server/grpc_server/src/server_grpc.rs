use tonic::transport::Server;

use crate::Users::student::service::{services::student_data_server::StudentDataServer, StudentServices};




pub mod services {
    tonic::include_proto!("services");
}

pub mod Users;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "127.0.0.1:50051".parse().unwrap();
    let std_service = StudentServices::default();

    // server is running on
    println!("Server running on: {}", addr);

    Server::builder()
        .add_service(StudentDataServer::new(std_service))
        .serve(addr)
        .await.unwrap();

    Ok(())
}

// cargo run --bin server