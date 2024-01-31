use serde_json;
use std::fs;

pub mod table_task_functions;

use crate::table_task_hashmap::table_task_functions::{process_data_row, process_header_row};

use crate::common_struct::table_hashmap_struct_enum::{DataType, JsonData, Row, Table};

pub fn table_task_hashmap_pdf() {
    // Read JSON data from file
    let json_content = fs::read_to_string("json_data/table_task_hashmap_json/data.json")
        .expect("Failed to read the JSON file");
    // Deserialize
    let json_data: JsonData =
        serde_json::from_str(&json_content).expect("Failed to deserialize JSON");

    // Process header row
    let header_cells = process_header_row(&json_data.headerRow, &json_data);

    // deffining the header data type
    // let header_datatype = DataType::Header("Header data".to_string());
    // Create header row from the dynamic data
    let header_row = Row::row_data(header_cells, DataType::HeaderData);

    // Process data rows
    let mut rows = Vec::new();
    for data_row in &json_data.dataRows.rows {
        let cells = process_data_row(data_row, &json_data, json_data.dataRows.font_size);

        // deffining the row data type
        // let row_datatype = DataType::Row("Row data".to_string());

        let row = Row::row_data(cells, DataType::RowData);
        rows.push(row);
    }

    // Create table from the dynamic data
    let table = Table::table_data(header_row, rows);

    // Print the table
    // println!("{:#?}", table);

    // Serialize
    let hashmap_table = serde_json::to_string_pretty(&table).expect("Failed to serialize JSON");

    fs::write(
        "json_data/table_task_hashmap_json/hashmap_table_data.json",
        hashmap_table,
    )
    .expect("Failed to write file");
}
