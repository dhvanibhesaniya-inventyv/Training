use crate::thread::task_manager::{HASH_DATA, USER_DATA};

pub fn task_assigner_from_hash_main() {
    // loop {
    //     thread::sleep(Duration::from_secs(5)); // Adjust the interval as needed

    let user_data = USER_DATA.read().unwrap();

    let mut hash_data = HASH_DATA.write().unwrap();
    for level in (1..=5).rev() {
        let level_key = format!("_L{}", level);
        for (key, data) in hash_data.iter_mut() {
            if key.ends_with(&level_key) {
                let mut matched_indexes = Vec::new();
                for (index, request_data) in data.iter().enumerate() {
                    let mut matched = false;
                    for user in user_data.iter() {
                        if user.skills.contains(&request_data.skill)
                            && user.language == request_data.language
                            && ((key.contains("Call") && user.status == "Offline")
                                || (key.contains("Chat") && user.status == "Online"))
                        {
                            println!("Matched data for user ID {}: {:?}", user.id, request_data);
                            matched = true;
                            break;
                        }
                    }
                    if matched {
                        matched_indexes.push(index);
                    }
                }

                matched_indexes.reverse();
                for index in matched_indexes {
                    data.remove(index);
                }
            }
        }
    }
    // }
}
