pub(crate) use lazy_static::lazy_static;
use std::{
    collections::{HashMap, VecDeque},
    fs,
    sync::{Arc, RwLock},
    thread,
    time::Duration,
};

use crate::common_struct::{Available, Individual, Language, Level, RequestData};

use super::task_manager_services::{
    biforcation_level_service::biforcation_data_level, biforcation_services::biforcation_data,
    generate_random_data::random_data, task_assigner_hash::task_assigner_from_hash_main,
    update_master_data::update_user_data_randomly,
};

lazy_static! {
    #[derive(Debug)]
    pub static ref USER_DATA: Arc<RwLock<VecDeque<Individual>>> = {
        let data = fs::read_to_string("json_data/task_manager_json/Master_Data.json")
            .expect("Failed to read the JSON file");

        Arc::new(RwLock::new(serde_json::from_str(&data).expect("Failed to deserialize JSON")))
    };

    pub static ref PENDING_DATA: Arc<RwLock<VecDeque<RequestData>>> = Arc::new(RwLock::new(VecDeque::new()));
    pub static ref REF1: Arc<RwLock<VecDeque<RequestData>>> = Arc::clone(&PENDING_DATA);
    pub static ref REF2: Arc<RwLock<VecDeque<RequestData>>> = Arc::clone(&PENDING_DATA);
    pub static ref REF3: Arc<RwLock<VecDeque<RequestData>>> = Arc::clone(&PENDING_DATA);


    pub static ref SKILLS: Vec<String> = vec![
        "Customer Service".to_string(),
        "Problem-solving".to_string(),
        "Product Knowledge".to_string(),
        "Effective Communication".to_string(),
        "Time Management".to_string(),
        "Adaptability".to_string(),
        "Team Collaboration".to_string(),
        "Feedback Analysis".to_string(),
        "Proactive Engagement".to_string(),
        "Technical Proficiency".to_string(),
        "Cultural Sensitivity".to_string(),
        "Documentation".to_string(),
    ];

    pub static ref HASH_DATA: Arc<RwLock<HashMap<String, VecDeque<RequestData>>>> = {
        let mut hash_map: HashMap<String, VecDeque<RequestData>> = HashMap::new();

        for skill in SKILLS.iter() {
            for language in &[Language::English,Language::Spanish] {
                for available in &[Available::Chat, Available::Call] {
                    for level in &[Level::L1, Level::L2, Level::L3, Level::L4, Level::L5] {
                        let key = format!("{}_{:?}_{:?}_{:?}", skill, language, available, level);
                        hash_map.insert(key, VecDeque::new());
                    }
                }
            }
        }

        Arc::new(RwLock::new(hash_map))
    };
}


/// The main entry point for the task manager.
pub fn task_manager_main() {
    // Thread to generate random data
    let random_data_thread: thread::JoinHandle<_> = thread::spawn(move || loop {
        thread::sleep(Duration::from_secs(2));
        random_data();
    });

    // Thread to biforcate data from random Data
    let process_data_thread = thread::spawn(move || loop {
        thread::sleep(Duration::from_secs(5));

        biforcation_data();
    });

    // Thread to biforcate data in levels according to their timeSpan
    let process_data_thread_level = thread::spawn(move || loop {
        thread::sleep(Duration::from_secs(30));
        biforcation_data_level();
    });

    // Update the master data
    let master_data_updata = thread::spawn(move || loop {
        thread::sleep(Duration::from_secs(5));
        update_user_data_randomly();
    });

    // thread for assigning tasks from hash data
    let task_assigner_from_hash = thread::spawn(move || loop {
        thread::sleep(Duration::from_secs(5));
        task_assigner_from_hash_main();
    });


    random_data_thread.join().expect("failed to join thread");
    process_data_thread.join().expect("failed to join thread");
    master_data_updata.join().expect("failed to join thread");
    process_data_thread_level
        .join()
        .expect("failed to join thread");
    task_assigner_from_hash
        .join()
        .expect("failed to join thread");
    // match_data_thread.join().expect("failed to join thread");

    // for i in HASH_DATA.read().unwrap().iter(){
    // println!("key: {:?}, value: {:?}", i.0, i.1);
    // }
}
