// https://petal-estimate-4e9.notion.site/Rust-for-Solana-contracts-1937dfd1073580c8a8fbc7e135e6d22a

fn main() {
    // let ans: i32 = sum(1, 2);
    // println!("The sum is {}", ans);

    // println!("Is even {}", is_even(1));

    // let name = String::from("Sumit");
    // println!("Hello {}", name);

    // let v = vec![1, 2, 3];
    // println!("The vector is {:?}", v);

    // create_str();

    /*
    let s1 = String::from("Sumit");

    let len = get_length(&s1); //

    println!("The length of the string is {}", len);

    println!("The string is {}", s1);  ---> now this is working

     println!("{}", str) ---> because now the ownership is moved to get_length
     */

    get_fun();

    get_sumit();
}

/*
fn sum(a: i32, b: i32) -> i32 {
    return a + b;
}*/

// boolean
/*
fn is_even(a: i32) -> bool {
    return a % 2 == 0;
}
*/

//--------------- ownership rules ---------------
/*
fn create_str() {
    let mut name = String::from("Sumit");

    //let name2 = name; // ownership rule came here

    name.push_str("Kumar");

    println!("name is {}", name)
}
*/

//--------------- borrowing  ---------------

/*
fn get_length(s2: &String) -> usize {
    return s2.len();
}

*/

fn get_fun() {
    let mut s1 = String::from("Sumit");
    let s2 = &mut s1;
    //let s3 = &s1; // can't mut more than once

    s2.push_str("Kumar");

    println!("{}", s2);
}

// --------------structs ---------------
#[derive(Debug)]
struct Sumit {
    active: bool,
    username: String,
    email: String,
    age: u8,
    sign_in_count: u32,
}

fn get_sumit() {
    let user1 = Sumit {
        active: true,
        username: String::from("imcaffiene"),
        email: String::from("imcaffiene@gmail.com"),
        age: 25,
        sign_in_count: 1,
    };
    println!("{:?}", user1);
}
