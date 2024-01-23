use crate::two_string::single_string::main as single_string;
// use std::io;
/// Main function for demonstrating operations on two strings.
pub fn two_string_main() {
    /// Function to calculate character frequency in a string.
    ///
    /// # Arguments
    ///
    /// * `s` - The input string for which character frequency needs to be calculated.
    ///
    /// # Returns
    ///
    /// An array representing the frequency of each character in the input string.
    fn calculate_frequency(s: &str) -> [usize; 26] {
        let mut frequency = [0; 26];
        for ch in s.chars() {
            let ascii_value = ch.to_ascii_lowercase() as u8;
            if ascii_value >= b'a' && ascii_value <= b'z' {
                let index = (ascii_value - b'a') as usize;
                frequency[index] += 1;
            }
        }
        frequency
    }

    // // Get input from the user
    // println!("Enter the first string:");
    // let mut string1 = String::new();
    // io::stdin()
    //     .read_line(&mut string1)
    //     .expect("Failed to read line");

    let string1 = String::from("hello world");

    // println!("Enter the second string:");
    // let mut string2 = String::new();
    // io::stdin()
    //     .read_line(&mut string2)
    //     .expect("Failed to read line");

    let string2 = String::from("hello from inventyv");

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
    println!(
        "Vector 1 (Common characters with total counts in both strings): {:?}",
        vector1
    );
    println!(
        "Vector 2 (Characters not common in both strings): {:?}",
        vector2
    );
    println!(
        "Vector 3 (Character and frequency at corresponding positions): {:?}",
        vector3
    );

    // println!("{}",single_string());

    let new_single_string = single_string();

    // this

    let new_replaced_single_string = replace_underscore(new_single_string, &mut vector3);

    // or this

    // for i in new_replaced_single_string.chars() {
    //         if i == '_' {
    //             if let Some((ch, count)) = vector3.first_mut() {
    //                 print!("{}", *ch);
        
    //                 if *count > 0 {
    //                     *count -= 1;
    //                 }
    //                 if *count == 0 {
    //                     vector3.remove(0);
    //                 }
    //             } else {
    //                 print!("_");
    //             }
    //         }else{
    //             print!("{}", i);
    //         }
    //     }

    println!("{:?}", new_replaced_single_string);

    println!(
        "Vector 3 (Character and frequency at corresponding positions): {:?}",
        vector3
    );
}

/// this function replace the underscore with the vector 3 values and changing its frequency.

fn replace_underscore(mut replaced_string: String, vector3: &mut Vec<(char, usize)>) -> String {
    let mut vector_index = 0;

    for (i, ch) in replaced_string.to_string().chars().enumerate() {
        if ch == '_' {
            while let Some((replacement, frequency)) = vector3.get_mut(vector_index) {
                if *frequency > 0 {
                    replaced_string.replace_range(i..i + 1, &replacement.to_string());
                    *frequency -= 1;

                    if *frequency == 0 {
                        vector3.remove(vector_index);
                    }
                    break;
                } else {
                    vector_index += 1;
                }
            }
        }
    }

    replaced_string

}
