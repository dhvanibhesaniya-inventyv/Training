use crate::thread::task_manager::HASH_DATA;
use chrono::Utc;
use std::collections::{HashMap, VecDeque};

pub fn biforcation_data_level() {
    let mut hash_temp = HashMap::new();
    let current_time = Utc::now();

    for (each_key, each_queue) in HASH_DATA.write().unwrap().iter_mut() {
        let mut new_queue_name: Option<String> = None;
        let mut list_req = VecDeque::new();

        while let Some(mut each_req) = each_queue.pop_front() {
            let time_second_diff = current_time
                .signed_duration_since(each_req.timestamp)
                .num_seconds();
            if time_second_diff > 30 {
                each_req.timestamp = Utc::now();
                let queue_name: Vec<&str> = each_key.split("_").collect();
                let old_level = *queue_name.last().unwrap();
                let new_level = match old_level {
                    "L1" => "L2",
                    "L2" => "L3",
                    "L3" => "L4",
                    "L4" => "L5",
                    _ => old_level,
                };
                let mut temp_name = queue_name.join("_");
                temp_name = temp_name.replace(old_level, new_level);
                // println!("Old level: {} and new level: {}", old_level, new_level);
                new_queue_name = Some(temp_name);

                list_req.push_back(each_req);
            }
        }

        if let Some(new_key) = new_queue_name {
            hash_temp.insert(new_key, list_req);
        }
    }

    for (each_key, mut each_req) in hash_temp {
        HASH_DATA
            .write()
            .unwrap()
            .entry(each_key)
            .and_modify(|queue| {
                queue.append(&mut each_req);
            });
    }
}
