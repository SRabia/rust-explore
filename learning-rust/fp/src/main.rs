use std::{vec, ops::Sub};

#[derive(Debug, PartialEq, Copy, Clone)]
enum ColorShirt {
    Red,
    Blue,
}

#[derive(Debug)]
struct Inventory {
    shirts: Vec<ColorShirt>,
}


impl Inventory {
    fn giveaway(&self, user_pref: Option<ColorShirt>) -> ColorShirt{
        user_pref.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) ->ColorShirt{

        let mut red_count =0;
        let mut blue_count =0;
        // blue_count = self.shirts.iter().fold(0, |acc, x|if x == ColorShirt::Blue {acc+=1});
        for color in &self.shirts{
            match color {
                ColorShirt::Blue => blue_count +=1,
                ColorShirt::Red => red_count +=1,
            }
        }

        if red_count > blue_count{
            ColorShirt::Red
        }
        else {
            ColorShirt::Blue
        }
    }
    
}

fn inventory_example(){

    let store = Inventory{
        shirts: vec![ColorShirt::Blue, ColorShirt::Red, ColorShirt::Blue],
    };

    let user_pref1 = Some(ColorShirt::Red);
    let giveaway1 = store.giveaway(user_pref1);

    println!("
the user with preference {:?} get {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!("
the user with preference {:?} get {:?}",
        user_pref2, giveaway2
    );
}

fn _closure_testing_out(){
    //example of explicit closure declaration
    // let expensive_closure = |num:u32| -> u32 {
    //     println!("calculating slowly...");
    //     thread::sleep(Duration::from_secs(2));
    //     num
    // };
    // let add_one_v2 = |x:u32| ->u32 {x +1};
    let add_one_v3 = |x| x+1;
    println!("closure {}", add_one_v3(3));
    println!("closure {}", add_one_v3(7));

    let list = vec![1,2,3];
    println!("before defining closure: {:?}", list);

    let only_borrows = || println!("from closure: {:?}", list);

    println!("Before calling closure: {:?}", list);
    only_borrows();
    println!("After calling closure: {:?}", list);

    println!("closure borrows mutable");

}

fn _closure_testing(){
    
    // let v = String::from("jejeo");
    // let mut op = vec![];

    // let mut list = vec![9,2,8];
    // list.sort_by_key(|r| {
    //     println!("h");
    //     op.push(v);
    //      r +1
    // });
    // println!("{:?}", list);
    // println!("{:?}", op);
}

trait Differential<T> {
    fn differential(&self) -> Vec<T>;
}

impl <T> Differential<T> for Vec<T> 
where
T: Sub<Output = T> + Copy,
{
    fn differential(&self) -> Vec<T> {
        let w = self.windows(2);
        w.map(|x| x[1] - x[0]).collect()
    }
    
}

fn differential<T>(v: &[T]) -> Vec<T>
where
T: Sub<Output = T> + Copy
{
    let w = v.windows(2);
    w.map(|x| x[1] - x[0]).collect()

}



fn filet(shoes:Vec<u32>, size: u32) -> Vec<u32> {
    shoes
        .into_iter()
        .filter(|&x| x > size)
        .collect()
}

fn main() {
    inventory_example();
    let v = vec![1,2,3,5];
    let filter = filet(v, 3);
    println!("{:?}", filter);

    // let total: i32 = iter.sum();


    // let v:Vec<i32> = iter.map(|x| {
    //     println!("hello");
    //     return x + 1;
    // }).collect();

    println!("{:?}", differential(&filter));
}
