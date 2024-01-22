
use crate::common_struct::{Cell, Row, Table, read_user_input};
pub fn main(){


    let cell1 = Cell::data_assign(1,2,1);
    let cell2 = Cell::data_assign(2,2,2);


    let row1 = Row::row_data(vec!(cell1,cell2));
    let cell3 = Cell::data_assign(3,2,3);
    let cell4 = Cell::data_assign(4,2,4);
    let row2 = Row::row_data(vec!(cell3,cell4));
    let mut table = Table::table_data(vec!(row1,row2));
    
      // // //println!("{:#?}",table);

    println!("");
    println!("");

    table.rows[1].cells[1] = Cell::data_assign(10,10, 10);

    table.rows[1].update_row_data();

    table.update_table_data();
    

    println!("{:#?}", table);

    // user input

    println!("do you want to update any values ?  if yes then enter  'y'  or  'n' .");
    let yn: String = read_user_input(); 
 
if yn == "y"{
        
    println!("Enter the row number to update:");
    let row_index: usize = read_user_input();
    
    if row_index >= table.rows.len() {
        println!("Invalid row index");
        return;
    }

    println!("Enter the cell number to update:");
    let cell_index: usize = read_user_input();

    if cell_index >= table.rows[row_index].cells.len() {
        println!("Invalid cell index");
        return;
    }

    println!("Enter the cell height:");
    let cell_height: u32 = read_user_input();

    println!("Enter the cell width:");
    let cell_width: u32 = read_user_input();

    println!("Enter the cell value:");
    let cell_value: u32 = read_user_input();

    table.rows[row_index].cells[cell_index] = Cell::data_assign(cell_height, cell_width, cell_value);

    table.rows[row_index].update_row_data();

    table.update_table_data();

    println!("{:#?}", table);

}else{
    println!("thankyou for youy input");
}

}





