use rand::Rng;

use crate::{common_struct::RequestData, thread::task_manager::REF1, thread::task_manager::SKILLS};

pub fn random_data() {
    let random_skill = rand::thread_rng().gen_range(0..=11);
    let random_language = rand::thread_rng().gen_range(0..=1);
    let random_status = rand::thread_rng().gen_range(0..=1);

    // let person_skills = [
    //     "Customer Service",
    //     "Problem-solving",
    //     "Product Knowledge",
    //     "Effective Communication",
    //     "Time Management",
    //     "Adaptability",
    //     "Team Collaboration",
    //     "Feedback Analysis",
    //     "Proactive Engagement",
    //     "Technical Proficiency",
    //     "Cultural Sensitivity",
    //     "Documentation",
    // ];
    let skill = SKILLS[random_skill].to_string();

    let person_language = ["English", "Spanish"];
    // let language = match random_language {
    //     0 => Language::English,
    //     1 => Language::Spanish,
    //     _ => Language::English,
    // };
    let language = person_language[random_language].to_string();

    let person_skill = ["Chat", "Call"];
    // let status = match random_status {
    //     0 => Available::Chat,
    //     1 => Available::Call,
    //     _ => Available::Chat,
    // };
    let status = person_skill[random_status].to_string();

    let request_data = RequestData {
        skill,
        language,
        status,
        timestamp: chrono::Utc::now(),
    };

    let mut data = REF1.write().expect("unavailable");
    data.push_back(request_data);
    
    println!("Random data generated: {:?}", data);
    // println!("Random data generated: .........");

    // Optionally pop the data if you don't need it in the vector
    // data.pop();
}
