use chrono::{DateTime, Utc};
use rand::Rng;
use std::sync::{Arc, RwLock};
use std::thread;
use std::time::Duration;

use crate::common_struct::PersonData;

fn generate_random_data() -> PersonData {
    // using rand crate we are generating a random number.
    let mut rng = rand::thread_rng();
    let id: u64 = rng.gen_range(1..=10);

    // this function call will generate a random name
    let name = generate_random_human_name();

    // this function will return current local tiume.
    let timestamp = get_current_timestamp();

    PersonData {
        id,
        name,
        timestamp,
    }
}

fn generate_random_human_name() -> String {
    let mut rd = rand::thread_rng();
    let cap_chaaracter = rd.gen_range('A'..='Z');
    let small_chaaracter = rd.gen_range('a'..='z');
    let random_length = rd.gen_range(1..=10);
    let mut name = String::new();
    for _i in 0..1 {
        name.push(cap_chaaracter);
        for _i in 0..=random_length {
            name.push(small_chaaracter);
        }
    }

    name
}

fn get_current_timestamp() -> DateTime<Utc> {
    // getting the local time using chrono crate
    let now: DateTime<Utc> = Utc::now();
    now
}

pub fn person_data_main() {
    // creating a rwlock on a new vector
    let v = RwLock::new(Vec::new());

    // creating a arc lock on the vector v
    let arc = Arc::new(v);

    let arc1 = Arc::clone(&arc);
    let arc2 = Arc::clone(&arc);
    let arc3 = Arc::clone(&arc);

    let t1 = thread::spawn(move || loop {
        thread::sleep(Duration::from_secs(5));
        let data = generate_random_data();

        println!("data entered : {:?} ", data);
        arc1.write().unwrap().push(data);
    });

    let t2 = thread::spawn(move || loop {
        thread::sleep(Duration::from_secs(30));
        println!("Data: {:?}", arc2.read().unwrap());
    });

    let t3 = thread::spawn(move || loop {
        thread::sleep(Duration::from_secs(60));
        // let popped_data = arc3.write().unwrap().pop();
        // println!("Popped Data: {:?}", popped_data);

        let mut data = arc3.write().unwrap();

        data.retain(|x| {
            let current_time = Utc::now();
            let duration = current_time.signed_duration_since(x.timestamp);
            duration.num_seconds() < 50
        });

        println!("Data removed.")
    });

    t1.join().unwrap();
    t2.join().unwrap();
    t3.join().unwrap();
}
