use lazy_static::lazy_static;
use std::{fs, sync::{Arc, RwLock}};

pub mod student_model;
pub mod employee_model;
pub mod master_model;
use axum::{routing::{get, post}, Router};
use crate::common_struct::{AStudent,AEmployee,MasterData,Message};

use self::student_model::{get_all_Students,get_student_by_id,delete_student_by_id,update_student_by_id,create_student};  
use self::employee_model::{get_all_employee,get_employee_by_id,delete_employee_by_id,update_employee,create_employee};
use self::master_model::{get_all_master,get_master_by_id,delete_master_by_id,update_master,create_master};



lazy_static! {

    #[derive(Debug)]
    pub static ref ALL_STUDENT: Arc<RwLock<Vec<AStudent>>> = {
        let data = fs::read_to_string("json_data/Axum_server_json/StudentData.json")
            .expect("Failed to read the JSON file");

        Arc::new(RwLock::new(serde_json::from_str(&data).expect("Failed to deserialize JSON")))
    };

    


    #[derive(Debug)]
    pub static  ref  ALL_EMPLOYEE: Arc<RwLock<Vec<AEmployee>>> = {
        let data = fs::read_to_string("json_data/Axum_server_json/Employee.json")
            .expect("Failed to read the JSON file");

        Arc::new(RwLock::new(serde_json::from_str(&data).expect("Failed to deserialize JSON")))
    };



    #[derive(Debug)]
    pub static ref ALL_MASTER: Arc<RwLock<Vec<MasterData>>> = {
        let data = fs::read_to_string("json_data/Axum_server_json/Master_Data.json")
            .expect("Failed to read the JSON file");

        Arc::new(RwLock::new(serde_json::from_str(&data).expect("Failed to deserialize JSON")))
    };

}


// get student Routes .

pub fn get_student_route()-> Router{
    let routees = Router::new().route("/student_get_all", post(get_all_Students))
    .route("/student/:id", post(get_student_by_id))
    .route("/student_delete/:id", post(delete_student_by_id))
    .route("/student_update/:id/:name/:phone/:email/:city/:address",post( update_student_by_id))
    .route("/create_student", post(create_student));
    routees
}


// get employee Routes .

pub fn get_employee_route()-> Router  {

let routees = Router::new().route("/employee_get_all", post(get_all_employee))
.route("/employee/:id", post(get_employee_by_id))
.route("/employee_delete/:id", post(delete_employee_by_id))
    .route("/employee_update",post( update_employee))
    .route("/employee_create", post(create_employee));
    routees

}



// get master Routes .
pub fn get_master_route() -> Router{
    let routees = Router::new().route("/master_get_all", post(get_all_master))
    .route("/master/:id", post(get_master_by_id))
    .route("/master_delete/:id", post(delete_master_by_id))
    .route("/master_update",post( update_master))
    .route("/master_create", post(create_master));
    routees

}