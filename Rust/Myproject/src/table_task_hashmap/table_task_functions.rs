use lazy_static::lazy_static;
use crate::common_struct::table_hashmap_struct_enum::{Cell, HeaderRow, JsonData};
use std::{collections::HashMap, fs};

// get the json data values in hashmap using lazystatic

lazy_static! {
    static ref WIDTH_MAP: HashMap<String, f64> = {
        // let json_data = read_json_file();
        let file_contents = fs::read_to_string("json_data/table_task_hashmap_json/fontData.json").expect("Failed to read the JSON file");

        let json_data: HashMap<String, f64> =
            serde_json::from_str(&file_contents).expect("Failed to deserialize JSON");
        let mut width_map = HashMap::new();

        for (key, value) in json_data {

                // Insert the character and its corresponding width into the hashmap
                width_map.insert(key, value);

        }

        width_map
    };
}

// processing the header data and its value  using font data to set the values in cell

pub fn process_header_row(header_row: &HeaderRow, json_data: &JsonData) -> Vec<Cell> {
    let padding_top = 2.0;
    let padding_bottom = 2.0;
    let padding_left = 2.0;
    let padding_right = 2.0;

    let page_margin_hight = 10.0;
    let page_margin_width = 10.0;

    let page_width = json_data.page_width as f64 - page_margin_hight - page_margin_width; // 792 - 10 -  10 = 772
    let mut cell_width_limit = page_width / header_row.title.len() as f64; // 772 / 16  = 48.25

    cell_width_limit = cell_width_limit - padding_left - padding_right; //  48.25 - 2 - 2 = 44.5

    let mut header_cells = Vec::new();

    for value in &header_row.title {
        let mut total_height: f64 = header_row.font_size as f64; // 18.0
                                                                 // println!("{}",total_height);
        let mut total_width: f64 = 0.0;
        let mut area = cell_width_limit;
        let mut val = String::new();

        for each_char in value.chars() {
            let char_width = WIDTH_MAP.get(&each_char.to_string()).unwrap_or(&12.0);
            total_width += char_width * header_row.font_size as f64;
            if area < total_width {
                total_height += header_row.font_size as f64 + 1.0; // total_hight + 18 + 1.
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

    header_cells
}

// processing the data rows and its value  using font data to set the values in cell

pub fn process_data_row(data_row: &Vec<String>, json_data: &JsonData, font_size: i32) -> Vec<Cell> {
    let padding_top = 2.0;
    let padding_bottom = 2.0;
    let padding_left = 2.0;
    let padding_right = 2.0;

    let page_margin_hight = 10.0;
    let page_margin_width = 10.0;

    let page_width = json_data.page_width as f64 - page_margin_hight - page_margin_width; // 792 - 10 -  10 = 772
    let mut cell_width_limit = page_width / data_row.len() as f64; //  772 / 16  = 48.25
    cell_width_limit = cell_width_limit - padding_left - padding_right; //  48.5 - 2 - 2 = 44.5

    // println!("{}",data_row.len());

    let mut cells = Vec::new();
    for value in data_row {
        let mut total_height: f64 = font_size as f64; // 12

        let mut total_width: f64 = 0.0;
        let mut area = cell_width_limit;
        let mut val = String::new();

        for each_char in value.chars() {
            let char_width = WIDTH_MAP.get(&each_char.to_string()).unwrap_or(&12.0);
            total_width += char_width * font_size as f64;

            if area < total_width {
                total_height += font_size as f64 + 1.0; //  total_height + 18  + 1
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

    cells
}
