// use crate::common_struct::{Available, RequestData};
use crate::thread::task_manager::{HASH_DATA, REF2};
use std::collections::VecDeque;

pub fn biforcation_data() {
    if let Some(request_data) = REF2.write().expect("unavailable").pop_front() {
        let key = format!(
            "{}_{}_{}_L1",
            request_data.skill, request_data.language, request_data.status,
        );

        // Update the hashmap with an empty VecDeque if the key doesn't exist
        HASH_DATA
            .write()
            .unwrap()
            .entry(key)
            .or_insert_with(VecDeque::new)
            .push_back(request_data);

        // println!("Hashmap: {:#?}",HASH_DATA.read().expect());
    }

    
    
    // let hash_data_read = HASH_DATA.read().expect("unavailable");

    // for (key, value) in hash_data_read.iter() {
    //     println!("key: {:#?}, value: {:#?}", key, value);
    // }


    println!("Biforcation done .......")
}
