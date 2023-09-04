#![allow(unused)]// no warning for unused variable

use std::{cell::RefCell, rc::{Rc, Weak}};
use crate::List::{Cons, Nil};

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn tail(&self)-> Option<&RefCell<Rc<List>>>{
        match self {
            Cons(_, item ) => Some(item),
            Nil => None,
        }
    }
}

/// Thinking about the relationships another way, a parent node should own 
/// its children: if a parent node is dropped, its child nodes should be 
/// dropped as well. However, a child should not own its parent: 
/// if we drop a child node, the parent should still exist. 
/// This is a case for weak references!
#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

fn _memory_cyle_example(){

    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));
    println!("a initial rc count = {}", Rc::strong_count(&a));

    println!("a next item = {:?}", a.tail());

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b  initial rc count ={}", Rc::strong_count(&b));
    println!("b nex item = {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));

    // Uncomment the next line to see that we have a cycle;
    // it will overflow the stack
    println!("a next item = {:?}", a.tail());
}

fn main() {
    let leaf = Rc::new(Node{
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!("leaf strong = {}, weak ={}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );

    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!(
            "branch strong ={}, weak ={}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch),
        );

        println!(
            "leaf strong ={}, weak ={}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );
    }

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    println!(
        "leaf strong ={}, weak ={}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );

    // let branch = Rc::new(Node{
    //     value: 5,
    //     parent: RefCell::new(Weak::new()),
    //     children: RefCell::new(vec![Rc::clone(&leaf)]),
    // });

    // *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    // println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

}

