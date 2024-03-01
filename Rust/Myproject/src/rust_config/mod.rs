mod resources;
use resources::config_create;

pub fn main() {
    let config_try = config_create().unwrap();
    println!("{:?}",config_try.get::<String>("Couchbase.username").unwrap());
}


// set RUN_MODE=config_test