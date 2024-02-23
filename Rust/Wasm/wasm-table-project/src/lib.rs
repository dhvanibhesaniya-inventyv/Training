use wasm_bindgen::prelude::*;

use crate::table_task_hashmap::{table_task_hashmap_pdf,greeting};
pub mod table_task_hashmap;
#[wasm_bindgen]
pub fn gettingstring(a:String) -> String {
   greeting(a)
}


#[wasm_bindgen]
pub fn process_json(json_data:&str)-> String {
    // Process the JSON data as needed
    // println!("Received JSON data in Rust: {}", json_data);
    let table_hashmap =  table_task_hashmap_pdf(&json_data);
    table_hashmap

    // You can parse the JSON data, perform operations, etc.
}