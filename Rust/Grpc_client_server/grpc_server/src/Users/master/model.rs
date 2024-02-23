use std::{fs, sync::{Arc, RwLock}};

use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct MasterData {
    pub id: u32,
    pub name: String,
    pub skills: Vec<String>,
    pub status: String,
    pub language: String,
}

lazy_static! {

    #[derive(Debug)]
    pub static ref ALL_MASTER: Arc<RwLock<Vec<MasterData>>> = {
        let data = fs::read_to_string("utils/Axum_server_json/Master_Data.json")
            .expect("Failed to read the JSON file");

        Arc::new(RwLock::new(serde_json::from_str(&data).expect("Failed to deserialize JSON")))
    };
}