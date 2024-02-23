use lazy_static::lazy_static;

// use crate::common_struct::table_hashmap_struct_enum::{Cell, HeaderRow, JsonData};
use crate::table_task_hashmap::{Cell,HeaderRow,JsonData};
use std::collections::HashMap;

// get the json data values in hashmap using lazystatic


lazy_static! {
    static ref WIDTH_MAP: HashMap<String, f64> = {
        let mut mp: HashMap<String, f64> = HashMap::new();
        mp.insert("0".to_string(), 0.5);
        mp.insert("1".to_string(), 0.5);
        mp.insert("2".to_string(), 0.5);
        mp.insert("3".to_string(), 0.5);
        mp.insert("4".to_string(), 0.5);
        mp.insert("5".to_string(), 0.5);
        mp.insert("6".to_string(), 0.5);
        mp.insert("7".to_string(), 0.5);
        mp.insert("8".to_string(), 0.5);
        mp.insert("9".to_string(), 0.5);
        mp.insert("".to_string(), 0.0);
        mp.insert(" ".to_string(), 0.25);
        mp.insert("!".to_string(), 0.333);
        mp.insert("\"".to_string(), 0.555);
        mp.insert("#".to_string(), 0.5);
        mp.insert("$".to_string(), 0.5);
        mp.insert("%".to_string(),1.0);
        mp.insert("&".to_string(),0.83300006);
        mp.insert("'".to_string(),0.27800003);
        mp.insert("(".to_string(),0.333);
        mp.insert(")".to_string(),0.333);
        mp.insert( "*".to_string(), 0.5);
        mp.insert("+".to_string(),0.57000005);
        mp.insert(":".to_string(),0.25);
        mp.insert("-".to_string(),0.333);
        mp.insert(".".to_string(),0.25);
        mp.insert("/".to_string(),0.27800003);
        mp.insert(",".to_string(),0.333);
        mp.insert(";".to_string(),0.333);
        mp.insert("<".to_string(),0.57000005);
        mp.insert("=".to_string(),0.57000005);
        mp.insert(">".to_string(),0.57000005);
        mp.insert("?".to_string(),0.5);
        mp.insert("@".to_string(),0.93000007);
        mp.insert("A".to_string(),0.72200006);
        mp.insert("B".to_string(),0.66700006);
        mp.insert("C".to_string(),0.72200006);
        mp.insert("D".to_string(),0.72200006);
        mp.insert("E".to_string(),0.66700006);
        mp.insert("F".to_string(),0.611);
        mp.insert("G".to_string(),0.77800006);
        mp.insert("H".to_string(),0.77800006);
        mp.insert("I".to_string(),0.38900003);
        mp.insert("J".to_string(),0.5);
        mp.insert("K".to_string(),0.77800006);
        mp.insert("L".to_string(),0.66700006);
        mp.insert("M".to_string(),0.94400007);
        mp.insert("N".to_string(),0.72200006);
        mp.insert("O".to_string(),0.77800006);
        mp.insert("P".to_string(),0.611);
        mp.insert("Q".to_string(),0.77800006);
        mp.insert("R".to_string(),0.72200006);
        mp.insert("S".to_string(),0.55600005);
        mp.insert("T".to_string(),0.66700006);
        mp.insert("U".to_string(),0.72200006);
        mp.insert("V".to_string(),0.72200006);
        mp.insert("W".to_string(),1.0);
        mp.insert("X".to_string(),0.72200006);
        mp.insert("Y".to_string(),0.72200006);
        mp.insert("Z".to_string(),0.66700006);
        mp.insert("[".to_string(),0.333);
        mp.insert("\\".to_string(), 0.27800003);
        mp.insert("]".to_string(), 0.333);
        mp.insert("^".to_string(), 0.58100003);
        mp.insert("_".to_string(), 0.5);
        mp.insert("`".to_string(), 0.333);
        mp.insert("a".to_string(), 0.5);
        mp.insert("b".to_string(), 0.55600005);
        mp.insert("c".to_string(), 0.44400004);
        mp.insert("d".to_string(), 0.55600005);
        mp.insert("e".to_string(), 0.44400004);
        mp.insert("f".to_string(), 0.333);
        mp.insert("g".to_string(), 0.5);
        mp.insert("h".to_string(), 0.55600005);
        mp.insert("i".to_string(), 0.27800003);
        mp.insert("j".to_string(), 0.333);
        mp.insert("k".to_string(), 0.55600005);
        mp.insert("l".to_string(), 0.27800003);
        mp.insert("m".to_string(), 0.83300006);
        mp.insert("n".to_string(), 0.55600005);
        mp.insert("o".to_string(), 0.5);
        mp.insert("p".to_string(), 0.55600005);
        mp.insert("q".to_string(), 0.55600005);
        mp.insert("r".to_string(), 0.44400004);
        mp.insert("s".to_string(), 0.38900003);
        mp.insert("t".to_string(), 0.333);
        mp.insert("u".to_string(), 0.55600005);
        mp.insert("v".to_string(), 0.5);
        mp.insert("w".to_string(), 0.72200006);
        mp.insert("x".to_string(), 0.5);
        mp.insert("y".to_string(), 0.5);
        mp.insert("z".to_string(), 0.44400004);
        mp.insert("{".to_string(), 0.39400002);
        mp.insert("|".to_string(), 0.22000001);
        mp.insert("}".to_string(), 0.39400002);
        mp.insert("~".to_string(), 0.52000004);
        mp
        
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
