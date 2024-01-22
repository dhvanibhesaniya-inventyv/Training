
// use std::io;

/// Main function for demonstrating character replacement in a string.

pub fn main() -> String{
    // // Take a string as input
         // println!("Enter a string:");
         // let mut input_string = String::new();
         // io::stdin().read_line(&mut input_string).expect("Failed to read line");

         let input_string = String::from("welcome to inventyv software services");

    // // Ask for a character to be removed
         // println!("Enter a character to be removed:");
         // let mut remove_char = String::new();
         // io::stdin().read_line(&mut remove_char).expect("Failed to read line");

         let remove_char = String::from("e");

    let remove_char = remove_char.chars().next().expect("No character entered");

    // Replace the character in the string with '_'
    let mut replaced_string = String::new();
    for c in input_string.chars() {
        if c == remove_char {
            replaced_string.push('_');
        } else {
            replaced_string.push(c);
        }
    }

    // Print the updated string with '_'
    // println!("{}", replaced_string);

    replaced_string

    // // Ask for a character to be replaced with '_'
    // println!("Enter a character to be replaced with '_' :");
    // let mut replace_char = String::new();
    // io::stdin().read_line(&mut replace_char).expect("Failed to read line");
    // let replace_char = replace_char.chars().next().expect("No character entered");

    // // Replace the '_' in the string with replace_char
    // let mut new_replaced_string = String::new();
    // for c in replaced_string.chars() {
    //     if c == '_' {
    //         new_replaced_string.push(replace_char);
    //     } else {
    //         new_replaced_string.push(c);
    //     }
    // }

    // // Print the updated string
    // println!("{}", new_replaced_string);
}

