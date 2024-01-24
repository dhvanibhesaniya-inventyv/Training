







use std::collections::VecDeque;


fn main(){
//     let mut a = Vec::new();

//     a.push(1);
//     a.push(4);
//     a.push(5);
//     a.push(7);
//     let c = a.pop();
//     println!("{:?}",c);
//     println!("{:?}",a);
//     // Iterating through the vector
// for element in &a {
//     println!("{}", element);
// }


//     let mut a = Vec::new();

//     a.push(1);
//     a.push(4);
//     a.push(5);
//     a.push(7);
//     let c = a.pop();
//     println!("{:?}",c);
//     println!("{:?}",a);
//     // Iterating through the vector
// for element in &a {
//     println!("{}", element);
// }


    // vecDeque ----------------

//     let mut deque: VecDeque<i32> = VecDeque::new();

// deque.push_front(0);
// deque.push_back(1);
// deque.push_back(2);
// deque.push_front(4);

// // Iterating through the vector
// for element in &deque {
//     println!("{}", element);
// }
// println!("_");


// // Pop from the front
// let front_element = deque.pop_front().unwrap();

// // Pop from the back
// let back_element = deque.pop_back().unwrap();


// // Iterating through the vector
// for element in &deque {
//     print!("{}", element);
// }


// concatenation -------------------------------------

// let vec1 = vec![1, 2, 3];
// let vec2 = vec![4, 5, 6];

// let concatenated_vec = [&vec1[..], &vec2[..]].concat();
// println!("{:?}", concatenated_vec);  // [1, 2, 3, 4, 5, 6]
// println!("{:?}", vec1); // [1, 2, 3]
// println!("{:?}", vec2);  // [4, 5, 6]


// extend and append ---------------------------------------

// let mut vec1 = vec![1, 2, 3];
// let vec2 = vec![4, 5, 6];

// // Using extend
// vec1.extend(&vec2);
// println!("{:?}", vec1);   // [1, 2, 3,4,5,6]
// println!("{:?}", vec2);   // [4,5,6]


// let mut vec3 = vec![1, 2, 3];
// let mut vec4 = vec![4, 5, 6];

// // Using append
// vec3.append(&mut vec4);
// println!("{:?}", vec3);   // [1, 2, 3,4,5,6]
// println!("{:?}", vec4);   // []




// iter ------------------------------------------------------------
// Returns an iterator over the slice. The iterator yields all items from start to end.

// let my_vector = vec![1, 2, 3, 4];


// // it iterates over references to the elements in the vector.
// for element in my_vector.iter() {
//     println!("{}", element);
// }
// // this iterate over myvector value
// for element in my_vector {
//     println!("{}", element);
// }

// let my_vector2 = vec!["apple", "banana", "cherry"];

// // The iterator returned yields pairs (i, val), where i is the current index of iteration and val is the value returned by the iterator.
// for (index, value) in my_vector2.iter().enumerate() {
//     println!("Index {}: {}", index, value);
// }




// .get()-------------------------------------------------------------

// let v = vec![1, 2, 3, 4, 5];

// // Attempt to access the element at index 100 using the get method.
// let does_not_exist = v.get(100);

// // Print the result, which is an Option<&T>.
// print!("{:?}", does_not_exist);  // None




// filter -------------------------------------------------------------------

//     let mut numbers = vec![1, 2, 3, 4, 5, 6];

//     // Filter elements greater than 3 and create a new vector
//     let filtered_numbers: Vec<i32> = numbers.iter().filter(|&x| *x > 3).cloned().collect();

//     // Retain elements greater than 3 in-place
//     // the original numbers vector was modified in-place using retain().
// numbers.retain(|&x| x == x);

//     println!("Original numbers: {:?}", numbers);
//     println!("Filtered numbers: {:?}", filtered_numbers);



// extend ----------------------------------------------------------------------------

// let vec1 = vec![1, 2, 3];
// let iter = vec![4, 5, 6].into_iter();

// let mut result_vec = vec1;
// result_vec.extend(iter);




// Creates a consuming iterator, that is, one that moves each value out of the vector (from start to end). The vector cannot be used after calling this.

// Example of .into_iter
// let v = vec!["a".to_string(), "b".to_string()];
// let mut v_iter = v.into_iter();

// let first_element: Option<String> = v_iter.next();

// assert_eq!(first_element, Some("a".to_string()));
// assert_eq!(v_iter.next(), Some("b".to_string()));
// assert_eq!(v_iter.next(), None);



// drain- ------------------------------------------------------

// // Creating a vector
// let mut my_vector = vec![1, 2, 3, 4, 5];

// // Using drain to remove elements from index 1 to 3 (inclusive)
// let removed_elements: Vec<i32> = my_vector.drain(1..=3).collect();

// // Displaying the original and removed elements
// println!("Original Vector: {:?}", my_vector);
// println!("Removed Elements: {:?}", removed_elements);

    
}
















































// #[derive(Debug)]
// enum Spreadsheet {
//     Int(i32),
//     Float(f64),
//     Text(String),
// }



// fn main(){
//     // let mut v = Vec::new();

//     // v.push(6);
//     // v.push(10);

//     // println!("{:?}",v);  // [6, 10]

//     // for i in &v{
//     //     print!("{i} ")   // 6 10
//     // }

//     // println!("{:?}",v); 


//     // let v = vec![1,2,3,4,5];  // index v[0]..v[i]
//     // let thd:&i32 = &v[1];
//     // println!("The 1st element is {:?}",thd);



//     // let thd: Option<&i32> = v.get(1);
//     // match thd {
//     //     Some(thd) => println!("The 1st element is {thd}"),
//     //     None => println!("There is no third element."),
//     // }


//     // let v = vec![1, 2, 3, 4, 5];

//     // // let does_not_exist = &v[100];
//     // let does_not_exist = v.get(100);

//     // print!("{:?}",does_not_exist);    // None


//     // let mut v = vec![1, 2, 3, 4, 5];

    
//     // v.push(6);
//     // let first:&i32 = &v[0];

//     // println!("The first element is: {first}");

//     // let mut  v = vec![100, 32, 57];
//     // for i in &mut v {
//     //     *i += 50;
//     //     // println!("{:?}",*i + 50);
//     // }
//     // println!("{:?}",v);


//     // let row = vec![

//     //     Spreadsheet::Int(3),
//     //     Spreadsheet::Float(9.81),
//     //     Spreadsheet::Text("Hello".to_string()),

//     // ];

//     // for i in &row{
//     //     match i {
//     //         Spreadsheet::Int(value) => println!("Int: {}", value),
//     //         Spreadsheet::Float(value) => println!("Float: {}", value),
//     //         Spreadsheet::Text(value) => println!("Text: {}", value),
//     //     }
//     //     }


//     // let mut s1 = String::from("foo");
//     // let s2 = "bar ";
//     // s1.push_str(s2);
//     // s1.push('!');
//     // println!("s2 is {s1}");
    
//     // let s1 = String::from("Hello, ");
//     // let s2 = String::from("world!");
//     // let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
    
//     // println!("s3 is {s3}");
    
    
    
//     let s1 = String::from("tic");
//     let s2 = String::from("tac");
//     let s3 = String::from("toe");
    
//     // let s = s1 + "-" + &s2 + "-" + &s3;   // println!("s is {s}");
//     let s = format!("{s1}-{s2}-{s3}");    
//     println!("s is {s}");

// }