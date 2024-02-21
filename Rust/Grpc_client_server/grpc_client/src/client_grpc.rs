
use client_side_calling::student_calling::{getallstudent,getstudentbyid,updatestudentbyid,deletestudentbyid,updaterealjsonfile};

pub mod client_side_calling;

pub mod services {
    tonic::include_proto!("services");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    getallstudent().await;
    // getstudentbyid().await;
    // updatestudentbyid().await;
    // deletestudentbyid().await;
    // updaterealjsonfile().await;
 
    

    Ok(())
  
}

// cargo run --bin client