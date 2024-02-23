pub mod student;
pub mod employee;
pub mod master;


//     #[derive(Debug)]
//     pub static ref ALL_MASTER: Arc<RwLock<Vec<MasterData>>> = {
//         let data = fs::read_to_string("json_data/Axum_server_json/Master_Data.json")
//             .expect("Failed to read the JSON file");

//         Arc::new(RwLock::new(serde_json::from_str(&data).expect("Failed to deserialize JSON")))
//     };

// }
