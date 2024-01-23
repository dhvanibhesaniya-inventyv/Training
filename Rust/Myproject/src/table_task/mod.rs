use crate::common_struct::{Cell, Row, Table, read_user_input};

/// Main function demonstrating the use of common data structures for creating and updating a table.
pub fn main() {
    // // Creating cells and rows
    // let cell1 = Cell::data_assign(1, 2, 1);
    // let cell2 = Cell::data_assign(2, 2, 2);
    // let row1 = Row::row_data(vec![cell1, cell2]);

    // let cell3 = Cell::data_assign(3, 2, 3);
    // let cell4 = Cell::data_assign(4, 2, 4);
    // let row2 = Row::row_data(vec![cell3, cell4]);

    // // Creating a table with rows
    // let mut table = Table::table_data(vec![row1, row2]);

    // println!("\n\n");

    // // Modifying a cell in the table
    // table.rows[1].cells[1] = Cell::data_assign(10, 10, 10);

    // // Updating row data and table data
    // table.rows[1].update_row_data();
    // table.update_table_data();

    // // Printing the table
    // println!("{:#?}", table);


        
// user input as value -----------------------------------------------------------------------------------------------------

println!("Enter the number of rows:");
let num_rows: usize = read_user_input();

let mut rows = Vec::with_capacity(num_rows);

for _ in 0..num_rows {
    println!("Enter the number of cells in the row:");
    let num_cells: usize = read_user_input();

    let mut cells = Vec::with_capacity(num_cells);

    for _ in 0..num_cells {
        println!("Enter the cell height:");
        let cell_height: u32 = read_user_input();

        println!("Enter the cell width:");
        let cell_width: u32 = read_user_input();

        println!("Enter the cell value:");
        let cell_value: u32 = read_user_input();

        cells.push(Cell::data_assign(cell_height, cell_width, cell_value));
    }

    let row = Row::row_data(cells);
    rows.push(row);
}

let mut table = Table::table_data(rows);
println!("{:#?}", table);


// user input as value -----------------------------------------------------------------------------------------------------


    // User input
    println!("Do you want to update any values? If yes, enter 'y' or 'n'.");
    let yn: String = read_user_input();

    if yn == "y" {
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

        // Updating the specified cell with user input
        table.rows[row_index].cells[cell_index] =
            Cell::data_assign(cell_height, cell_width, cell_value);

        table.rows[row_index].update_row_data();
        table.update_table_data();

        // Printing the updated table
        println!("{:#?}", table);
    } else {
        println!("Thank you for your input");
    }
}
