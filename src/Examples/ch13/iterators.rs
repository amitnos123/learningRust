fn main() {
    let v1 = vec![1,2,3];

    // iter() return &T (immutable references)
    // iter_mut() return &mut T (mutable references)
    // into_iter() return yield any of T, &T or &mut T
    let v1_iter = v1.iter();

    for value in v1_iter {
        println!("Got: {}", value);
    }

    let v1_iter = v1.iter();
    let total : i32 = v1_iter.sum();
    println!("Sum: {}", total);

    let v2 : Vec<_> = v1.iter().map(|x| x+1).collect();

    for value in v2.iter() {
        println!("Got: {}", value);
    }
}


// How to implement my own iterator

struct  Counter {
    count : u32
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

#[test]
fn calling_next_directly() {
    let mut counter = Counter::new();
    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), None);
}