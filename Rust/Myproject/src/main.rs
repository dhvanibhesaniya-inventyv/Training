use crate::employee::employee_main;
use crate::student::student_main;
use crate::table_task::main as table_main;
use crate::two_string::gussing_game::gussing_main;
use crate::two_string::single_string::main as single_main;
use crate::two_string::to_string::two_string_main;

pub mod common_struct;
pub mod employee;
pub mod student;
pub mod table_task;
pub mod two_string;

fn main() {
    // student_main();
    // employee_main();
    // gussing_main(); // two_string::gussing_game::gussing_main();
    two_string_main(); // two_string::to_string::two_string_main();
    // single_main();
    // table_main();
}
