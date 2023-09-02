use std::ops::Deref;

#[derive(Debug)]
struct MyBox<T> (T);

impl<T> MyBox<T> {
    fn new(x:T) ->MyBox<T>{
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T>{
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug)]
struct MyShit<R> {
    toto: R,
    _roro: i32,
}

impl<R> MyShit<R> {
    fn new(t:R) ->Self{
        Self {toto: t, _roro: 0}
    }
}
impl<R> Deref for MyShit<R> {
    type Target = R;
    fn deref(&self) -> &Self::Target {
        &self.toto
    }
    
}
enum ListRc {
    Cons(i32, Rc<ListRc>),
    Nil,
}
use crate::ListRc::{Cons,Nil};
use std::rc::Rc;


fn _testing_list(){
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c {}", Rc::strong_count(&a));
    }
    println!("count after creating c goes out of scope {}", Rc::strong_count(&a));

}

fn hello(n: &str){
    println!("hello {n}");
}


fn main() {

    let n = MyBox::new(2);
    let s = MyBox::new(String::from("hh"));
    hello(&s);// without the impl of trait Deref this won't work!!

    let re = &n;
    // assert_eq!(2, *n); // why do I get an error here but when I assign re I don't
    println!("{:?}", *re);
    let shit = MyShit::new(1);

    // let re = &shit;// why this work ???
    // println!("ref {:?}", *re);
    println!("ref {:?}", *shit);
    

}

