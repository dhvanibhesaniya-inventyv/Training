// generic type


// struct Point<T,U>{
//     x: T,
//     y: U,
    
// }
// impl<T,U> Point<T,U>{
//     fn  mixup<V,W>(self,other:Point<V,W>) -> Point<T,W>{
//         Point{
//         x:self.x,
//         y:other.y,
//     }
// }
// }

// fn main(){


//     let p1 = Point{x:5,y:6.0};
//     let p2 = Point { x:"hello",y:'c' };
//     let p3  = p1.mixup(p2);
//     println!("x: {:?}, y: {:?}",p3.x,p3.y);


// }



// fn largest_i32(list: &[i32]) -> i32 {
//     let mut largest = &list[0];

//     for item in list {
//         if item > largest {
//             largest = item;
//         }
//     }

//     largest
// }

// fn largest_char(list: &[char]) -> &char {
//     let mut largest = &list[0];

//     for item in list {
//         if item > largest {
//             largest = item;
//         }
//     }

//     largest
// }

// fn main() {
//     let number_list = vec![34, 50, 25, 100, 65];

//     let mut largest = number_list[0];

//     // for item in number_list {
//     //     if item > largest {
//     //         largest = item;
//     //     }
//     // }
//     let result_1 = largest_i32(&number_list);
//     println!("The largest number is {}", result_1);

//     let char_list = vec!['y', 'm', 'a', 'q'];
    
//     let result = largest_char(&char_list);
//     println!("The largest char is {:?}", result);
// }



fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
}