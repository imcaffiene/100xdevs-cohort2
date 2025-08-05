// use std::env;

// use chrono::{Local, Utc};
// use dotenv::dotenv;

// fn main() {
//     //utc time zone
//     let utc_now = Utc::now();
//     println!("Utc time : {}", utc_now);

//     //local timezone
//     let local_time = Local::now();
//     println!("Local : {}", local_time);

//     dotenv().ok();

//     let var = env::var("password");

//     match var {
//         Ok(str) => println!("{}", str),
//         Err(e) => println!("{}", e),
//     };

//     let var2 = env::var("userid").unwrap();

//     println!("{}", var2)
// }

// ---------------------------------- generics and traits-----------------------------

// fn main() {
//     println!("Max of 5 and 3 : {}", max(5, 3)); // i32

//     println!("Max of 5.5 and 3.2 : {}", max(5.5, 3.3)); // f64

//     println!("Max of 'z' and 's' : {}", max('z', 's')); // char
// }

// fn max<T: PartialOrd>(a: T, b: T) -> T {
//     if a > b { a } else { b }
// }

// ------------------------ genric function -------------------------

use std::fmt::Display;

fn compare_and_print<T>(a: T, b: T) -> T
where
    T: PartialOrd + Display,
{
    if a > b {
        println!("{} is greater than {}", a, b);
        a // ← Return 'a' (the larger value)
    } else {
        println!("{} is greater than {}", a, b);
        b // ← Return 'b' (the larger value)
    }
}

fn main() {
    let max_num = compare_and_print(5, 12);
    println!("Result : {}", max_num)
}
