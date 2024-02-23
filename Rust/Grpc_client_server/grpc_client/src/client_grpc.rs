
use client_side_calling::student_calling::{getallstudent,getstudentbyid,updatestudentbyid,deletestudentbyid,updaterealjsonfile};
use client_side_calling::employee_calling::{getallemployee,getemployeebyid,updateemployeebyid,deleteemployeebyid,updateerealjsonfile};
use client_side_calling::master_calling::{ getallMaster,getMasterbyid,updateMasterbyid,deleteMasterbyid,createnewMaster,updatemrealjsonfile};

pub mod client_side_calling;

pub mod services {
    tonic::include_proto!("services");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    // getallstudent().await;
    // getstudentbyid().await;
    // updatestudentbyid().await;
    // deletestudentbyid().await;
    // updaterealjsonfile().await;
 
    // getallemployee().await;
    // getemployeebyid().await;
    // updateemployeebyid().await;
    // deleteemployeebyid().await;
    // updateerealjsonfile().await;

    getallMaster().await;
    // getMasterbyid().await;
    // updateMasterbyid().await;
    // deleteMasterbyid().await;
    // createnewMaster().await;
    // updatemrealjsonfile().await;


    Ok(())
  
}

// cargo run --bin client