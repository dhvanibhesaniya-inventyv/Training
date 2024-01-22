

// Ownership Rules

// First, let’s take a look at the ownership rules. Keep these rules in mind as we work through the examples that illustrate them:

// Each value in Rust has an owner.
// There can only be one owner at a time.
// When the owner goes out of scope, the value will be dropped.



// Here are some of the types that implement Copy:

// All the integer types, such as u32.
// The Boolean type, bool, with values true and false.
// All the floating-point types, such as f64.
// The character type, char.
// Tuples, if they only contain types that also implement Copy. For example, (i32, i32) implements Copy, but (i32, String) does not.


// The Rules of References
// Let’s recap what we’ve discussed about references:

// At any given time, you can have either one mutable reference or any number of immutable references.
// References must always be valid.

fn main() {
    // let x = 5;
    // let y = x;

    // println!("x = {}, y = {}", x, y);

    // let mut s = String::from("hello");

    // s.push_str(", world!"); // push_str() appends a literal to a String

    // println!("{}", s); // This will print `hello, world!`





    //-------------------------------------------------------------------------------------
    
    
    
    // return type using a tuple
    
    // let mut s1 = String::from("hello");
    
    // let len = calculate_length(&mut s1);
    
    // println!("The string values are : Name: '{}', length: is {}, pointer refrence is {:?}, capacity is {}.", s1, len,s1.as_ptr(),s1.capacity());
    
    

    //-------------------------------------------------------------------------------------


// let mut s  = String::from("hello");

// let r1 = &mut s;
// println!("{}", r1);
// let r2 = &mut s;
// println!("{}", r2);
// let r3 = &s;


// let mut s  = String::from("hello");

// let r1 = & s;
// let r2 = &s;
// println!("{}, {}", r1,r2);
// let r3 = &s;



//-------------------------------------------------------------------------------------------------
//     slice


// let mut s = String::from("hello world");

// let word = first_word(&s); // word will get the value 5
// println!("{}",word);

// s.clear(); // this empties the String, making it equal to ""

// word still has the value 5 here, but there's no more string that
// we could meaningfully use the value 5 with. word is now totally invalid!


//-------------------------------------------------------------------------------------------------
//     String Slices



// let s = String::from("hello world");

// let hello = &s[0..5];
// let world = &s[6..11];

// println!("{}",hello);
// println!("{}",world);

// let s2: &str = "hello"; // string litereal is in string slice 
// // Because string literals *are* string slices already,
// let len = s2.len();
// println!("{}",len);  //5


// let slice1 = &s2[3..len];
// println!("{}",slice1); //lo

// let slice2 = &s2[3..];
// println!("{}",slice2); // lo

// let slice3 = &s[0..len];
// println!("{}",slice3); //hello
// let slice4 = &s[..];
// println!("{}",slice4); // hello world



let mut s = String::from("hello world");

let word = second_word(&s); // word will get the value 5
println!("{}",word);

s.clear(); // this empties the String, making it equal to ""


// word still has the value 5 here, but there's no more string that
// we could meaningfully use the value 5 with. word is now totally invalid!







    // let my_string = String::from("hello world");

    // // `first_word` works on slices of `String`s, whether partial or whole
    // let word = second_word(&my_string[0..6]);
    // let word = second_word(&my_string[..]);
    // // `first_word` also works on references to `String`s, which are equivalent
    // // to whole slices of `String`s
    // let word = second_word(&my_string);

    // let my_string_literal = "hello world";

    // // `first_word` works on slices of string literals, whether partial or whole
    // let word = second_word(&my_string_literal[0..6]);
    // let word = second_word(&my_string_literal[..]);

    // // Because string literals *are* string slices already,
    // // this works too, without the slice syntax!
    // let word = second_word(my_string_literal);





    // let mut my_string = String::from("Hello");
    // my_string.push_str(", World!");
    // println!("{}", my_string);

    // // &str - Immutable view into a string
    // let string_slice: &str = &my_string;
    // println!("{}", &string_slice[..3]);

    // // String literals - &'static str
    // let static_str: &'static str = "Hello, World!";
    // println!("{}", static_str);




    //-----------------------------------------------------------------------------------
    //  string slice array

//     let a = [1, 2, 3, 4, 5];

// let slice = &a[1..3];

// assert_eq!(slice, &[2, 3]);




}







// fn calculate_length(s: &mut String) -> usize {
//     println!("The string values are : Name: '{}', length: is {}, pointer refrence is {:?}, capacity is {}.", s, s.len(),s.as_ptr(),s.capacity());
//     s.push_str(" dhvani");
//     s.len() // len() returns the length of a String
// }


// fn first_word(s: &String) -> usize {
//     let bytes = s.as_bytes();
//
//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b'o'  {
//             // println!("{:?}",item);
//
//             return i;
//         }
//         // println!("{:?} {:?}",i,item);
//     }
//
//
//     s.len()
// }

fn second_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            // println!("{:?}",item);  // 32
            return &s[0..i];
        }
        //  println!("{:?} {:?}",i,item); // {(0,104),(1,101),(2,108),(3,108),(4,111)}
    }

    &s[..]
}



