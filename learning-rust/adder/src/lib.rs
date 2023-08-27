//!# adder
//!
//! `adder` is a collection of utulities to make performing certain 
//! calculation bla bla bla


#[derive(Debug)]
struct Rectangle {
    w: u32,
    h: u32,
}

#[derive(Debug)]
pub struct Guess {
    pub value: u32,
}


impl Guess {
    /// new guess number
    /// create a guess number that is between 1 and g100
    /// # Example usage
    /// ```
    /// use adder::Guess;
    /// let guess = Guess::new(3);
    /// ```
    pub fn new(value: u32) ->Self{
        if value < 1{
            panic!("Guess value must be greater than or equal to 1, got {}", value);
        }
        else if value > 100{
            panic!("Guess value must be less than or equal to 100, got {}", value);
        }
        Self{value}
    }
}

impl Rectangle {
    fn can_hold(&self, other:&Rectangle) ->bool{
        self.h >= other.h  && self.w >= other.w
    }
    fn rec(h: u32, w: u32) ->Self{
        Self{h,w}
    }
}

fn greeting(name: &str) ->String{
    format!("Hello {}", name)
}
//this is just so we avoid warning! 
pub fn no_warn(){
    let s = Rectangle::rec(1, 8);
    let l = Rectangle::rec(1, 8);
    let _g = Guess::new(1);
    let _can= s.can_hold(&l);

}
pub fn prints_and_returns_10(a: i32) ->i32 {
    println!("fuck you!!!!!!!!!!! number print {}",a );
    10
}

/// Adds 2 numbers to the number given.
/// 
/// # Examples
///
/// ```
/// use adder::add;
/// let arg1 = 5;
/// let arg2 = 5;
/// let answer = add(arg1, arg2);
///
/// assert_eq!(10, answer);
/// ````
/// `
pub fn add(left: usize, right: usize) -> usize {
    left + right
}


#[cfg(test)]
mod tests {
    use super::*;

    //this is useful apparently to use the `?` operation
    //to make a fail test with unexpected result
    #[test]
    fn it_works() ->Result<(), String> {
        if add(2, 2) == 4{
            Ok(())
        }
        else {
            Err(String::from("2 + 2 doesn not equal 4"))
        }
    }

    #[test]
    fn it_add_two() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn when_larger_should_hold_smaller(){
        let larger = Rectangle {
            w: 8,
            h: 7,
        };
        let smaller = Rectangle {
            w: 5,
            h: 1,
        };
        assert!(larger.can_hold(&smaller));

    }
    #[test]
    fn when_smaller_should_not_hold_larger(){
        let smaller = Rectangle::rec(1,1);
        let larger = Rectangle::rec(2,2);
        assert!(!smaller.can_hold(&larger));

    }

    #[test]
    fn when_smaller_w_should_not_hold_larger(){
        let smaller = Rectangle::rec(5,1);
        let larger = Rectangle::rec(2,2);
        assert!(!smaller.can_hold(&larger));

    }
    #[test]
    fn when_same_size_should_hold(){
        let smaller = Rectangle::rec(1,1);
        let larger = Rectangle::rec(1,1);
        assert!(smaller.can_hold(&larger));

    }
    #[test]
    fn when_greeting_should_countain_name(){
        let r = greeting("toto");
        assert!(r.contains("toto"),
        "Greeting did not contain name, value was {}",
        r
        );
    }

    #[test]
    #[should_panic(expected = "less than or equal to 100")]
    fn greater_than_100(){
        Guess::new(200);
    }

    #[test]
    #[should_panic(expected = "greater than or equal to 1")]
    fn lesser_than_1(){
        Guess::new(0);
    }

    #[test]
    fn testing_print_pass() {
        let v = prints_and_returns_10(7);
        assert_eq!(10, v);
    }
    #[test]
    #[ignore = "failing test test"]
    fn testing_print_fail() {
        let v = prints_and_returns_10(7);
        assert_eq!(8, v);
    }
}


