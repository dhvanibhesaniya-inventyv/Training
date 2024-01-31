use rust_book::{notify, NewsArticle, Summary, Tweet};

fn main(){
   let tweet = Tweet{
      username: String::from("horse_ebooks"),
      content: String::from(" hi my name is dhvani"),
      reply: false,
      retweet: false
   };

   println!("1 st new tweet: {}",tweet.summarize());
   


   let article = NewsArticle{
      headline: String::from("i won the cup"),
      location:String::from("gondal,rajkot,india"),
      author:String::from("dhvani"),
      content:String::from(
         "i am the best again \
             at playing badminton.",
   ),
   };


   println!("news article available! : {}",article.summarize());

notify(&article);

}