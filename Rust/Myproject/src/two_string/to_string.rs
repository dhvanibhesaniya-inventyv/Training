use std::io;

pub fn two_string_main() {
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
    io::stdin()
        .read_line(&mut string1)
        .expect("Failed to read line");

    println!("Enter the second string:");
    let mut string2 = String::new();
    io::stdin()
        .read_line(&mut string2)
        .expect("Failed to read line");

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
}