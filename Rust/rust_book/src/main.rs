use rust_book::{Summary,Tweet};

fn main(){
   let tweet = Tweet{
      username: String::from("horse_ebooks"),
      content: String::from(" hi my name is dhvani"),
      reply: false,
      retweet: false
   };

   println!("1 st new tweet: {}",tweet.summarize());
   
}