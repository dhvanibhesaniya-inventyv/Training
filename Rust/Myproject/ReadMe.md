# MyProject

Welcome to MyProject! This repository contains a collection of Rust modules showcasing various functionalities and examples. Each module serves as a learning resource for different aspects of Rust programming.

## Modules Overview

1. `axum-server`  : This module contains an Axum server that performs CRUD operations on JSON files.

2. `Routing-crud`  : An example demonstrating how to create an Axum server for CRUD operations.

3. `Testing`  : Explore testing methodologies in Rust with this module.

4. `Area`  : Implementation of a structure to calculate the area.

5. `common-struct`  : Collection of shared structs, enums, and implementations used across different modules.

6. `employee`  : Module focusing on serialization and deserialization of employee JSON data with specific conditions.

7. `employee-hashmap`  : Task involving separation of data with specific conditions and storing it in a HashMap.

8. `student`  : Handle serialization, deserialization, and percentage calculation for student JSON data.

9. `student_hashmap`  : Similar to the 'student' module but utilizing HashMap for data storage.

10. `table_task`  : Creation of table rows, columns, and cells with calculated dimensions based on cell values.

11. `table_task_hashmap`  : Utilizes JSON data from 'fontData.json' and 'data.json' to create table structures with appropriate dimensions.

12. `thread`  : Implements a TaskManager service that operates with threads, dynamically loading JSON data at runtime.

13. `two_string`  : Module focusing on string functionalities such as string comparison and frequency analysis.

14. `main.rs`  : Entry point containing declared modules used within the project.

## Usage

Each module contains its own set of functionalities and examples. Feel free to explore each module individually to understand its purpose and implementation details.

## Contributing

If you find any issues or have suggestions for improvements, please feel free to open an issue or submit a pull request. Contributions are always welcome!


## Additional Information

For Rust documentation, you can run the following commands:

- `rustup doc`: This command can be executed anywhere in the Rust environment to access Rust documentation.
- `rust doc --open`: If you're using a Cargo project, you can use this command (`cargo doc --open`) to generate and open documentation.

To execute a specific binary file, you can use the following command:

- `cargo run -q --bin file_name`

For creating new Rust projects, you can use the following commands:

- `cargo new --lib moduless`: This command creates a library file.
- `cargo new moduless --bin`: This command creates a binary file.
- `cargo watch -x run`: This command with make compile the code at every change but ,
                        to use this install it globally by this command `cargo install cargo-watch`.
