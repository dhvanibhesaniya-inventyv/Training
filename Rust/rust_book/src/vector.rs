


#[derive(Debug)]
enum Spreadsheet {
    Int(i32),
    Float(f64),
    Text(String),
}



fn main(){
    // let mut v = Vec::new();

    // v.push(6);
    // v.push(10);

    // println!("{:?}",v);  // [6, 10]

    // for i in &v{
    //     print!("{i} ")   // 6 10
    // }

    // println!("{:?}",v); 


    // let v = vec![1,2,3,4,5];  // index v[0]..v[i]
    // let thd:&i32 = &v[1];
    // println!("The 1st element is {:?}",thd);



    // let thd: Option<&i32> = v.get(1);
    // match thd {
    //     Some(thd) => println!("The 1st element is {thd}"),
    //     None => println!("There is no third element."),
    // }


    // let v = vec![1, 2, 3, 4, 5];

    // // let does_not_exist = &v[100];
    // let does_not_exist = v.get(100);

    // print!("{:?}",does_not_exist);    // None


    // let mut v = vec![1, 2, 3, 4, 5];

    
    // v.push(6);
    // let first:&i32 = &v[0];

    // println!("The first element is: {first}");

    // let mut  v = vec![100, 32, 57];
    // for i in &mut v {
    //     *i += 50;
    //     // println!("{:?}",*i + 50);
    // }
    // println!("{:?}",v);


    // let row = vec![

    //     Spreadsheet::Int(3),
    //     Spreadsheet::Float(9.81),
    //     Spreadsheet::Text("Hello".to_string()),

    // ];

    // for i in &row{
    //     match i {
    //         Spreadsheet::Int(value) => println!("Int: {}", value),
    //         Spreadsheet::Float(value) => println!("Float: {}", value),
    //         Spreadsheet::Text(value) => println!("Text: {}", value),
    //     }
    //     }


    // let mut s1 = String::from("foo");
    // let s2 = "bar ";
    // s1.push_str(s2);
    // s1.push('!');
    // println!("s2 is {s1}");
    
    // let s1 = String::from("Hello, ");
    // let s2 = String::from("world!");
    // let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
    
    // println!("s3 is {s3}");
    
    
    
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    
    // let s = s1 + "-" + &s2 + "-" + &s3;   // println!("s is {s}");
    let s = format!("{s1}-{s2}-{s3}");    
    println!("s is {s}");

}