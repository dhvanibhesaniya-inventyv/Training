// dependency  
// rand = "0.8.5"

use std::{cmp::Ordering, io};
//  The Rng trait defines methods that random number generators implement,
use rand::Rng;

fn main() {
    println!(" guess the number! ");

    //The gen_range method takes a range expression as an argument and generates
    // a random number in the range. The kind of range expression we’re using here takes
    // the form start..=end and is inclusive on the lower and upper bounds, so we need to
    //specify 1..=100 to request a number between 1 and 100.
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("the secret number is : {secret_number}");

    // loop for mmultiple guess
    loop {
        println!(" please input your guess! ");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess) // here guess value is string
            .expect("failed to read line");

        // convert string to intiger

        //The trim method on a String instance will eliminate any whitespace at the beginning and end,
        // let guess: u32 = guess.trim().parse().expect("please type a number");
       
       
       // Handling Invalid Input
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,  // The underscore, _, is a catchall value; in this example, we’re saying we want to match all Err values, no matter what information they have inside them. So the program will execute the second arm’s code,
                                    //continue, which tells the program to go to the next iteration of the loop and ask for another guess. So, effectively, the program ignores all errors that parse might encounter!
        };

        println!("your guessed is  {guess} ");

        // comparing the guess to the secret number

        // a type called std::cmp::Ordering into scope from the standard library.
        // The Ordering type is another enum and has the variants Less, Greater, and Equal.
        // These are the three outcomes that are possible when you compare two values.

        // The cmp method compares two values and can be called on anything that can be compared.
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("too small"),
            Ordering::Greater => println!("too big"),
            Ordering::Equal => {
                println!("you win");
                break;
            }
        }
    }
}
