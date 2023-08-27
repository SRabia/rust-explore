use std::cmp::Ordering;
use examples::*;
use aggregator::{Summary, Tweet};

use crate::aggregator::NewArticle;
mod examples;
mod aggregator;

// fn largest(list:[u32]) ->u32{
//     let mut largest = list[0];
//     for item in  list{
//         if(largest < item){
//             largest = item;
//         }
//         
//     }
//     largest
// }
//
fn _largest_i32(list: &[i32]) ->&i32{
    let mut largest = &list[0];
    for item in list{
        if largest < item{
            largest = item;
        }
    }
    largest
}

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
   fn x(&self) ->&T{
        &self.x
    }
    fn y(&self) ->&T{
        &self.y
    }
}

impl Point<f32> {
    fn distance(&self) ->f32{
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }

}

fn largest<T: std::cmp::PartialOrd>(list:&[T]) -> &T{
    let mut largest = &list[0];
    for item in list{
        if largest < item{
            largest = item;
        }
    }
    largest
}

fn function_call(s: &mut str)
{
    s.make_ascii_uppercase();

}


fn main() {
    // from example show how other module work
    if_expression();
    //example generic struct
    let integer = Point {x: 5, y: 10};
    let floatpoint = Point {x: 5.0, y: 1.0};

    println!("point {:?}", integer);
    println!("point x{}", integer.x());
    println!("point y{}", integer.y());
    println!("distance {}", floatpoint.distance());

    // example generic function 
    let t = [8, 1,2,3,4,9];
    let slice = &t[0..3];
    println!("largest {}", largest(slice));

    //example trait

    let tweet = Tweet {
        username: String::from("horse"),
        content: String::from("hello"),
        reply: false,
        retweet: false,
    };
    let article = NewArticle {
        headline: String::from("Penguins with "),
        location: String::from("Penguins"),
        author: String::from("Iceburgh"),
        content: String::from("content"),
    };
    
    println!("1 new tweet: {}", tweet.summarize());
    println!("New article available {}", article.summarize());

    let string1 = String::from("longest string is long");
    let mut res: String = String::from("hello");
    function_call(res.as_mut_str());

    let arr_2 = [1,2,3,4, 5,6,6];
    let mut loop_idx =0;
    while loop_idx< arr_2[loo] {
        
    }
    

}



