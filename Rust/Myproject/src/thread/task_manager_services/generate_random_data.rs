use rand::Rng;

use crate::{common_struct::{Available, RequestData}, thread::task_manager::REF1};

pub fn random_data(){


  

            let random_skill = rand::thread_rng().gen_range(0..=10);
            let random_language = rand::thread_rng().gen_range(0..=1);
            let random_status = rand::thread_rng().gen_range(0..=1);

            let person_skills = [
                "Customer Service",
                "Problem-solving",
                "Product Knowledge",
                "Effective Communication",
                "Time Management",
                "Adaptability",
                "Team Collaboration",
                "Feedback Analysis",
                "Proactive Engagement",
                "Technical Proficiency",
                "Cultural Sensitivity",
                "Documentation",
            ];
            let skill = person_skills[random_skill].to_string();

            let person_language = ["English", "Spanish"];
            let language = person_language[random_language].to_string();

            let status = match random_status {
                0 => Available::Chat,
                1 => Available::Call,
                _ => Available::Chat,
            };

            let request_data = RequestData {
                skill,
                language,
                status,
            };

            let mut data = REF1.write().unwrap();
            data.push(request_data);
            println!("Random data generated: {:?}", data);

            // Optionally pop the data if you don't need it in the vector
            // data.pop();
        }
  