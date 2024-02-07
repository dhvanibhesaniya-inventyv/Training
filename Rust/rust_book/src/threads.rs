// use std::{thread, time::Duration};

// fn main(){
//     thread::spawn(|| {
//         for i in 1..10{
//             println!("spawn :: {}",i);
// //  thread::sleep(Duration::from_millis(1));
//         }
//     });

//     for i in 1..5{
//         println!("main :: {}",i);
// // thread::sleep(Duration::from_millis(1));
//     }
// }

//In Rust, "threads" generally refer to concurrent execution units.
//  Rust provides built-in support for multi-threading through the std::thread module.
// Threads in Rust are similar to threads in other programming languages and operating
// systems, allowing you to execute multiple pieces of code concurrently.

// -------------------------------------------------------------------

use std::rc::Rc;
use std::sync::{mpsc, Arc, Mutex, RwLock};
use std::thread;
use std::time::Duration;
#[derive(Debug)]

struct Data{
    id:i32
}

fn main() {

let v = RwLock::new(Vec::new());

let arc = Arc::new(v);

let arc1 = Arc::clone(&arc);
let arc2 = Arc::clone(&arc);

let t1 = thread::spawn(move ||  loop{
    thread::sleep(Duration::from_secs(2));

    println!("length of vector: {:?}",arc1.read().unwrap().len());
});

let t2 = thread::spawn(move || loop {
    thread::sleep(Duration::from_secs(1));

    arc2.write().unwrap().push(Data{id:1});
    // println!("length of vector: {:?}",);
});

t2.join().unwrap();
t1.join().unwrap();

}


// ------------------------------------------------------------------------





// // rand = "0.8.5"
// // chrono = "0.4.33"


// use chrono::format::strftime::StrftimeItems;
// use chrono::offset::Local;
// use chrono::DateTime;
// use rand::Rng;
// use std::rc::Rc;
// use std::sync::{mpsc, Arc, Mutex, RwLock};
// use std::thread;
// use std::time::{Duration, SystemTime};

// #[derive(Debug)]
// struct Dataee {
//     id: u64,
//     name: String,
//     timestamp: String,
// }

// fn generate_random_data() -> Dataee {
//     let mut rng = rand::thread_rng();
//     let id: u64 = rng.gen_range(1..=100);

//     let name = generate_random_human_name();

//     let timestamp = get_current_timestamp();

//     Dataee {
//         id,
//         name,
//         timestamp,
//     }
// }

// fn generate_random_human_name() -> String {
  
//     let names = vec![
//         "John Doe", "Jane Doe", "Alice", "Bob", "Charlie", "David", "Eva", "Frank", "Grace",
//         "Henry",
//     ];

//     let mut rng = rand::thread_rng();
//     let random_index = rng.gen_range(0..names.len());

//     names[random_index].to_string()
// }

// fn get_current_timestamp() -> String {
//     let now: DateTime<Local> = Local::now();
//     let format = StrftimeItems::new("%H:%M:%S");
//     now.format_with_items(format).to_string()
// }

// fn main() {
//     let v = RwLock::new(Vec::new());
//     let arc = Arc::new(v);

//     let arc1 = Arc::clone(&arc);
//     let arc2 = Arc::clone(&arc);
//     let arc3 = Arc::clone(&arc);

//     let t1 = thread::spawn(move || loop {
//         let data = generate_random_data();
//         arc1.write().unwrap().push(data);
//         thread::sleep(Duration::from_secs(10));
//     });

//     let t2 = thread::spawn(move || loop {
//         println!("Data: {:?}", arc2.read().unwrap());
//         thread::sleep(Duration::from_secs(5));
//     });

//     let t3 = thread::spawn(move || loop {
//         thread::sleep(Duration::from_secs(15));
//         let popped_data = arc3.write().unwrap().pop();
//         println!("Popped Data: {:?}", popped_data);
//     });

//     t1.join().unwrap();
//     t2.join().unwrap();
//     t3.join().unwrap();
// }




// or  


// -----------------------------------------


// use chrono::{DateTime, Utc};
// use rand::Rng;
// use std::sync::{Arc, RwLock};
// use std::thread;
// use std::time::Duration;

// #[derive(Debug)]
// pub struct PersonData {
//     pub id: u64,
//     pub name: String,
//     pub timestamp: DateTime<Utc>,
// }

// fn generate_random_data() -> PersonData {
//     // using rand crate we are generating a random number.
//     let mut rng = rand::thread_rng();
//     let id: u64 = rng.gen_range(1..=10);

//     // this function call will generate a random name
//     let name = generate_random_human_name();

//     // this function will return current local tiume.
//     let timestamp = get_current_timestamp();

//     PersonData {
//         id,
//         name,
//         timestamp,
//     }
// }

// fn generate_random_human_name() -> String {
//     let mut rd = rand::thread_rng();
//     let cap_chaaracter = rd.gen_range('A'..='Z');
//     let small_chaaracter = rd.gen_range('a'..='z');
//     let random_length = rd.gen_range(1..=10);
//     let mut name = String::new();
//     for i in 0..1 {
//         name.push(cap_chaaracter);
//         for i in 0..=random_length {
//             name.push(small_chaaracter);
//         }
//     }

//     name
// }

// fn get_current_timestamp() -> DateTime<Utc> {
//     // getting the local time using chrono crate
//     let now: DateTime<Utc> = Utc::now();
//     now
// }

// pub fn main() {
//     // creating a rwlock on a new vector
//     let v = RwLock::new(Vec::new());

//     // creating a arc lock on the vector v
//     let arc = Arc::new(v);

//     let arc1 = Arc::clone(&arc);
//     let arc2 = Arc::clone(&arc);
//     let arc3 = Arc::clone(&arc);

//     let t1 = thread::spawn(move || loop {
//         thread::sleep(Duration::from_secs(5));
//         let data = generate_random_data();

//         println!("data entered : {:?} ", data);
//         arc1.write().unwrap().push(data);
//     });

//     let t2 = thread::spawn(move || loop {
//         thread::sleep(Duration::from_secs(30));
//         println!("Data: {:?}", arc2.read().unwrap());
//     });

//     let t3 = thread::spawn(move || loop {
//         thread::sleep(Duration::from_secs(60));
//         // let popped_data = arc3.write().unwrap().pop();
//         // println!("Popped Data: {:?}", popped_data);

//         let mut data = arc3.write().unwrap();

//         data.retain(|x| {
//             let current_time = Utc::now();
//             let duration = current_time.signed_duration_since(x.timestamp);
//             duration.num_seconds() < 50
//         });

//         println!("Data removed.")
//     });

//     t1.join().unwrap();
//     t2.join().unwrap();
//     t3.join().unwrap();
// }
