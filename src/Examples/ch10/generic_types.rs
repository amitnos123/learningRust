struct Point<T> {
    x : T,
    y : T
}

impl<U> Point<U> {
    fn x(&self) -> &U {
        &self.x
    }
}

// This only applies for Point with type f64
impl Point<f64> {
    fn y(&self) -> f64 {
        self.y
    }
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let largest = get_largest(number_list);

    println!("The largest number is {}", largest);

    // let char_list = vec!['y', 'm', 'a', 'q'];

    // let largest = get_largest(number_list);

    println!("The largest number is {}", largest);
}

fn get_largest<T: PartialOrd + Copy>(number_list : Vec<T>) -> T {
    let mut largest = number_list[0];
    for number in number_list {
        if number > largest {
            largest = number;
        }
    }
    largest
}