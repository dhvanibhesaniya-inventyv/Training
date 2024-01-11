use std::fmt;
use std::io;

#[derive(Debug, Clone)]
struct Cell {
    height: f32,
    width: f32,
    value: f32,
}

impl Cell {
    fn new(height: f32, width: f32, value: f32) -> Self {
        Cell { height, width, value }
    }
}

#[derive(Debug, Clone)]
struct Row {
    cells: Vec<Cell>,
    height: f32,
    width: f32,
}

impl Row {
    fn new(cells: Vec<Cell>) -> Self {
        let height = cells.iter().map(|cell| cell.height).fold(0.0, f32::max); // Maximum height of all cells
        let width = cells.iter().map(|cell| cell.width).sum(); // Sum of cell widths as row width
        Row { cells, height, width }
    }
}

#[derive(Debug)]
struct Table {
    rows: Vec<Row>,
    height: f32,
    width: f32,
}

impl Table {
    fn new(rows: Vec<Row>) -> Self {
        let height = rows.iter().map(|row| row.height).sum(); // Sum of row heights as table height
        let width = rows[0].width; // Assuming all rows in a table have the same width
        Table { rows, height, width }
    }
}

impl fmt::Display for Table {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.rows {
            for cell in &row.cells {
                write!(f, "{:.2} ", cell.value)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn main() {
    // Get user input for the number of rows
    let num_rows = get_user_input("Enter the number of rows: ").round() as usize;

    // Get user input for the number of cells per row
    let num_cells = get_user_input("Enter the number of cells per row: ").round() as usize;

    // Create rows with user-specified properties
    let mut rows = Vec::new();

    for _ in 0..num_rows {
        let mut cells = Vec::new();

        for _ in 0..num_cells {
            let height = get_user_input("Enter the height for the cell: ");
            let width = get_user_input("Enter the width for the cell: ");
            let value = get_user_input("Enter the value for the cell: ");

            cells.push(Cell::new(height, width, value));
        }

        rows.push(Row::new(cells));
    }

    // Create the table with the rows
    let table = Table::new(rows);

    // Print the table data
    println!("{:?}", table);
}

fn get_user_input(prompt: &str) -> f32 {
    loop {
        println!("{}", prompt);

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        match input.trim().parse() {
            Ok(value) => return value,
            Err(_) => println!("Invalid input. Please enter a valid number."),
        }
    }
}
