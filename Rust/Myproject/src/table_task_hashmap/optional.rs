// json data formating done in this code.


use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::{cell, fs};

#[derive(Debug, Serialize, Deserialize)]
enum DataType {
    Header,
    Row,
}

#[derive(Debug, Serialize, Deserialize)]
struct Cell {
    height: u32,
    width: f64,
    value: String,
}

impl Cell {
    fn data_assign(height: u32, width: f64, value: String) -> Cell {
        Cell { height, width, value }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Row {
    data_type: DataType,
    r_height: u32,
    r_width: f64,
    cells: Vec<Cell>,
    total_cells: u32,
}

impl Row {
    fn row_data(mut cells: Vec<Cell>, data_type: DataType) -> Row {
        let mut r_height: u32 = 0;
        let mut r_width: f64 = 0.0;
        let total_cells: u32 = cells.len() as u32;

        for i in 0..cells.len() {
            if r_height <= cells[i].height {
                r_height = cells[i].height;
            }
            r_width += cells[i].width;
        }

        let cell_max_height = &r_height;
        for i in 0..cells.len() {
            cells[i].height = *cell_max_height;
        }

        Row {
            data_type,
            r_height,
            r_width,
            cells,
            total_cells,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Table {
    // header: Row,
    rows: Vec<Row>,
    t_height: u32,
    t_width: f64,
    t_row: u32,
    t_cell: u32,
}

impl Table {
    fn table_data(rows: Vec<Row>) -> Table {
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
            // header,
            rows,
            t_height,
            t_width,
            t_row,
            t_cell,
        }
    }
}

#[derive(Debug, Deserialize)]
struct HeaderRow {
    #[serde(rename = "fontSize")]
    font_size: i32,
    title: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct DataRow {
    #[serde(rename = "fontSize")]
    font_size: i32,
    rows: Vec<Vec<String>>,
}

#[derive(Debug, Deserialize)]
struct JsonData {
    headerRow: HeaderRow,
    dataRows: DataRow,
    #[serde(rename = "pageWidth")]
    page_width: f64,
}

lazy_static! {
    static ref WIDTH_MAP: HashMap<String, f64> = {
        let json_data = read_json_file("data_json/fontData.json");
        let mut width_map = HashMap::new();

        for (key, value) in json_data {
            width_map.insert(key, value);
        }

        width_map
    };
}

fn read_json_file(file_path: &str) -> HashMap<String, f64> {
    let file_contents = fs::read_to_string(file_path).expect("Failed to read the JSON file");
    serde_json::from_str(&file_contents).expect("Failed to deserialize JSON")
}

fn main() {
    let json_content =
        fs::read_to_string("data_json/data.json").expect("Failed to read the JSON file");
    let json_data: JsonData =
        serde_json::from_str(&json_content).expect("Failed to deserialize JSON");

    let padding_top = 2.0;
    let padding_bottom = 2.0;
    let padding_left = 2.0;
    let padding_right = 2.0;

    let page_margin_height = 10.0;
    let page_margin_width = 10.0;

    let page_width =
        json_data.page_width as f64 - page_margin_height - page_margin_width; // 792 - 10 -  10 = 772

    let mut all_datas: Vec<Row> = Vec::new();

    // Process header row
    let mut header_cells = Vec::new();


    let mut cell_width_limit = page_width / json_data.headerRow.title.len() as f64; // 772 / 16  = 48.25
    cell_width_limit = cell_width_limit - padding_left - padding_right; //  48.5 - 2 - 2 = 44.5

    for value in &json_data.headerRow.title {
        let mut total_height: f64 = json_data.headerRow.font_size as f64; // 18.0
        let mut total_width: f64 = 0.0;
        let mut area = cell_width_limit;
        let mut val = String::new();

        for each_char in value.chars() {
            let char_width = WIDTH_MAP.get(&each_char.to_string()).unwrap_or(&12.0);
            total_width += char_width * json_data.headerRow.font_size as f64;

            if area < total_width {
                total_height += json_data.headerRow.font_size as f64 + 1.0; //   total_height + 18 + 1.0
                area += cell_width_limit;
                val.push('\n');
            }

            val.push(each_char);
        }

        total_width = cell_width_limit + padding_left + padding_right;
        total_height += padding_top + padding_bottom;

        header_cells.push(Cell::data_assign(
            total_height as u32,
            total_width as f64,
            val,
        ));


    }
    
    all_datas.push(Row::row_data(header_cells, DataType::Header));

    // Process data rows
    // let mut rows = Vec::new();
    for data_row in &json_data.dataRows.rows {
        let mut cells = Vec::new();
        let mut cell_width_limit = page_width / data_row.len() as f64; //  772 / 16  = 48.25
        cell_width_limit = cell_width_limit - padding_left - padding_right; //  48.5 - 2 - 2 = 44.5

        for value in data_row {
            let mut total_height: f64 = json_data.dataRows.font_size as f64; // 12

            let mut total_width: f64 = 0.0;
            let mut area = cell_width_limit;
            let mut val = String::new();

            for each_char in value.chars() {
                let char_width = WIDTH_MAP
                    .get(&each_char.to_string())
                    .unwrap_or(&12.0);
                total_width += char_width * json_data.dataRows.font_size as f64;

                if area < total_width {
                    total_height += json_data.dataRows.font_size as f64 + 1.0; //  total_height + 18  + 1
                    area += cell_width_limit;
                    val.push('\n');
                }

                val.push(each_char);
            }

            total_width = cell_width_limit + padding_left + padding_right;
            total_height += padding_top + padding_bottom;

            cells.push(Cell::data_assign(
                total_height as u32,
                total_width as f64,
                val,
            ));
        }

        // let row = Row::row_data(cells, DataType::Row);
        // rows.push(row);
        
        all_datas.push(Row::row_data(cells, DataType::Row));
    }


    let table = Table::table_data(all_datas);

    let hashmap_table =
        serde_json::to_string_pretty(&table).expect("Failed to serialize JSON");

    fs::write("data_json/hashmap_table_data.json", hashmap_table)
        .expect("Failed to write file");
}
