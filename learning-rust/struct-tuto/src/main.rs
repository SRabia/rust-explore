use std::vec;

// struct User {
//     active: bool,
//     username: String,
//     email: String,
//     sign_in_count: u64,
// }
#[derive(Debug)]
struct Rectangle{
    width: u32,
    height: u32,
}

// struct Color(i32, i32, i32);
// struct Point(i32, i32, i32);
// struct FPoint(f32, f32, f32);


impl Rectangle {
    fn area(&self) ->u32{
        self.height * self.width
    }
}

impl Rectangle {
    fn square(size: u32) -> Self{
        Self {
            width: size, 
            height: size,
        }
    }
}


enum IpAddrKind {
    v4,
    v6,
}


fn main() {
    let s: Rectangle = Rectangle::square(39);
    dbg!("{}", s);
    let config_max = Some(4u8);

    if let Some(max) = config_max{
        println!("the maxim is configured to be {}", max);
    }

    let some_number = Some(5);

    let n:Option<i32> = None;

    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter,
    }

    fn value_in_cents(coin: Coin) -> u8{
        match coin {
            Coin::Dime => {
                println!("this is a dime");
                10
            }
            Coin::Quarter => 25,
            _ => 0,
        }
    }

    value_in_cents(Coin::Penny);
    // let user1 = User{
    //     active: true,
    //     username: String::from("someuser"),
    //     email: String::from("someuser@example.com"),
    //     sign_in_count:1,
    // };
    // 
    // let user2 = User{
    //     email: String::from("another"),
    //     ..user1
    // };
    // 
    // println!("{}", user1.active);
    // println!("{}", user2.username);
    // let black = Color(0,0,0);
    // println!("{}", black.0);

    // println!("{}", black.1);

    let rect = Rectangle { width: 49, height: 50 };

    println!("recta: {:#?}", rect);
    println!("area {}", rect.area());

    let scale =2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect1);
}
