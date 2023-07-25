// use closure::generate_workout;

// fn main() {
//     let simulated_user_specified_value = 10;
//     let simulated_random_number = 7;

//     generate_workout(simulated_user_specified_value, simulated_random_number);
// }
fn do_twice<F>(mut func: F)
//? 他吃的是一個閉包
where
    F: FnMut(),
{
    func();
    func();
}

fn main() {
    let mut x: usize = 1;
    let y: usize = 2;
    {
        let add_two_to_x = || x += y;
        do_twice(add_two_to_x);
    }
    println!("{}", x)
}
// fn hello_something(something: &str) -> String {
//     let mut s = String::from("Hello ");
//     s.push_str(something);
//     s.push_str("!");
//     s
// }

// fn main() {
//     let h = hello_something;
//     println!("{}", h("world"));
// }

// fn main() {
//     fn hello_something(something: &str) -> String {
//         let mut s = String::from("Hello ");
//         s.push_str(something);
//         s.push_str("!");
//         s
//     }
//     let h: fn(&str) -> String = hello_something;
//     println!("{}", h("world"));
// }
