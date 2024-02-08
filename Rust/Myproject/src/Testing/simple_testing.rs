// #[derive(Debug)]
// struct Rectangle{
//     width:u32,
//     height:u32
// }

// impl Rectangle{
//     fn can_hold(&self, other: &Rectangle) -> bool{
//         self.width > other.width && self.height > other.height
//     }
    
// }

// #[cfg(test)]
// mod test{
//     use super::*;


//     #[test]
//     fn larger_can_hol_smaller(){
//         let larger = Rectangle{
//             width: 8,
//             height: 7,
//         };
//         let smaller = Rectangle {
//             width: 5,
//             height: 1,
//         };


//         assert!(larger.can_hold(&smaller));
//     }

//     #[test]
//     fn smaller_cannot_hold_larger(){
//         let larger = Rectangle{
//             width: 8,
//             height: 7,
//         };
//         let smaller = Rectangle {
//             width: 5,
//             height: 1,
//         };


//         assert!(!smaller.can_hold(&larger));
//     }
// }




pub fn main(){

    add(2,3);   
    greeting("dhvani".to_string());

}

pub fn add(a:i32,b:i32)-> i32{
    a+b
}
pub fn greeting(name:String)-> String{
    
    // format!("dhvanii")
    name
}



#[cfg(test)]

mod unit_testing{
    use super::*;

    #[test]
    fn add_two_number(){
        assert_eq!(4,add(2,2));
    }


    #[test]
    fn adding(){
        assert_eq!(10,add(20,-10));
    } 


    #[test]
    fn greeting_contains_name(){
        let result  = greeting("dhvani".to_string());

        assert!(result.contains("dhvani"),"greetings did not contain name: {}",result);     // "hi" gives error  and custom message will be printed. 
    }
}