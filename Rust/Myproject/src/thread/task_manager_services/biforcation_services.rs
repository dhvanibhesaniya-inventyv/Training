
use crate::{common_struct::Available, thread::task_manager::{CALL_VECTOR, CHAT_VECTOR, REF2}};


pub fn biforcation_data(){

        let mut data2 = REF2.write().unwrap();

        for _i in 0..data2.len() {
            let popedvalue = data2.pop().unwrap();

            match popedvalue.status {
                Available::Chat => {
                    CHAT_VECTOR.write().unwrap().push_back(popedvalue);
                }
                Available::Call => {
                    CALL_VECTOR.write().unwrap().push_back(popedvalue);
                }
            }
        }
    }