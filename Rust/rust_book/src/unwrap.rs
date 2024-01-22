
// urwrap


// The adult has seen it all, and can handle any drink well.
// All drinks are handled explicitly using `match`.
// fn give_adult(drink: Option<&str>) {
//     // Specify a course of action for each case.
//     match drink {
//         Some("lemonade") => println!("Yuck! Too sugary."),
//         Some(inner)   => println!("{}? How nice.", inner),
//         None          => println!("No drink? Oh well."),
//     }
// }
//
// Others will `panic` before drinking sugary drinks.
// All drinks are handled implicitly using `unwrap`.


fn drink(drink: Option<&str>) {
    // `unwrap` returns a `panic` when it receives a `None`.
    let inside = drink.unwrap();

    // if option valye is none it returns default
    let inside2 = drink.unwrap_or("default"); 

    // it returns empty if it is valye is none
    let inside3 = drink.unwrap_or_default();

    // it is same as unwrap_or return else value if none
    let inside4 = drink.unwrap_or_else(|| "dhvani"); 

    let inside5 = unsafe{drink.unwrap_unchecked()};

    // Using unwrap_unchecked assumes that drink is Some, and it 
    // extracts the inner value without any runtime check. This is 
    // safe in this case, and the output is 
    // "Unwrapped value (unchecked): coffee". However, using this 
    // method without ensuring that the option is Some can lead to 
    // undefined behavior.



    println!("{}",inside);
    println!("{}",inside2);
    println!("{}",inside3);
    println!("{}",inside4);
    println!("{}",inside5);

    // if inside == "coffee." { panic!("AAAaaaaa!!!!"); }

    // println!("I love {}s!!!!!", inside);
}

fn main() {



    // let water  = Some("water");
    // let lemonade = Some("lemonade");
    // let void  = None;
//
    // give_adult(water);
    // give_adult(lemonade);
    // give_adult(void);

    let coffee = Some("coffee");
    let nothing = None;

    drink(coffee);
    drink(nothing);



}