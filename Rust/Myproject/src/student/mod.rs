// use serde::{Deserialize, Serialize};
use serde_json;
use std::fs;

use crate::common_struct::Student;

/// Calculate percentage and grade for a student based on their marks.

pub fn calculate_percentage_and_grade(student: &mut Student) {
    let total_marks: u32 = student.marks.iter().sum();
    let percentage = total_marks as f64 / student.marks.len() as f64;
    let grade = match percentage {
        _ if percentage >= 90.0 => "A+",
        _ if percentage >= 80.0 => "A",
        _ if percentage >= 70.0 => "B",
        _ if percentage >= 60.0 => "C",
        _ if percentage >= 50.0 => "D",
        _ => "F",
    };

    student.percentage = Some(percentage);
    student.grade = Some(grade.to_string());
}

/// Main function for processing student data, calculating percentage and grade, and updating JSON.

pub fn student_main() {
    // Read JSON file content
    let content =
        fs::read_to_string("json_data/student_json/StudentData.json").expect("Failed to read file");

    //Deserialize
    let mut students: Vec<Student> = serde_json::from_str(&content).expect("Failed to parse JSON");

    for student in &mut students {
        calculate_percentage_and_grade(student);
    }

    // Serialize
    let updated_json = serde_json::to_string_pretty(&students).expect("Failed to serialize JSON");

    fs::write("json_data/student_json/StudentData_new.json", updated_json)
        .expect("Failed to write file");
}
