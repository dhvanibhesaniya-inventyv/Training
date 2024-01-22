use serde::{Deserialize, Serialize};
use serde_json;
use std::fs;

#[warn(unused_must_use)]
#[derive(Debug, Deserialize, Serialize)]
struct Employee {
    name: String,
    age: i32,
    skills: Vec<Skill>,
    position: Option<Position>,
    #[serde(rename(serialize = "experience(y)", deserialize = "experience(y)"))]
    experience: Option<i32>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
enum Skill {
    #[serde(rename = "C#")]
    CSharp,
    Java,
    Rust,
    Python,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
enum Position {
    #[serde(rename = "Software Developer")]
    SoftwareDeveloper,
    #[serde(rename = "Jr. Software Developer")]
    JrSoftwareDeveloper,
    #[serde(rename = "Sr. Software Developer")]
    SrSoftwareDeveloper,
    #[serde(rename = "Team Lead")]
    TeamLead,
    #[serde(rename = "Project Manager")]
    ProjectManager,
}

fn main() {
    let file_content =
        fs::read_to_string("./json_file_enum/Employee.json").expect("unable to read json file");

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

    let sdev =
        serde_json::to_string_pretty(&soft_dev_employees).expect("unable to convert to json");
    fs::write("./json_file_enum/soft_dev_employee.json", sdev)
        .expect("unable to create to json");

    let jrdev =
        serde_json::to_string_pretty(&jr_employees).expect("unable to convert to json ");
    fs::write("./json_file_enum/jr_dev_employee.json", jrdev)
        .expect("unable to create to json");

    let sr_c_dev =
        serde_json::to_string_pretty(&sr_or_c_employees).expect("unable to convert to json ");
    fs::write("./json_file_enum/sr_or_c_employee.json", sr_c_dev)
        .expect("unable to create to json");

    let other_dev =
        serde_json::to_string_pretty(&other_employees).expect("unable to convert to json ");
    fs::write("./json_file_enum/other_employee.json", other_dev)
        .expect("unable to create to json");
}
