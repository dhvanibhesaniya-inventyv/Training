
//!  MyProject contain 5 modules.

// use crate::employee::employee_main;
// use crate::student::student_main;
// use crate::table_task::main as table_main;
// use crate::two_string::gussing_game::gussing_main;
// use crate::two_string::single_string::main as single_main;
// use crate::two_string::to_string::two_string_main;
// use crate::area::main as area_main;
use crate::employee_hashmap::employee_hashmap_main;
// use crate::student_hashmap::student_hashmap_main;

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


pub mod employee_hashmap;
pub mod student_hashmap;


/// The main entry point for the application.
///
/// It calls functions from various modules to demonstrate functionality.
fn main() {
    // Uncomment the functions you want to include in the documentation.

    // student_main();
    // employee_main();
    // gussing_main(); // two_string::gussing_game::gussing_main();
    // two_string_main(); // two_string::to_string::two_string_main();
    // single_main();
    //  table_main();
    // area_main();
    employee_hashmap_main();
    // student_hashmap_main()
    


}
