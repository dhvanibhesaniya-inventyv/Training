use std::io;
#[derive(Debug)]
struct Cell{
    height:u32,
    width:u32,
    value:u32,
}

impl Cell{

fn data_assign (height: u32,width: u32,value: u32) -> Cell{
    Cell {
        height,
        width,
        value
    }
}

}

#[derive(Debug)]

struct Row{
    r_height:u32,
    r_width:u32,
    cells: Vec<Cell>,
    total_cells:u32,
}


impl Row{

    fn row_data(cells: Vec<Cell>) -> Row{
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

    fn update_row_data(&mut self) {
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

struct Table{
    rows:Vec<Row> ,  
    t_height:u32,
    t_width: u32,
    t_row: u32,
    t_cell:u32,
}

impl Table{


    fn table_data(rows:Vec<Row>) -> Table{
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

    fn update_table_data(&mut self) {
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


fn read_user_input<T>() -> T
where
    T: std::str::FromStr,
    T::Err: std::fmt::Debug,
{
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().parse().expect("Failed to parse input")
}


fn main(){

// // static data -----------------------------------------------------------------------------------------

    let cell1 = Cell::data_assign(1,2,1);
    let cell2 = Cell::data_assign(2,2,2);


    let row1 = Row::row_data(vec!(cell1,cell2));
    let cell3 = Cell::data_assign(3,2,3);
    let cell4 = Cell::data_assign(4,2,4);
    let row2 = Row::row_data(vec!(cell3,cell4));
    let mut table = Table::table_data(vec!(row1,row2));
    println!("{:#?}",table);

    println!("");
    println!("");

    table.rows[1].cells[1] = Cell::data_assign(10,10, 10);

    table.rows[1].update_row_data();

    table.update_table_data();
    

    println!("{:#?}", table);

    //  // static data ------------------------------------------------------------------------------------

    
// // user input as value -----------------------------------------------------------------------------------------------------

// println!("Enter the number of rows:");
// let num_rows: usize = read_user_input();

// let mut rows = Vec::with_capacity(num_rows);

// for _ in 0..num_rows {
//     println!("Enter the number of cells in the row:");
//     let num_cells: usize = read_user_input();

//     let mut cells = Vec::with_capacity(num_cells);

//     for _ in 0..num_cells {
//         println!("Enter the cell height:");
//         let cell_height: u32 = read_user_input();

//         println!("Enter the cell width:");
//         let cell_width: u32 = read_user_input();

//         println!("Enter the cell value:");
//         let cell_value: u32 = read_user_input();

//         cells.push(Cell::data_assign(cell_height, cell_width, cell_value));
//     }

//     let row = Row::row_data(cells);
//     rows.push(row);
// }

// let mut table = Table::table_data(rows);
// println!("{:#?}", table);


// // user input as value -----------------------------------------------------------------------------------------------------




    // user input for updation --------------------------------------------------------------------------------------------

//     println!("do you want to update any values ?  if yes then enter  'y'  or  'n' .");
//     let yn: String = read_user_input(); 
 
// if yn == "y"{
        
//     println!("Enter the row number to update:");
//     let row_index: usize = read_user_input();
    
//     if row_index >= table.rows.len() {
//         println!("Invalid row index");
//         return;
//     }

//     println!("Enter the cell number to update:");
//     let cell_index: usize = read_user_input();

//     if cell_index >= table.rows[row_index].cells.len() {
//         println!("Invalid cell index");
//         return;
//     }

//     println!("Enter the cell height:");
//     let cell_height: u32 = read_user_input();

//     println!("Enter the cell width:");
//     let cell_width: u32 = read_user_input();

//     println!("Enter the cell value:");
//     let cell_value: u32 = read_user_input();

//     table.rows[row_index].cells[cell_index] = Cell::data_assign(cell_height, cell_width, cell_value);

//     table.rows[row_index].update_row_data();

//     table.update_table_data();

//     println!("{:#?}", table);

// }else{
//     println!("thankyou for youy input");
// }

}









//------------------------------------------------



// use std::cell;
// #[derive(Debug)]
// struct Cell{
//     height:u32,
//     width:u32,
//     value:u32,
// }



// impl Cell{

// fn data_assign (height: u32,width: u32,value: u32) -> Self{
//     Cell {
//         height,
//         width,
//         value
//     }
// }

// }

// #[derive(Debug)]

// struct Row{
//     r_height:u32,
//     r_width:u32,
//     cells: Vec<Cell>,
//     total_cells:u32,
// }


// impl Row{

//     fn row_data(cells: Vec<Cell>) -> Self{
//         let mut r_height: u32 = 0;
//         let mut r_width: u32 = 0;
//         let total_cells: u32 = cells.len() as u32;


//         for i in 0..cells.len(){
//             if r_height <= cells[i].height{
//                 r_height = cells[i].height;
//             }
//             r_width += cells[i].width;
//         }

//         Row { r_height, r_width, cells, total_cells }


//     }

// }

// #[derive(Debug)]

// struct Table{
//     rows:Vec<Row> ,  // Vector of Rows
//     t_height:u32,
//     t_width: u32,
//     t_row: u32,
// }


// impl Table{


//     fn table_data(rows:Vec<Row>) -> Self{
//         let mut t_height: u32=0;
//         let mut t_width: u32 =0;
//         let t_row: u32 = rows.len() as u32 ;

//         for i in 0..rows.len(){
//             if t_width <= rows[i].r_width{
//                 t_width = rows[i].r_width;
//             }
//             t_height+=rows[i].r_height;
//         }

//         Table { rows,t_height,t_width,t_row}
//     }


// }





// fn main(){


//     let cell1 = Cell::data_assign(1,2,1);
//     let cell2 = Cell::data_assign(2,2,2);

//     let row1 = Row::row_data(vec!(cell1,cell2));
//     let cell3 = Cell::data_assign(3,2,3);
//     let cell4 = Cell::data_assign(4,2,4);
//     let row2 = Row::row_data(vec!(cell3,cell4));
//     let table = Table::table_data(vec!(row1,row2));



//     println!("{:#?}",table);

// }