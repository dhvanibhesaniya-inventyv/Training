// Question

//  1.  mid-level(software developer) and working on rust
//  2. junior emp and working on rust
//  3. senior emp or c#

use serde::{Deserialize, Serialize};
use serde_json;
use std::fs;
#[warn(unused_must_use)]
#[derive(Debug, Deserialize, Serialize)]
struct Employee {
    name: String,
    age: i32,
    skills: Vec<String>,
    position: Option<String>,
    #[serde(rename(serialize = "experiance(y)", deserialize = "experiance(y)"))]
    experiance: Option<i32>,
}

fn main() {
    let file_content = fs::read_to_string("./json_file/Employee.json").expect("unable json file");

    //deserialize
    let emp: Vec<Employee> = serde_json::from_str(&file_content).expect("unable json file");

    let mut soft_dev_employees: Vec<&Employee> = Vec::new();
    let mut jr_employees: Vec<&Employee> = Vec::new();
    let mut sr_or_c_employees: Vec<&Employee> = Vec::new();
    let mut other_employees: Vec<&Employee> = Vec::new();

    for i in 0..emp.len() {
        let employee = &emp[i];

        if let Some(position) = &employee.position {
            //  1. mid-level(software developer) and working on rust

            if position == "Software Developer" && employee.skills.contains(&"Rust".to_string()) {
                soft_dev_employees.push(employee);
            }
            //  2. junior emp and working on rust
            else if position == "Jr. Software Developer"
                && employee.skills.contains(&"Java".to_string())
            {
                jr_employees.push(employee);
            }
            //  3. senior emp or c#
            else if position == "Sr. Software Developer"
                || employee.skills.contains(&"C#".to_string())
            {
                sr_or_c_employees.push(employee);
            }
            // other employes
            else {
                other_employees.push(employee);
            }
        } else {
            // Employee has no position, treat as "other_employee"
            other_employees.push(employee);
        }
    }

    let sdev =
        serde_json::to_string_pretty(&soft_dev_employees).expect("unable to convert to json");
    fs::write("./json_file/soft_dev_employee.json", sdev).expect("unable to create to json");

    let jrdev = serde_json::to_string_pretty(&jr_employees).expect("unable to convert to json ");
    fs::write("./json_file/jr_dev_employee.json", jrdev).expect("unable to create to json");

    let sr_c_dev =
        serde_json::to_string_pretty(&sr_or_c_employees).expect("unable to convert to json ");
    fs::write("./json_file/sr_or_c_employee.json", sr_c_dev).expect("unable to create to json");

    let other_dev =
        serde_json::to_string_pretty(&other_employees).expect("unable to convert to json ");
    fs::write("./json_file/other_employee.json", other_dev).expect("unable to create to json");
}





// using match

// use serde::{Deserialize, Serialize};
// use serde_json;
// use std::fs;
// #[warn(unused_must_use)]
// #[derive(Debug, Deserialize, Serialize)]
// struct Employee {
//     name: String,
//     age: i32,
//     skills: Vec<String>,
//     position: Option<String>,
//     #[serde(rename(serialize = "experiance(y)", deserialize = "experiance(y)"))]
//     experiance: Option<i32>,
// }

// fn has_rust_skill(skills: &Vec<String>) -> bool {
//     for skill in skills {
//         if skill == "Rust" {
//             return true;
//         }
//     }
//     false
// }
// fn has_java_skill(skills: &Vec<String>) -> bool {
//     for skill in skills {
//         if skill == "Java" {
//             return true;
//         }
//     }
//     false
// }
// fn has_c_skill(skills: &Vec<String>) -> bool {
//     for skill in skills {
//         if skill == "C#" {
//             return true;
//         }
//     }
//     false
// }

// fn main() {
//     let file_content = fs::read_to_string("./json_file/Employee.json");
//         match file_content {
//             Ok(content) => {
//                 //  println!("{:#?}",content);

//                 //deserialize
//                 let emp: Result<Vec<Employee>, serde_json::Error> = serde_json::from_str(&content);
//                 match emp {
//                     Ok(emp) => {
//                         // println!("{:#?}", emp);

//     let mut soft_dev_employees: Vec<&Employee> = Vec::new();
//     let mut jr_employees: Vec<&Employee> = Vec::new();
//     let mut sr_or_c_employees: Vec<&Employee> = Vec::new();
//     let mut other_employees: Vec<&Employee> = Vec::new();

//     for i in 0..emp.len() {
//         let employee = &emp[i];

//         if let Some(position) = &employee.position {
//             if position == "Software Developer" && has_rust_skill(&employee.skills) {
//                 soft_dev_employees.push(employee);
//             } else if position == "Jr. Software Developer" && has_java_skill(&employee.skills) {
//                 jr_employees.push(employee);
//             } else if position == "Sr. Software Developer" || has_c_skill(&employee.skills) {
//                 sr_or_c_employees.push(employee);
//             }  // other employes
//             else {
//                 other_employees.push(employee);
//             }
//         } else {
//             // Employee has no position, treat as "other_employee"
//             other_employees.push(employee);
//         }
//     }

//     let sdev =
//         serde_json::to_string_pretty(&soft_dev_employees).expect("unable to convert to json");
//     fs::write("./json_file/soft_dev_employee.json", sdev).expect("unable to create to json");

//     let jrdev = serde_json::to_string_pretty(&jr_employees).expect("unable to convert to json ");
//     fs::write("./json_file/jr_dev_employee.json", jrdev).expect("unable to create to json");

//     let sr_c_dev =
//         serde_json::to_string_pretty(&sr_or_c_employees).expect("unable to convert to json ");
//     fs::write("./json_file/sr_or_c_employee.json", sr_c_dev).expect("unable to create to json");

//     let other_dev =
//         serde_json::to_string_pretty(&other_employees).expect("unable to convert to json ");
//     fs::write("./json_file/other_employee.json", other_dev).expect("unable to create to json");

// }
//                 Err(err) => {
//                     eprintln!("Error reading file: {}", err);
//                 }
//             }
//         }
//         Err(_) => {}
//     }
// }




