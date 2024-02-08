use crate::{common_struct::Skill, thread::task_manager::{SKILLS, USER_DATA}};
use rand::{thread_rng, Rng};

pub fn update_user_data_randomly() {
    let mut user_data = USER_DATA.write().unwrap();

    // let person_skills = vec![
    //     "Customer Service".to_string(),
    //     "Problem-solving".to_string(),
    //     "Product Knowledge".to_string(),
    //     "Effective Communication".to_string(),
    //     "Time Management".to_string(),
    //     "Adaptability".to_string(),
    //     "Team Collaboration".to_string(),
    //     "Feedback Analysis".to_string(),
    //     "Proactive Engagement".to_string(),
    //     "Technical Proficiency".to_string(),
    //     "Cultural Sensitivity".to_string(),
    //     "Documentation".to_string(),
    // ];

    let person_language = ["English", "Spanish"];
    let statuses = ["Online", "Offline"];

    for user in user_data.iter_mut() {
        user.skills.clear();
        user.language.clear();
        user.status.clear();

        let num_skills = thread_rng().gen_range(2..=3);

        let random_language_index = thread_rng().gen_range(0..=1);
        user.language = person_language[random_language_index].to_string();

        let random_status_index = thread_rng().gen_range(0..=1);
        user.status = statuses[random_status_index].to_string();

        let mut rng = thread_rng();

        for _ in 0..num_skills {
            let skill_index = rng.gen_range(0..SKILLS.len());
            let new_skill = SKILLS[skill_index].to_string();

            if !user.skills.contains(&new_skill) {
                user.skills.push(new_skill);
            } else {
                let mut new_skill_index = rng.gen_range(0..SKILLS.len());
                while user.skills.contains(&SKILLS[new_skill_index]) {
                    new_skill_index = rng.gen_range(0..SKILLS.len());
                }
                user.skills.push(SKILLS[new_skill_index].to_string());
            }
        }
    }

    println!("Shuffled USER_DATA . . . . . . .");
    // println!("Updated USER_DATA: {:?}", user_data);
}
