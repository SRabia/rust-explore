#![allow(unused)]// no warning for unused variable
use std::fmt::{Display, format};


 pub trait Summary {
    fn summarize_author(&self) -> String;
    fn summarize(&self) -> String {
        format!("(Read more from {} ...)", self.summarize_author())
    }
}

pub struct NewArticle {
    pub headline: String,
    pub location: String, 
    pub author: String, 
    pub content: String, 
}

pub struct Tweet{
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

//use default impl of Summary 
impl Summary for NewArticle {
    fn summarize_author(&self) -> String {
        format!("{}", self.author)
    }
    
}

pub fn notify(item: &impl Summary){
    println!("Breaking news! {}", item.summarize());
}


//same as above but more verbose
//instead of notify(item1: &impl Summary, item2: &impl Summary)
//using the generic allow for simplified syntax
//but connstrains the function to same type
pub fn notify_2<T: Summary>(item1: &T, item2: &T){
    println!("Breaking news1! {}", item1.summarize());
    println!("Breaking news2! {}", item2.summarize());
}

//this means that item has to implement both Display and summary
pub fn notify_display(item: &(impl Summary + Display)){
    println!("Breaking news1! {}", item.summarize());
    println!("display {}", item);//implementation of display here
}


// frankinstein function that take many trait implementation
fn some_function_multi_trait<T, U>(t: &T, u: &U) ->i32
where
    T:Display + Clone,// T should implement Display and Clone
    U: Clone + Summary,// U should implement Clone and Summary
{
    return  32;

}

fn returns_summarizable(username:String, content:String) -> impl Summary {
    Tweet{
        username,
        content,
        reply: false,
        retweet: false,
    }
}

// impl Summary for NewArticle {
//     fn summarize(&self) -> String {
//         format!("{}, by {} ({})", self.headline, self.author, self.location)
//     }
// }

// don't know how this work...
 // impl Display for Tweet {
 //    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
 //        format!("{}:{}, reply: {}, retweet: {}",
 //            self.username, self.content, self.reply, self.retweet)
 //    }
 //     
 // }

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

