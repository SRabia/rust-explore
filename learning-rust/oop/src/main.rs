use oop::Draw;
struct SelectBox{
    width: u32,
    height: u32,
    options: Vec<String>,
}


impl Draw for SelectBox {
    fn draw(&self) {
    }
}

use oop::{Button, Screen};

fn main() {
    let screen = Screen{
        components: vec![
            Box::new(SelectBox{
                width: 75,
                height: 10,
                options: vec![
                    String::from("yes"),
                    String::from("maybe"),
                    String::from("no"),
                ],
            }),
            Box::new(Button{
                width: 50,
                height: 10,
                label: String::from("Ok"),
            }),
        ],
    };

    screen.run();
    
    println!("Hello, world!");
}
