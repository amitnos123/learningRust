pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T : Messenger> {
    messenger: &'a T,
    value : usize,
    max : usize
}

impl<'a, T> LimitTracker<'a, T>
where
    T : Messenger,
{
    pub fn new(messenger : &T, max : usize) -> LimitTracker<T> {
        LimitTracker { messenger, value: 0, max }
    }

    pub fn set_value(&mut self, value : usize) {
        self.value = value

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger.send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger.send("Warning: You've used up over 75% of your quota!");
        }
    }
}

fn main() {
    // Because RefCell<T> allows mutable borrows checked at runtime,
    // you can mutate the value inside the RefCell<T> even when
    // the RefCell<T> is immutable.
    
    let a = 5
    let b = &mut a; // Cannot mutate immutable variable
    
    let mut c = 10;
    let d = &c;
    *d = 20;
    // Cannot assign to `*d`, which is behind a `&` reference
    // `d` is a `&` reference, so the data it refers to cannot be written
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct MockMessenger {
        sent_messages : RefCell<Vec<String>>
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger { sent_messages: RefCell::new(vec![]) }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message : &str) {
            self.sent_messages.borrow_mut().push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messager = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_message, 100);

        limit_tracker.set_value(80);

        assert_eq!(mock_messager.sent_messages.borrow().len(), 1);
    }
}