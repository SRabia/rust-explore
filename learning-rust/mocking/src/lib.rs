
pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger>{
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl <'a, T> LimitTracker<'a, T> 
where
    T: Messenger,
{
    pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T>{
        LimitTracker { 
            messenger, 
            value: 0, 
            max,
        }
    }

    pub fn set_value(&mut self, value: usize){
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0{
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max > 0.9 {
            self.messenger
            .send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger
            .send("Warning: You've used up over 75% of your quota!");
        }
    }
}

#[cfg(test)]
mod test{
    use super::*;
    use std::{cell::RefCell, vec};

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }
    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger{
                sent_messages: RefCell::new(vec![]),
            }
        }
    }
    impl Messenger for MockMessenger {
        fn send(&self, msg: &str) {
            self.sent_messages
                .borrow_mut()
                .push(String::from(msg));
            // let mut one = self.sent_messages.borrow_mut();
            // let mut two = self.sent_messages.borrow_mut();
            // one.push(String::from("fofo"));
            // two.push(String::from("fofo"));


        }
    }

    #[test]
    fn it_sends_an_over_75_precent_warning_message(){
        let mock_messenger = MockMessenger::new();
        let mut limit = LimitTracker::new(&mock_messenger, 100);

        limit.set_value(80);

        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);

    }
}

