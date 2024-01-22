
// fn main(){

// let str1 = "hello world";
//     let str2 = "inventyv software";

//     let mut matched: Vec<char> = Vec::new();
//     let mut unmatched: Vec<char> = Vec::new();
//     let mut result:Vec<char> = Vec::new(); 


//     for c in str1.chars() {
//         if str2.contains(c) && !matched.contains(&c) && c!=' ' {
//             matched.push(c);
//         }
//         else if !matched.contains(&c) && !unmatched.contains(&c) && c!=' ' {
//             unmatched.push(c);
//         }
//     }

//     for c in str2.chars() {
//         if !str1.contains(c) && !unmatched.contains(&c) && c!=' ' {
//             unmatched.push(c);
//         }
//     }

    
//     println!("Matched: {:?}", matched);
//     println!("Unmatched: {:?}", unmatched);
    

//     result.extend(matched);
//     result.extend(unmatched);

//    // Manually sort characters in ascending order using selection sort , as it uses ascii values 
//    let len = result.len();
//    for i in 0..len {
//        let mut min_index = i;
//        for j in i + 1..len {
//            if result[j] < result[min_index] {
//                min_index = j;
//            }
//        }
//        // Swap the characters
//        result.swap(i, min_index);
//    }
//     println!("\nResult: {:?}", result);

// }

//----------------------------------------------------------------------------------------------------------------------------------
// task 2 of two string  on a single string

// 


//use std::io;

// fn main() {
//     // Take a string as input
//     println!("Enter a string:");
//     let mut input_string = String::new();
//     io::stdin().read_line(&mut input_string).expect("Failed to read line");

//     // Ask for a character to be removed
//     println!("Enter a character to be removed:");
//     let mut remove_char = String::new();
//     io::stdin().read_line(&mut remove_char).expect("Failed to read line");
//     let remove_char = remove_char.chars().next().expect("No character entered");

//     // Replace the character in the string with '_'
//     let mut replaced_string = String::new();
//     for c in input_string.chars() {
//         if c == remove_char {
//             replaced_string.push('_');
//         } else {
//             replaced_string.push(c);
//         }
//     }

//     // Print the updated string with '_'
//     println!("{}", replaced_string);

//     // Ask for a character to be replaced with '_'
//     println!("Enter a character to be replaced with '_' :");
//     let mut replace_char = String::new();
//     io::stdin().read_line(&mut replace_char).expect("Failed to read line");
//     let replace_char = replace_char.chars().next().expect("No character entered");

//     // Replace the '_' in the string with replace_char
//     let mut new_replaced_string = String::new();
//     for c in replaced_string.chars() {
//         if c == '_' {
//             new_replaced_string.push(replace_char);
//         } else {
//             new_replaced_string.push(c);
//         }
//     }

//     // Print the updated string
//     println!("{}", new_replaced_string);
// }



// ----------------------------------------------------------------------------------------------------------------------------------

pub fn main() {
    // Function to calculate character frequency in a string
    fn calculate_frequency(s: &str) -> [usize; 26] {
        let mut frequency = [0; 26];
        for ch in s.chars() {
            let ascii_value = ch.to_ascii_lowercase() as u8;
            if ascii_value >= b'a' && ascii_value <= b'z' {
                let index = (ascii_value - b'a') as usize;
                frequency[index] += 1;

                // println!("{}, {}", ch, frequency[index]);
            }
        }
        frequency
    }

    // Get input from the user
    println!("Enter the first string:");
    let mut string1 = String::new();
    io::stdin().read_line(&mut string1).expect("Failed to read line");

    println!("Enter the second string:");
    let mut string2 = String::new();
    io::stdin().read_line(&mut string2).expect("Failed to read line");

    // Calculate frequency of characters in each string
    let frequency1 = calculate_frequency(&string1);
    let frequency2 = calculate_frequency(&string2);

    // Create vectors as per the requirements
    let mut vector1 = Vec::new(); // Vector for common characters with total counts in both strings
    let mut vector2 = Vec::new(); // Vector for characters that are not common
    let mut vector3 = Vec::new(); // Vector for character and frequency at their corresponding positions 

    // Iterate over characters in the alphabet
    for i in 0..26 {
        let ch = (i as u8 + b'a') as char;

        let count1 = frequency1[i];
        let count2 = frequency2[i];

        if count1 > 0 && count2 > 0 {
            vector1.push((ch, count1 + count2));
            vector3.push((ch, count1 + count2));
        } else if count1 > 0 {
            vector2.push((ch, count1));
            vector3.push((ch, count1));
        } else if count2 > 0 {
            vector2.push((ch, count2));
            vector3.push((ch, count2));
        }

       
    }

    // Display the vectors
    println!("Vector 1 (Common characters with total counts in both strings): {:?}", vector1);
    println!("Vector 2 (Characters not common in both strings): {:?}", vector2);
    println!("Vector 3 (Character and frequency at corresponding positions): {:?}", vector3);
}


