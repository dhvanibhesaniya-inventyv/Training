use std::{thread, time::Duration};

fn main(){
    thread::spawn(|| {
        for i in 1..10{
            println!("spawn :: {}",i);
//  thread::sleep(Duration::from_millis(1));
        }
    });



    
    for i in 1..5{
        println!("main :: {}",i);
// thread::sleep(Duration::from_millis(1));
    }
}





//In Rust, "threads" generally refer to concurrent execution units.
//  Rust provides built-in support for multi-threading through the std::thread module. 
// Threads in Rust are similar to threads in other programming languages and operating 
// systems, allowing you to execute multiple pieces of code concurrently.