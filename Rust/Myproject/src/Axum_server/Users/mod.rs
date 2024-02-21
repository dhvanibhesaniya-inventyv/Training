use lazy_static::lazy_static;
use std::{fs, sync::{Arc, RwLock}};


pub mod student;
pub mod employee;
pub mod master;

use crate::common_struct::Message;

// lazy_static! {

//     #[derive(Debug)]
//     pub static ref ALL_STUDENT: Arc<RwLock<Vec<AStudent>>> = {
//         let data = fs::read_to_string("json_data/Axum_server_json/StudentData.json")
//             .expect("Failed to read the JSON file");

//         Arc::new(RwLock::new(serde_json::from_str(&data).expect("Failed to deserialize JSON")))
//     };

    


//     #[derive(Debug)]
//     pub static  ref  ALL_EMPLOYEE: Arc<RwLock<Vec<AEmployee>>> = {
//         let data = fs::read_to_string("json_data/Axum_server_json/Employee.json")
//             .expect("Failed to read the JSON file");

//         Arc::new(RwLock::new(serde_json::from_str(&data).expect("Failed to deserialize JSON")))
//     };



//     #[derive(Debug)]
//     pub static ref ALL_MASTER: Arc<RwLock<Vec<MasterData>>> = {
//         let data = fs::read_to_string("json_data/Axum_server_json/Master_Data.json")
//             .expect("Failed to read the JSON file");

//         Arc::new(RwLock::new(serde_json::from_str(&data).expect("Failed to deserialize JSON")))
//     };

// }


