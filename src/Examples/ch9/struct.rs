pub struct Guess {
    value : i32
}

impl Guess {
    pub fn new(value : i32) -> Guess { // Constructor
        if value <1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess { value }
    }

    pub fn value(&self) -> i32 { // Get function
        self.value
    }
}