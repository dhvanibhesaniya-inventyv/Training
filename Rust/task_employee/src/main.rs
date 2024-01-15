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

fn has_rust_skill(skills: &Vec<String>) -> bool {
    for skill in skills {
        if skill == "Rust" {
            return true;
        }
    }
    false
}
fn has_java_skill(skills: &Vec<String>) -> bool {
    for skill in skills {
        if skill == "Java" {
            return true;
        }
    }
    false
}
fn has_c_skill(skills: &Vec<String>) -> bool {
    for skill in skills {
        if skill == "C#" {
            return true;
        }
    }
    false
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
            if position == "Software Developer" && has_rust_skill(&employee.skills) {
                soft_dev_employees.push(employee);
            } else if position == "Jr. Software Developer" && has_java_skill(&employee.skills) {
                jr_employees.push(employee);
            } else if position == "Sr. Software Developer" || has_c_skill(&employee.skills) {
                sr_or_c_employees.push(employee);
            } else if position == "Project Manager"
                || (has_java_skill(&employee.skills) && has_c_skill(&employee.skills))
            {
                other_employees.push(employee);
            }
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

// fn main() {
//     let file_content = fs::read_to_string("./json_file/Employee.json");
//     match file_content {
//         Ok(content) => {
//             //  println!("{:#?}",content);

//             //deserialize
//             let emp: Result<Vec<Employee>, serde_json::Error> = serde_json::from_str(&content);
//             match emp {
//                 Ok(emp) => {
//                     // println!("{:#?}", emp);

//                     let mut soft_dev_employees: Vec<&Employee> = Vec::new();
//                     let mut jr_employees: Vec<&Employee> = Vec::new();
//                     let mut sr_or_c_employees: Vec<&Employee> = Vec::new();
//                     let mut other_employees: Vec<&Employee> = Vec::new();

//                     // Check each employee as sr dev and working on rust

//                     for employee in &emp {
//                         if let Some(position) = &employee.position {
//                             if position == "Software Developer"
//                                 && employee.skills.contains(&"Rust".to_string())
//                             {
//                                 soft_dev_employees.push(employee);
//                             }
//                         }
//                     }

//                     // Print the soft_dev_employees

//                     // for emp in soft_dev_employees {
//                     //     println!("{:#?}", emp);
//                     // }

//                     // seriliaze

//                     let sdev = serde_json::to_string_pretty(&soft_dev_employees)
//                         .expect("unable to convert to json");
//                     fs::write("./json_file/soft_dev_employee.json", sdev)
//                         .expect("unable to create to json");

//                     //-------------------------------------------------------------------------------------------------------------------------------------------
//                     // check for emp as jr dev and working on rust

//                     for employee in &emp {
//                         if let Some(position) = &employee.position {
//                             if position == "Jr. Software Developer"
//                                 && employee.skills.contains(&"Java".to_string())
//                             {
//                                 jr_employees.push(employee);
//                             }
//                         }
//                     }

//                     // print the jr_employee

//                     // for i in jr_employees{
//                     //     println!("{:#?}",i);
//                     // }

//                     // serialize

//                     let jrdev = serde_json::to_string_pretty(&jr_employees)
//                         .expect("unable to convert to json ");
//                     fs::write("./json_file/jr_dev_employee.json", jrdev)
//                         .expect("unable to create to json");

//                     //-------------------------------------------------------------------------------------------------------------------------------------------

//                     // check for emp as sr or C# dev and working on rust

//                     for employee in &emp {
//                         if let Some(position) = &employee.position {
//                             if position == "Sr. Software Developer"
//                                 || employee.skills.contains(&"C#".to_string())
//                             {
//                                 sr_or_c_employees.push(employee);
//                             }
//                         }
//                     }

//                     //  //  print the sr_c_dev

//                     // for i in sr_or_c_employees{
//                     //     println!("{:#?}",i);
//                     // }

//                     // //serialize

//                     let sr_c_dev = serde_json::to_string_pretty(&sr_or_c_employees)
//                         .expect("unable to convert to json ");
//                     fs::write("./json_file/sr_or_c_employee.json", sr_c_dev)
//                         .expect("unable to create to json");

//                     //-------------------------------------------------------------------------------------------------------------------------------------------

//                     // check for emp as no position and product manager and working on rust

//                     for employee in &emp {
//                         if let Some(position) = &employee.position {
//                             if position == "Project Manager" {
//                                 other_employees.push(employee);
//                             } else {
//                                 // if employee.position.is_none() && employee.experiance.is_none() {
//                                 //     other_employees.push(employee);

//                                 if employee.skills.contains(&"C#".to_string()) && employee.skills.contains(&"Rust".to_string()){
//                                     other_employees.push(employee);
//                                 }

//                             }
//                         }
//                         // else {
//                         //     // Employee has no position
//                         //     other_employees.push(employee);
//                         // }
//                     }

//                     //  //  print the sr_c_dev

//                     // for i in sr_or_c_employees{
//                     //     println!("{:#?}",i);
//                     // }

//                     // //serialize

//                     let other_dev = serde_json::to_string_pretty(&other_employees)
//                         .expect("unable to convert to json ");
//                     fs::write("./json_file/other_employee.json", other_dev)
//                         .expect("unable to create to json");
//                 }
//                 Err(err) => {
//                     eprintln!("Error reading file: {}", err);
//                 }
//             }
//         }
//         Err(_) => {}
//     }
// }
