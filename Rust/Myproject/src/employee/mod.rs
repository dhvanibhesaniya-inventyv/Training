// Question_4 different json files with specified content 

//  1.  mid-level(software developer) and working on rust
//  2. junior emp and working on rust
//  3. senior emp or c#

// use serde::{Deserialize, Serialize};
use serde_json;
use std::fs;

use super::common_struct::{Employee,Skill,Position};


// #[warn(unused_must_use)]

/// Main function for categorizing employees into different JSON files based on their position and skills.

pub fn employee_main() {
    let file_content =
        fs::read_to_string("json_data/employee_json/Employee.json").expect("unable to read json file");

    // Deserialize
    let emp: Vec<Employee> =
        serde_json::from_str(&file_content).expect("unable to parse json file");

    let mut soft_dev_employees: Vec<&Employee> = Vec::new();
    let mut jr_employees: Vec<&Employee> = Vec::new();
    let mut sr_or_c_employees: Vec<&Employee> = Vec::new();
    let mut other_employees: Vec<&Employee> = Vec::new();

    for employee in &emp {

        if let Some(position) = &employee.position {
            match position {
                Position::SoftwareDeveloper if employee.skills.contains(&Skill::Rust) => {
                    soft_dev_employees.push(employee);
                }
                Position::JrSoftwareDeveloper if employee.skills.contains(&Skill::Java) => {
                    jr_employees.push(employee);
                }
                Position::SrSoftwareDeveloper => {
                    sr_or_c_employees.push(employee);
                }
                _ => {
                    if employee.skills.contains(&Skill::CSharp) {
                        sr_or_c_employees.push(employee);
                    } 
            }
        }
        }
        else if employee.skills.contains(&Skill::CSharp) {
            sr_or_c_employees.push(employee);
        } else {
            other_employees.push(employee);
        }
    }

        // Convert categorized employees to JSON and write to separate files

    let sdev =
        serde_json::to_string_pretty(&soft_dev_employees).expect("unable to convert to json");
    fs::write("json_data/employee_json/soft_dev_employee.json", sdev)
        .expect("unable to create to json");

    let jrdev =
        serde_json::to_string_pretty(&jr_employees).expect("unable to convert to json ");
    fs::write("json_data/employee_json/jr_dev_employee.json", jrdev)
        .expect("unable to create to json");

    let sr_c_dev =
        serde_json::to_string_pretty(&sr_or_c_employees).expect("unable to convert to json ");
    fs::write("json_data/employee_json/sr_or_c_employee.json", sr_c_dev)
        .expect("unable to create to json");

    let other_dev =
        serde_json::to_string_pretty(&other_employees).expect("unable to convert to json ");
    fs::write("json_data/employee_json/other_employee.json", other_dev)
        .expect("unable to create to json");
}
