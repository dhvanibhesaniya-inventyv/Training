//!  MyProject contain 10 modules.

// use crate::employee::employee_main;
// use crate::student::student_main;
// use crate::table_task::main as table_main;
// use crate::two_string::gussing_game::gussing_main;
// use crate::two_string::single_string::main as single_main;
// use crate::two_string::to_string::two_string_main;
// use crate::area::main as area_main;
// use crate::employee_hashmap::employee_hashmap_main;
// use crate::student_hashmap::student_hashmap_main;
// use crate::table_task_hashmap::table_task_hashmap_pdf;
//use crate::thread::person_data_thread::person_data_main;
// use crate::thread::task_manager::task_manager_main;
use crate::Routing_Crud::main as crud_operation;

// use crate::Testing::simple_testing::main as tesing;


/// The `common_struct` module contains common data structures used in the crate.
pub mod common_struct;

/// The `employee` module contains functionality related to employees.
pub mod employee;

/// The `student` module contains functionality related to students.
pub mod student;

/// The `table_task` module contains functionality related to table tasks.
pub mod table_task;

/// The `two_string` module contains functionality related to operations on two strings.
pub mod two_string;

///  The `area` module contains values and function calling.
pub mod area;

/// The `employee` module contains functionality related to employees.
pub mod employee_hashmap;

/// The `student` module contains functionality related to students.
pub mod student_hashmap;

/// The `table_task` module contains functionality related to table tasks. and 
pub mod table_task_hashmap;


/// The `thread` module contains functionality related to threads.
pub mod thread;

/// The `Testing` module contains functionality related to testing.
pub mod Testing;

/// The `Routing_crud` module contain an server with crusd operations.
pub mod Routing_Crud;


/// The main entry point for the application.
///
/// It calls functions from various modules to demonstrate functionality.
pub fn main() {
    // Uncomment the functions you want to include in the documentation.

    // student_main();
    // employee_main();
    
    // gussing_main();         // two_string::gussing_game::gussing_main();
    // two_string_main();      // two_string::to_string::two_string_main();
    // single_main();
    //  table_main();
    // area_main();
    // employee_hashmap_main();
    // student_hashmap_main()
    // table_task_hashmap_pdf();

    // person_data_main();
    // task_manager_main();
    // tesing();

    crud_operation();

    
}
