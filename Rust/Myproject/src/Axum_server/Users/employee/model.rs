use std::{fs, sync::{Arc, RwLock}};

use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct AEmployee {
   pub  id: u32,
   pub  name: String,
   pub  age: u32,
   pub  skills: Vec<String>,
   pub  position: Option<String>,
   #[serde(rename(serialize = "experience(y)", deserialize = "experience(y)"))]
   pub  experience: Option<u32>,
}
lazy_static! {

    #[derive(Debug)]
    pub static  ref  ALL_EMPLOYEE: Arc<RwLock<Vec<AEmployee>>> = {
        let data = fs::read_to_string("src/Axum_server/utils/Axum_server_json/Employee.json")
            .expect("Failed to read the JSON file");

        Arc::new(RwLock::new(serde_json::from_str(&data).expect("Failed to deserialize JSON")))
    };
}