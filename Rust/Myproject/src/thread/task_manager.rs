use lazy_static::lazy_static;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::{
    collections::VecDeque,
    fmt, fs,
    sync::{Arc, RwLock},
    thread,
    time::Duration,
};

use crate::common_struct::{Available, Individual, RequestData, Skill, Status};

use super::task_manager_services::{
    biforcation_services::biforcation_data, generate_random_data::random_data,
};

lazy_static! {
    pub static ref USER_DATA: VecDeque<Individual> = {
        let data = fs::read_to_string("json_data/task_manager_json/Master_Data.json")
            .expect("Failed to read the JSON file");

        serde_json::from_str(&data).expect("Failed to deserialize JSON")
    };
    pub static ref PENDING_DATA: Arc<RwLock<Vec<RequestData>>> = Arc::new(RwLock::new(Vec::new()));
    pub static ref REF1: Arc<RwLock<Vec<RequestData>>> = Arc::clone(&PENDING_DATA);
    pub static ref REF2: Arc<RwLock<Vec<RequestData>>> = Arc::clone(&PENDING_DATA);
    pub static ref REF3: Arc<RwLock<Vec<RequestData>>> = Arc::clone(&PENDING_DATA);
    pub static ref CHAT_VECTOR: Arc<RwLock<VecDeque<RequestData>>> =
        Arc::new(RwLock::new(VecDeque::new()));
    pub static ref CHAT_REF1: Arc<RwLock<VecDeque<RequestData>>> = Arc::clone(&CHAT_VECTOR);
    pub static ref CALL_VECTOR: Arc<RwLock<VecDeque<RequestData>>> =
        Arc::new(RwLock::new(VecDeque::new()));
    pub static ref CALL_REF1: Arc<RwLock<VecDeque<RequestData>>> = Arc::clone(&CALL_VECTOR);
}

pub fn task_manager_main() {

    let random_data_thread: thread::JoinHandle<_> = thread::spawn(move || {
        loop {
            thread::sleep(Duration::from_secs(10));
    random_data();
        
        }
    }
);

let process_data_thread = thread::spawn(move || loop {
    thread::sleep(Duration::from_secs(1));

    biforcation_data();

});

    let match_data_thread = thread::spawn(move || loop {
        thread::sleep(Duration::from_secs(1));
        // let data3 = REF3.read().unwrap();

        match_data(&USER_DATA, &CALL_REF1.write().unwrap(), "Call");
        CALL_REF1.write().unwrap().pop_front();

        match_data(&USER_DATA, &CHAT_REF1.write().unwrap(), "Chat");
        CHAT_REF1.write().unwrap().pop_front();
    });

    random_data_thread.join().unwrap();
    process_data_thread.join().unwrap();
    match_data_thread.join().unwrap();
}

fn match_data(user_data: &VecDeque<Individual>, vector: &VecDeque<RequestData>, status: &str) {
    for request_data in vector.iter() {
        for user in user_data.iter() {
            if user.skills.contains(&request_data.skill)
                && user.language == request_data.language.to_string()
            {
                match status {
                    "Chat" => {
                        if user.status == "Online" {
                            println!("Matched data for user ID {}: {:?}", user.id, request_data);
                        }
                    }
                    "Call" => {
                        if user.status == "Offline" {
                            println!("Matched data for user ID {}: {:?}", user.id, request_data);
                        }
                    }
                    _ => {
                        print!("status not found");
                    }
                }
            }
        }
    }
}
