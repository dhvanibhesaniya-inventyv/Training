use serde::{Deserialize, Serialize};

/// Represents a student with details such as name, phone, email, etc.
#[derive(Debug, Deserialize, Serialize)]
pub struct Student {
    pub name: String,
    pub phone: String,
    pub email: String,
    pub city: String,
    pub address: String,
    pub marks: Vec<u32>,
    pub percentage: Option<f64>,
    pub grade: Option<String>,
}

/// Represents an employee with details like name, age, skills, position, and experience.
#[derive(Debug, Deserialize, Serialize)]
pub struct Employee {
    pub name: String,
    pub age: i32,
    pub skills: Vec<Skill>,
    pub position: Option<Position>,
    #[serde(rename(serialize = "experience(y)", deserialize = "experience(y)"))]
    pub experience: Option<i32>,
}

/// Represents a skill that an employee may possess.
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub enum Skill {
    #[serde(rename = "C#")]
    CSharp,
    Java,
    Rust,
    Python,
}

/// Represents a position that an employee may hold.
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub enum Position {
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

/// Represents a cell in a table with height, width, and a value.
use std::io;
#[derive(Debug)]
pub struct Cell {
    pub height: u32,
    pub width: u32,
    pub value: u32,
}

impl Cell {
    /// Create a new cell with the given height, width, and value.
    pub fn data_assign(height: u32, width: u32, value: u32) -> Cell {
        Cell { height, width, value }
    }
}

/// Represents a row in a table with height, width, cells, and total cells.
#[derive(Debug)]
pub struct Row {
    pub r_height: u32,
    pub r_width: u32,
    pub cells: Vec<Cell>,
    pub total_cells: u32,
}

impl Row {
    /// Create a new row with the given cells.
    pub fn row_data(cells: Vec<Cell>) -> Row {
        let mut r_height: u32 = 0;
        let mut r_width: u32 = 0;
        let total_cells: u32 = cells.len() as u32;

        for i in 0..cells.len() {
            if r_height <= cells[i].height {
                r_height = cells[i].height;
            }
            r_width += cells[i].width;
        }

        Row {
            r_height,
            r_width,
            cells,
            total_cells,
        }
    }

    /// Update the row data based on the cells.
    pub fn update_row_data(&mut self) {
        self.r_height = 0;
        self.r_width = 0;

        for i in 0..self.cells.len() {
            if self.r_height <= self.cells[i].height {
                self.r_height = self.cells[i].height;
            }
            self.r_width += self.cells[i].width;
        }
    }
}

/// Represents a table with rows, height, width, row count, and cell count.
#[derive(Debug)]
pub struct Table {
    pub rows: Vec<Row>,
    pub t_height: u32,
    pub t_width: u32,
    pub t_row: u32,
    pub t_cell: u32,
}


impl Table {
    /// Creates a new `Table` instance with specified rows.
    pub fn table_data(rows: Vec<Row>) -> Table {
        let mut t_height: u32 = 0;
        let mut t_width: u32 = 0;
        let mut t_cell: u32 = 0;
        let t_row: u32 = rows.len() as u32;

        for i in 0..rows.len() {
            if t_width <= rows[i].r_width {
                t_width = rows[i].r_width;
            }
            t_height += rows[i].r_height;
            t_cell += rows[i].total_cells;
        }

        Table {
            rows,
            t_height,
            t_width,
            t_row,
            t_cell,
        }
    }

    /// Updates the total height and width of the table based on its rows.
    pub fn update_table_data(&mut self) {
        self.t_height = 0;
        self.t_width = 0;

        for i in 0..self.rows.len() {
            if self.t_width <= self.rows[i].r_width {
                self.t_width = self.rows[i].r_width;
            }
            self.t_height += self.rows[i].r_height;
        }
    }
}

/// Reads user input from the console and parses it to the specified type.
pub fn read_user_input<T>() -> T
where
    T: std::str::FromStr,
    T::Err: std::fmt::Debug,
{
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().parse().expect("Failed to parse input")
}