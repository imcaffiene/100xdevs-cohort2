fn main() {
    //----------------borrowing and references------

    // let mut s = String::from("sumit");
    // print_length(&s); // ----> Immutable ref
    // println!("Real:{}", s);

    // changes(&mut s); // ---> mut ref
    // println!("modified: {}", s);

    // --------------Struct -------------
    // get_user();

    // -------------- enums -------------
    get_direction();
}

//---------------- borrowing and references------
/*
fn print_length(s: &String) {
    println!("Length:{}", s.len())
}

fn changes(s: &mut String) {
    s.push_str("kumar");
}
*/

// ----- struct --------

/*
struct User {
    username: String,
    email: String,
    active: bool,
    sign_in_count: u64,
}

// instance
fn get_user() {
    let mut user1 = User {
        username: String::from("sumit"),
        email: String::from("smtkur31@gmail.com"),
        active: true,
        sign_in_count: 12,
    };
    println!("Username:{}", user1.username);
    println!("email:{}", user1.email);
    println!("is_active:{}", user1.active);
    println!("xyz:{}", user1.sign_in_count);

    user1.email = String::from("imcaffiene@gmail.com");

    println!("Updated email: {}", user1.email);

    // let user2 = User {
    //     username: String::from("punit"),
    //     active: user1.active,
    //     email: String::from("punit@gmail.com"),
    //     sign_in_count: user1.sign_in_count,
    // };

    // println!("next Username:{}", user2.username);
    // println!(" next email:{}", user2.email);
    // println!("next is_active:{}", user2.active);
    // println!(" next xyz:{}", user2.sign_in_count);
}
*/
