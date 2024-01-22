
use serde::{Deserialize, Serialize};

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

#[derive(Debug, Deserialize, Serialize)]
pub struct Employee {
   pub name: String,
   pub age: i32,
   pub skills: Vec<Skill>,
   pub position: Option<Position>,
   #[serde(rename(serialize = "experience(y)", deserialize = "experience(y)"))]
   pub experience: Option<i32>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub enum Skill {
    #[serde(rename = "C#")]
    CSharp,
    Java,
    Rust,
    Python,
}

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




// table_task struct and enum and input


use std::io;
#[derive(Debug)]
pub struct Cell{
    pub height:u32,
    pub width:u32,
    pub value:u32,
}

impl Cell{

    pub fn data_assign (height: u32,width: u32,value: u32) -> Cell{
    Cell {
        height,
        width,
        value
    }
}

}

#[derive(Debug)]

pub struct Row{
   pub r_height:u32,
   pub r_width:u32,
   pub cells: Vec<Cell>,
   pub total_cells:u32,
}


impl Row{

    pub  fn row_data(cells: Vec<Cell>) -> Row{
        let mut r_height: u32 = 0;
        let mut r_width: u32 = 0;
        let total_cells: u32 = cells.len() as u32;


        for i in 0..cells.len(){
            if r_height <= cells[i].height{
                r_height = cells[i].height;
            }
            r_width += cells[i].width;
        }

        Row { r_height, r_width, cells, total_cells }
    }

    pub  fn update_row_data(&mut self) {
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

#[derive(Debug)]

pub struct Table{
   pub rows:Vec<Row> ,  
   pub t_height:u32,
   pub t_width: u32,
   pub t_row: u32,
   pub t_cell:u32,
}

impl Table{


    pub fn table_data(rows:Vec<Row>) -> Table{
        let mut t_height: u32= 0;
        let mut t_width: u32 = 0;
        let mut t_cell: u32 = 0;
        let t_row: u32 = rows.len() as u32 ;

        for i in 0..rows.len(){
            if t_width <= rows[i].r_width{
                t_width = rows[i].r_width;
            }
            t_height+=rows[i].r_height;
            t_cell += rows[i].total_cells;
        }

        Table { rows,t_height,t_width,t_row,t_cell}
    }

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


pub fn read_user_input<T>() -> T
where
    T: std::str::FromStr,
    T::Err: std::fmt::Debug,
{
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().parse().expect("Failed to parse input")
}

