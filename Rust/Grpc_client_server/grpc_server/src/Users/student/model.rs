use std::{fs, sync::{Arc, RwLock}};

use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct AStudent {
    pub id: u32,
    pub  name: String,
    pub phone: String,
    pub email: String,
    pub city: String,
    pub address: String,
    pub marks: Vec<u32>,
}


lazy_static! {

    #[derive(Debug)]
    pub static ref ALL_STUDENT: Arc<RwLock<Vec<AStudent>>> = {
        let data = fs::read_to_string("utils/Axum_server_json/StudentData.json")
            .expect("Failed to read the JSON file");

        Arc::new(RwLock::new(serde_json::from_str(&data).expect("Failed to deserialize JSON")))
    };
}