use std::{thread, time::Duration};
// fn simulated_expensive_calculation(intensity: u32) -> u32 {
//     println!("calculating slowly ...");
//     thread::sleep(Duration::from_secs(2));
//     return intensity;
// }

pub struct Cacher<T, U>
where
    T: Fn(U) -> U,
    U: std::cmp::Eq + std::hash::Hash + Copy,
{
    calculation: T,
    value: Option<U>,
}

impl<T, U> Cacher<T, U>
where
    T: Fn(U) -> U,
    U: std::cmp::Eq + std::hash::Hash + Copy,
{
    pub fn new(calculation: T) -> Cacher<T, U> {
        Cacher {
            calculation,
            value: None,
        }
    }

    pub fn value(&mut self, argc: U) -> U {
        match self.value {
            Some(value) => value,
            None => {
                let v = (self.calculation)(argc);
                self.value = Some(v);
                return v;
            }
        }
    }
}

pub fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_closure = Cacher::new(|_num: u32| -> u32 {
        println!("calculating slowly ...");
        thread::sleep(Duration::from_secs(1));
        return intensity;
    });
    // let expensive_result = simulated_expensive_calculation(intensity);
    if intensity < 25 {
        // let ez_to_cause_code_duplication = expensive_closure(intensity);
        println!("Today, do {} pushups!", expensive_closure.value(intensity));
        println!("Next, do {} situps", expensive_closure.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes",
                expensive_closure.value(intensity)
            );
        }
    }
}

pub struct Counter {
    count: u32,
}

impl Counter {
    pub fn constructor() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            return Some(self.count);
        }
        None
    }
}

///# Examples
/// ```
/// let mut workout = 5;
/// workout = closure::add_one(workout);
/// assert_eq!(workout, 6);
/// ```
///
pub fn add_one(x: i32) -> i32 {
    x + 1
}
