
use std::{collections::HashMap, fs};
use serde_json::{self, json, Value};


use crate::common_struct::Student;

/// Calculates the percentage and assigns the corresponding grade to a student.

fn calculate_percentage_and_grade(student: &mut Student) {
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

/// Main function for processing students and updating JSON data using HashMap.

pub fn student_hashmap_main() {

    let content = fs::read_to_string("json_data/student_hashmap_json/StudentData.json").expect("Failed to read file");

    //Deserialize
    let mut students: Vec<Student> = serde_json::from_str(&content).expect("Failed to parse JSON");

    for student in &mut students {
        calculate_percentage_and_grade(student);
    }
    // Create a vector of HashMaps to represent students with JSON-compatible data

    let mut student_vec: Vec<HashMap<&str, Value>> = Vec::new();

     // Convert each student to a HashMap and add it to the vector
    for student in students{
        let mut student_hashmap: HashMap<&str, Value> = HashMap::new();
        student_hashmap.insert("name:", Value::String(student.name.to_string()));
        student_hashmap.insert("phone:", Value::String(student.phone.to_string()));
        student_hashmap.insert("email:", Value::String(student.email.to_string()));
        student_hashmap.insert("city:", Value::String(student.city.to_string()));
        student_hashmap.insert("address:", Value::String(student.address.to_string()));
        student_hashmap.insert("marks:", json!(student.marks));
        student_hashmap.insert("percentage:", json!(student.percentage));
        student_hashmap.insert("grade:", json!(student.grade));
        student_vec.push(student_hashmap);
    }

    // Serialize the updated student data to a JSON string
    let updated_json = serde_json::to_string_pretty(&student_vec).expect("Failed to serialize JSON");

    fs::write("json_data/student_hashmap_json/StudentData_updated.json",updated_json).expect("Failed to write file");
}








