
use serde::{Deserialize, Serialize};
use std::{fs, sync::{Arc, RwLock}};
use lazy_static::lazy_static;

#[derive(Debug, Serialize, Deserialize,Clone)]
pub struct Student {
    pub s_id: i32,
    pub name: String,
    pub phone: String,
    pub email: String,
    pub city: String,
    pub address: String,
    pub marks: Vec<u32>,
}


#[derive(Serialize , Deserialize)]
pub struct Message<T>{
    pub status: u32,
    pub message_key: String,
    pub data:T,
}

pub fn parse_json_data(filename:String) -> Vec<Student> {
    let json_data = fs::read_to_string(filename)
            .expect("Failed to read the JSON file");
        //deserialize
      let   students = serde_json::from_str(&json_data)
        .expect("Failed to parse JSON");

    students
}


pub fn allidstd()-> Vec<String>{

    let  all_id:Vec<String> = vec!["student_1".to_string(),"student_2".to_string(),"student_3".to_string(),"student_4".to_string(),"student_5".to_string()];
    println!("{:#?}",all_id);
    all_id
}

 

// lazy_static! {

//     #[derive(Debug)]
//     pub static ref ALL_STUDENT: Arc<RwLock<Vec<Student>>> = {
//         let data = fs::read_to_string("./Axum_server_json/StudentData.json")
//             .expect("Failed to read the JSON file");

//         Arc::new(RwLock::new(serde_json::from_str(&data).expect("Failed to deserialize JSON")))
//     };

// }
