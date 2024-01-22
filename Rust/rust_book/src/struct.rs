// structure

// struct User {
//     active: bool,
//     username: String,
//     email: String,
//     sign_in_count: u64,
// }

struct Color(i32, i32);
struct Point(i32, i32);

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    //     let mut user1 = User {
    //         active: true,
    //         username: String::from("someusername123"),
    //         email: String::from("someone@example.com"),
    //         sign_in_count: 1,
    //     };

    //     let user2 = User {

    //         email: String::from("another@example.com"),
    //    ..user1
    //     };

    // ----------------------------------------------------------------------

    // tuple struct

    let black = Color(10, 20);
    let origin = Point(0, 0);

    println!("{:?}", black.0);

    println!("{:?}", area(black.0, black.1));

    let rect1 = (30, 50);

    println!("The area of the rectangle is {:?} square pixels.",area2(rect1));

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect1 is {:?}", rect1);

    println!(
        "The area of the rectangle is {} square pixels.",
        area3(&rect1)
    );

    let scale = 2;
    let rect2 = Rectangle {
        width: dbg!(30 * scale),
        ..rect1
    };

    dbg!(&rect2);



}

fn area(x: i32, y: i32) -> i32 {
    x + y
}
fn area2(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}
fn area3(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}