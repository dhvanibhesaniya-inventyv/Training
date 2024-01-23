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