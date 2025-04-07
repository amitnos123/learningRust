use std::thread;
use std::time::Duration;

fn simulated_expensive_calculation(intensity : u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

struct Cacher<T>
where
    T: Fn(u32) -> u32, // Fn is a trait
{
    calculation : T,
    value : Option<u32>
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation : T) -> Cacher<T> {
        Cacher { calculation, value: None }
    }

    fn value(&mut self, arg : u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

fn main() {
    // Closures are anonymous functions
    // Closures have access to variables in same scope
    // Closures have 3 ways to capture variables from the scope: taking ownership, borrowing mutably, borrowing immutably
    // FnOnce - Takes ownership once on the variables inside the closures environment. This Closures can only be called onces
    // FnMut - mutably borrow values
    // Fn - immutably borrow values
    
    // To create closure of type FnOnce:
    // move |arg| {...}

    let simulated_insensity = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_insensity,simulated_random_number)
}

fn generate_workout(intensity : u32, random_number : u32) {
    let mut cached_result = Cacher::new(|num : u32| -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });
    // let expensive_closure = |num : u32| -> u32 {
    //     println!("calculating slowly...");
    //     thread::sleep(Duration::from_secs(2));
    //     num
    // } // expensive_closure stores the function

    if intensity < 25 {
        println!("Today, do {} pushup!", cached_result.value(intensity));
        println!("Next, do {} situps!", cached_result.value(intensity));
    } else {
        if random_number ==3 {
            println!("Take a break today! remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!", cached_result.value(intensity));
        }
    }
}