// Question_6 different json files with specified content  using hashmap

//  1.  mid-level(software developer) and working on rust
//  2. junior emp and working on rust
//  3. senior emp or c#

use std::{collections::HashMap, fs};
use serde_json::{self, json, Value};
use super::common_struct::{Employee,Skill,Position};


// #[warn(unused_must_use)]

/// Main function for categorizing employees into HasHmap data which write in different JSON files based on their position and skills.

pub fn employee_hashmap_main() {
    let file_content =
        fs::read_to_string("json_data/employee_hashmap_json/Employee.json").expect("Unable to read JSON file");

    // Deserialize
    let employees: Vec<Employee> =
        serde_json::from_str(&file_content).expect("Failed to parse JSON");

            // Create vectors ઓફ hashmap to store categorized employees

    let mut soft_dev_employees: Vec<HashMap<String, Value>> = Vec::new();
    let mut jr_employees: Vec<HashMap<String, Value>> = Vec::new();
    let mut sr_or_c_employees: Vec<HashMap<String, Value>> = Vec::new();
    let mut other_employees: Vec<HashMap<String, Value>> = Vec::new();

    for employee in &employees {
        let mut employee_map: HashMap<String, Value> = HashMap::new();

        
        // Insert employee details into the HashMap

        employee_map.insert("name".to_string(), json!(employee.name));
        employee_map.insert("age".to_string(), json!(employee.age));
        employee_map.insert("skills".to_string(), json!(employee.skills));
        employee_map.insert("position".to_string(), json!(employee.position));
        employee_map.insert("experiance".to_string(), json!(employee.experience));

        if let Some(position) = &employee.position {
            match position {
                Position::SoftwareDeveloper if employee.skills.contains(&Skill::Rust) => {
                    soft_dev_employees.push(employee_map);
                }
                Position::JrSoftwareDeveloper if employee.skills.contains(&Skill::Java) => {
                    jr_employees.push(employee_map);
                }
                Position::SrSoftwareDeveloper => {
                    sr_or_c_employees.push(employee_map);
                }
                _ => {
                    if employee.skills.contains(&Skill::CSharp) {
                        sr_or_c_employees.push(employee_map);
                    } 
            }
        }
        }
        else if employee.skills.contains(&Skill::CSharp) {
            sr_or_c_employees.push(employee_map);
        } else {
            other_employees.push(employee_map);
        }
    }
    
    // Write categorized employees to separate JSON files
    
    fs::write(
        "json_data/employee_hashmap_json/soft_dev_employee.json",
        serde_json::to_string_pretty(&soft_dev_employees).expect("Failed to serialize JSON"),
    )
    .expect("Failed to write JSON file");

    fs::write(
        "json_data/employee_hashmap_json/jr_dev_employee.json",
        serde_json::to_string_pretty(&jr_employees).expect("Failed to serialize JSON"),
    )
    .expect("Failed to write JSON file");

    fs::write(
        "json_data/employee_hashmap_json/sr_or_c_employee.json",
        serde_json::to_string_pretty(&sr_or_c_employees).expect("Failed to serialize JSON"),
    )
    .expect("Failed to write JSON file");

    fs::write(
        "json_data/employee_hashmap_json/other_employee.json",
        serde_json::to_string_pretty(&other_employees).expect("Failed to serialize JSON"),
    )
    .expect("Failed to write JSON file");
}