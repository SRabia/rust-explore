#![allow(unused)]// no warning for unused variable

use std::io;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;


pub fn greeting_example()
{
    println!("Hello, fucker!");
    println!("What is your name?");
    let mut name: String = String::new();
    let greeting : &str = "Nice to meet you";

    let result: Result<usize, _> = io::stdin().read_line(&mut name);
    result.expect("Didn't receive input");

    println!("Hello {}! {}", name.trim_end(), greeting);

}

pub fn shadowing(){
    const ONE_MIL:u32 = 1_000_000;
    const PI: f32 = 3.141592;
    let age: &str = "47";
    let mut age: u32 = age.trim().parse()
        .expect("age wasn't assigned a number");// shadowing

    age = age +1;
    println!("I'm {} and I want ${}", age, ONE_MIL);

}

pub fn data_types_examples(){

    println!("Max u32: {}", u32::MAX);
    println!("Max u64: {}", u64::MAX);
    println!("Max f32: {}", f32::MAX);

    let is_true: bool = true;
    let is_false: bool = false;

}

pub fn random_example(){
    let random_num: i32 = rand::thread_rng().gen_range(1..101);
}

pub fn if_expression(){
    let age: i32 = 8;

    if age > 1 && age <= 18{
        println!("important birthday");
    }else if age ==21 || age == 50{
        println!("important Birthday");
    }else {
        println!("not important");
    }

    let mut my_age = 47;

    let can_vote = if my_age >= 18 {true} else {false};

}

pub fn generic_largest<T:std::cmp::PartialOrd>(list: &[T]) -> &T{
    let mut largest = &list[0];

    for element in list{
        if(element > largest){
            largest = element;
        }
    }
    largest
}

