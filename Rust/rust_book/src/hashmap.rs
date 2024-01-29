fn main() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    scores.entry(String::from("green")).or_insert(60);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);


    // for (key, value) in &scores {
    //     println!(" {value}");
//     }


    
//     let field_name = String::from("Favorite color");
//     let field_value = String::from("Blue");

//     let mut map = HashMap::new();
//     map.insert(&field_name, &field_value);
}


// extra knowledge




use std::{collections::HashMap, ops::Add};

fn main(){


    let mut a = String::from("dhvani");

    // println!("{}",&a);
    let b="Hello".to_string();
   a.extend(b.chars());
    // println!("{:?}",c);
    println!("{}",a);
    println!("{}",b);
    
    



     // Creating a HashMap
     let mut my_map = HashMap::new();

     // Inserting initial key-value pairs
     my_map.insert("one", 1);
     my_map.insert("two", 2);
 
     // Creating another HashMap
     let additional_map: HashMap<&str, i32> = vec![("three", 3), ("four", 4)].into_iter().collect();
 
     // Extending the original HashMap using an iterator
     my_map.extend(additional_map);
 
     // Displaying the extended HashMap
     println!("{:?}", my_map);
// c.a



// }



//   // Creating a String
//   let my_string = String::from("Hello, Rust!");

//   // Using the iter method to iterate over characters
//   for character in my_string.iter() {
//       println!("{}", character);
//   }




 // Creating a String
//  let my_string = String::from("Hello, Rust!");

//  // Using the chars method to iterate over owned characters
//  for character in my_string.chars() {
//      println!("{}", character);
//  }



// let my_string = String::from("Hello, Rust!");

// // Using the bytes method to iterate over byte slices
// for byte in my_string.bytes() {
//     println!("{}", byte);
// }
}

// use std::collections::HashMap;

// // fn main() {
//     // // Creating a HashMap
//     // let mut my_map = HashMap::new();

//     // // Inserting key-value pairs
//     // my_map.insert("one", 1);
//     // my_map.insert("two", 2);
//     // my_map.insert("three", 3);


//     // // Using iter and enumerate to iterate over key-value pairs with indices
//     // for (index, (key, value)) in my_map.iter().enumerate() {
//     //     println!("Index: {}, Key: {}, Value: {}", index, key, value);
//     // }



//      // Creating a HashMap
//      let mut my_map = HashMap::new();

//      // Inserting key-value pairs
//      my_map.insert("one", 1);
//      my_map.insert("two", 2);
//      my_map.insert("three", 3);
//      my_map.insert("four", 4);
 
//      // Filtering elements based on a condition
//      let filtered_map: HashMap<_, _> = my_map
//          .iter()
//          .filter(|&(_, value)| *value > 2)
//          .map(|(&key, &value)| (key, value))
//          .collect();
 
//      // Retaining elements based on a condition
//      my_map.retain(|_, value| value > &mut 2);
 
//      // Displaying the filtered map
//      println!("Filtered Map: {:?}", filtered_map);
 
//      // Displaying the retained map
//      println!("Retained Map: {:?}", my_map);






//      //get


//       // Creating a HashMap
//     let mut my_map = HashMap::new();

//     // Inserting key-value pairs
//     my_map.insert("one", 1);
//     // my_map.insert("two", 2);
//     my_map.insert("three", 3);

//     // Using get to retrieve a value by key
//     if let Some(value) = my_map.get("two") {
//         println!("Value for key 'two': {}", value);
//     } else {
//         println!("Key 'two' not found");
//     }
// }








// UTF (Unicode Transformation Format) is a character encoding standard that provides a way to represent a wide range of characters and symbols from various writing systems, languages, and cultures. It aims to be a universal character encoding capable of representing all possible characters defined by the Unicode standard.

// Key points about UTF:

// 1. **Unicode Standard:**
//    - UTF is closely tied to the Unicode standard, which assigns a unique code point (numeric value) to each character, symbol, or emoji.

// 2. **Variable-Length Encoding:**
//    - UTF uses variable-length encoding, allowing it to represent characters using different numbers of bytes.
//    - UTF-8, UTF-16, and UTF-32 are the most common variants of UTF, differing in the number of bytes used to represent characters.

// 3. **UTF-8:**
//    - UTF-8 is the most widely used variant of UTF.
//    - It represents characters using one to four bytes, ensuring compatibility with ASCII (1-byte characters).

// 4. **UTF-16:**
//    - UTF-16 represents characters using one or two 16-bit code units.
//    - It is commonly used in languages with larger character sets, such as CJK (Chinese, Japanese, Korean).

// 5. **UTF-32:**
//    - UTF-32 represents characters using one 32-bit code unit per character.
//    - It provides a fixed-length representation, simplifying certain operations but using more space.

// 6. **Universal Representation:**
//    - UTF is designed to support characters from all languages and scripts, including special symbols, mathematical symbols, and emojis.

// 7. **Interoperability:**
//    - UTF has become the standard for character encoding in modern computing, ensuring interoperability across different systems, platforms, and programming languages.

// 8. **Web and Internationalization:**
//    - UTF is widely used on the internet, supporting the global nature of online content.
//    - It plays a crucial role in internationalization (i18n) and localization (l10n) efforts to make software and content accessible to users worldwide.

// In summary, UTF is a character encoding standard that provides a comprehensive and extensible way to represent characters, symbols, and emojis from diverse languages and writing systems. Its variable-length encoding allows for efficient representation of a vast character repertoire while maintaining compatibility with legacy systems that use fixed-length encodings like ASCII.