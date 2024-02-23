use serde_json;
use wasm_bindgen::prelude::wasm_bindgen;

pub mod table_task_functions;

use crate::table_task_hashmap::table_task_functions::{process_data_row, process_header_row};

use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize)]
pub enum DataType {
    HeaderData,
    RowData,
}

// table struct to serialize the json data in a cell json format


#[derive(Debug, Serialize, Deserialize)]
pub struct Cell {
    pub height: u32,
    pub width: f64,
    pub value: String,
}

impl Cell {
    pub fn data_assign(height: u32, width: f64, value: String) -> Cell {
        Cell {
            height,
            width,
            value,
        }
    }
}

// table struct to serialize the json data in a row json format

#[derive(Debug, Serialize, Deserialize)]
pub struct Row {
    pub data_type: DataType,
    pub r_height: u32,
    pub r_width: f64,
    pub cells: Vec<Cell>,
    pub total_cells: u32,
}


impl Row {
    //  this function initialize the hight and width of the row.

    pub fn row_data(mut cells: Vec<Cell>, data_type: DataType) -> Row {
        let mut r_height: u32 = 0;
        let mut r_width: f64 = 0.0;
        let total_cells: u32 = cells.len() as u32;

        for i in 0..cells.len() {
            if r_height <= cells[i].height {
                r_height = cells[i].height;
            }
            r_width += cells[i].width;
        }
        let cell_max_hight = &r_height;
        for i in 0..cells.len() {
            cells[i].height = *cell_max_hight;
        }

        Row {
            r_height,
            r_width,
            cells,
            total_cells,
            data_type,
        }
    }
}

// table struct to serialize the json data in a table json format

#[derive(Debug, Serialize, Deserialize)]
pub struct Table {
    pub header: Row,
    pub rows: Vec<Row>,
    pub t_height: u32,
    pub t_width: f64,
    pub t_row: u32,
    pub t_cell: u32,
}

impl Table {
    //  this function initialize the hight and width of the table.
    pub fn table_data(header: Row, rows: Vec<Row>) -> Table {
        let mut t_height: u32 = 0;
        let mut t_width: f64 = 0.0;
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
            header,
            rows,
            t_height,
            t_width,
            t_row,
            t_cell,
        }
    }
}

// this struct created to read the json data from json file

#[derive(Debug, Deserialize)]
pub struct HeaderRow {
    #[serde(rename = "fontSize")]
    pub font_size: i32,
    pub title: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct DataRow {
    #[serde(rename = "fontSize")]
    pub font_size: i32,
    pub rows: Vec<Vec<String>>,
}


#[derive(Debug, Deserialize)]
pub struct JsonData {
    // data_type: DataType,
    pub headerRow: HeaderRow,
    pub dataRows: DataRow,
    #[serde(rename = "pageWidth")]
    pub page_width: f64,
}


#[wasm_bindgen]
pub fn greeting(a:String) -> String{
a +"..ok everything is good.."
}

#[wasm_bindgen]
pub fn table_task_hashmap_pdf(json_data: &str) -> String {
   
  // Deserialize
let json_data: JsonData = match serde_json::from_str(&json_data) {
    Ok(data) => data,
    Err(err) => {
        println!("Failed to deserialize JSON: {}", err);
       
        return "Error to get data".to_string(); // Or any other error handling mechanism you prefer
    }
};

// Process header row
let header_cells = process_header_row(&json_data.headerRow, &json_data);

// Create header row from the dynamic data
let header_row = Row::row_data(header_cells, DataType::HeaderData);

// Process data rows
let mut rows = Vec::new();
for data_row in &json_data.dataRows.rows {
    let cells = process_data_row(data_row, &json_data, json_data.dataRows.font_size);
    let row = Row::row_data(cells, DataType::RowData);
       
    rows.push(row);

}


// Create table from the dynamic data
let table = Table::table_data(header_row, rows);

// Serialize
let json_result = match serde_json::to_string_pretty(&table) {
    Ok(result) => result,
    Err(err) => {
        eprintln!("Failed to serialize JSON: {}", err);
        // Handle the error here, possibly returning early or with a default value
        return  "Error to get data".to_string(); // Or any other error handling mechanism you prefer
    }
};

json_result
}
