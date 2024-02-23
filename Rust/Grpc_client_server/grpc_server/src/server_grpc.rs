use tonic::transport::Server;

use crate::Users::student::service::{services::student_data_server::StudentDataServer, StudentServices};
use crate::Users::employee::service::{services::employee_data_server::EmployeeDataServer ,EmployeeServices};
use crate::Users::master::service::{services::master_data_server::MasterDataServer, MasterServices};


pub mod services {
    tonic::include_proto!("services");
}

pub mod Users;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "127.0.0.1:50051".parse().unwrap();
    let std_service = StudentServices::default();
    let emp_services = EmployeeServices::default();
    let mst_services = MasterServices::default();

    // server is running on
    println!("Server running on: {}", addr);

    Server::builder()
        .add_service(StudentDataServer::new(std_service))
        .add_service(EmployeeDataServer::new(emp_services))
        .add_service(MasterDataServer::new(mst_services))
        .serve(addr)
        .await.unwrap();

    Ok(())
}

// cargo run --bin server