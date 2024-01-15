
//updates in different file

use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};

#[derive(Debug, Deserialize, Serialize)]
struct Student {
    name: String,
    phone: String,
    email: String,
    city: String,
    address: String,
    marks: Vec<u32>,
    percentage: Option<f64>,   // some , none
    grade: Option<String>,
}

fn calculate_percentage(marks: &[u32]) -> f64 {
    let total: u32 = marks.iter().sum();
    (total as f64) / (marks.len() as f64)
}

fn calculate_grade(percentage: f64) -> String {
    match percentage {
                p if p >= 90.0 => String::from("A+"),
                p if p >= 80.0 => String::from("A"),
                p if p >= 70.0 => String::from("B+"),
                p if p >= 60.0 => String::from("B"),
                p if p >= 50.0 => String::from("C+"),
                p if p >= 40.0 => String::from("C"),
                _ => String::from("F"),
            }
}

fn main() {
    let file_path = "D:/inventyv/Training/gitdemo/Training/Rust/Json_files/StudentData.json";
    let new_file_path = "D:/inventyv/Training/gitdemo/Training/Rust/Json_files/StudentData_3.json";

    // Read the JSON file
    let mut file = File::open(&file_path).expect("Unable to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read file");
    
    // Deserialize the JSON data into a Vec<Student>
    let mut students: Vec<Student> = serde_json::from_str(&contents).expect("Unable to parse JSON");

    // Calculate percentage and grade for each student and add fields to JSON data
    for student in &mut students {
        if student.percentage.is_none() {
            let percentage = calculate_percentage(&student.marks);
            let grade = calculate_grade(percentage);
    
            student.percentage = Some(percentage);
            student.grade = Some(grade);
        }
    }

    // Write updated JSON data to a new file
    let mut new_file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&new_file_path)
        .expect("Unable to create/open file for writing");

    new_file
        .write_all(serde_json::to_string_pretty(&students).unwrap().as_bytes())
        .expect("Unable to write to file");
}






// use serde::{Deserialize, Serialize};
// use std::fs;


// #[derive(Debug, Deserialize, Serialize)]
// struct Student {
//     name: String,
//     phone: String,
//     email: String,
//     city: String,
//     address: String,
//     marks: Vec<u32>,
//     percentage: Option<f64>,
//     grade: Option<String>,
// }

// fn calculate_percentage_and_grade(student: &mut Student) {
//     let total_marks: u32 = student.marks.iter().sum();
//     let percentage = total_marks as f64 / student.marks.len() as f64;
//     let grade = match percentage {
//         _ if percentage >= 90.0 => "A+",
//         _ if percentage >= 80.0 => "A",
//         _ if percentage >= 70.0 => "B",
//         _ if percentage >= 60.0 => "C",
//         _ if percentage >= 50.0 => "D",
//         _ => "F",
//     };

//     student.percentage = Some(percentage);
//     student.grade = Some(grade.to_string());
// }


// fn main() {
//     // let path = "";

//     let content = fs::read_to_string("D:/inventyv/Training/gitdemo/Training/Rust/serdjson/src/main.rs").expect("Failed to read file");

//     //Deserialize
//     let mut students: Vec<Student> = serde_json::from_str(&content).expect("Failed to parse JSON");

//     for student in &mut students {
//         calculate_percentage_and_grade(student);
//     }

//     // Serialize
//     let updated_json = serde_json::to_string_pretty(&students).expect("Failed to serialize JSON");

//     fs::write("D:/inventyv/Training/gitdemo/Training/Rust/Json_files/StudentData_3.json",updated_json).expect("Failed to write file");
// }















//   updates in same file 

// use serde::{Deserialize, Serialize};
// use std::fs::File;
// use std::io::{Read, Write};

// #[derive(Debug, Deserialize, Serialize)]
// struct Student {
//     name: String,
//     phone: String,
//     email: String,
//     city: String,
//     address: String,
//     marks: Vec<u32>,
//     percentage: Option<f64>,
//     grade: Option<String>,
// }

// fn calculate_percentage(marks: &[u32]) -> f64 {
//     let total: u32 = marks.iter().sum();
//     (total as f64) / (marks.len() as f64)
// }

// fn calculate_grade(percentage: f64) -> String {
//     match percentage {
//         p if p >= 90.0 => String::from("A+"),
//         p if p >= 80.0 => String::from("A"),
//         p if p >= 70.0 => String::from("B+"),
//         p if p >= 60.0 => String::from("B"),
//         p if p >= 50.0 => String::from("C+"),
//         p if p >= 40.0 => String::from("C"),
//         _ => String::from("F"),
//     }
// }

// fn main() {
//     let file_path = "D:/inventyv/Training/gitdemo/Training/Rust/Json_files/StudentData.json";

//     // Read the JSON file
//     let mut file = File::open(&file_path).expect("Unable to open file");
//     let mut contents = String::new();
//     file.read_to_string(&mut contents).expect("Unable to read file");

//     // Deserialize the JSON data into a Vec<Student>
//     let mut students: Vec<Student> = serde_json::from_str(&contents).expect("Unable to parse JSON");

//     // Calculate percentage and grade for each student and add fields to JSON data
//     for student in &mut students {
//         if student.percentage.is_none() {
//             let percentage = calculate_percentage(&student.marks);
//             let grade = calculate_grade(percentage);
    
//             student.percentage = Some(percentage);
//             student.grade = Some(grade);
//         }
//     }

//     // Write updated JSON data to a new file (automatically created or truncated)
//     let mut new_file = File::create(&file_path)
//         .expect("Unable to create/open file for writing");
// // Serialize the updated data and write to the new file
//     new_file
//         .write_all(serde_json::to_string_pretty(&students).unwrap().as_bytes())
//         .expect("Unable to write to file");
// }
