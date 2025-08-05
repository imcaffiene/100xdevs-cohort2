# 🧱 Structs in Rust (with Examples)

Rust's **structs** are a way to create custom data types by grouping related data together. They are fundamental for organizing and structuring your data.

---

## What is a Struct?

A struct is a collection of named fields that can have different types. Structs enable you to model complex data.

### Defining a Struct

```rust
struct User {
username: String,
email: String,
active: bool,
sign_in_count: u64,
}
```

### Creating an Instance

```rust
fn main() {
let user1 = User {
username: String::from("alice"),
email: String::from("alice@example.com"),
active: true,
sign_in_count: 1,
};

println!("Username: {}", user1.username);
}
```

### Mutable Structs

To modify fields, the struct instance must be mutable.

```rust
fn main() {
let mut user1 = User {
username: String::from("alice"),
email: String::from("alice@example.com"),
active: true,
sign_in_count: 1,
};


user1.email = String::from("alice@newdomain.com");
println!("Updated Email: {}", user1.email);
}
```

### Struct Update Syntax

You can create a new instance using fields from another instance.

```rust
let user2 = User {
email: String::from("bob@example.com"),
username: String::from("bob"),
..user1 // copies other fields from user1
};


```

### Tuple Structs

Tuple structs are like tuples but have named types.

```rust

struct Color(u8, u8, u8);
struct Point(i32, i32, i32);

fn main() {
let black = Color(0, 0, 0);
let origin = Point(0, 0, 0);

text
println!("Black color’s red component: {}", black.0);
}
```

### Unit-Like Structs

Structs without fields, often used as markers.

```rust
struct Marker;

fn main() {
let \_m = Marker;
}
```

### Ownership and Structs

Struct fields own their data by default. For example, `String` fields own the string data they hold.

```rust
struct Book {
title: String,
}

fn main() {
let my_book = Book {
title: String::from("Rust in Action"),
};

text
println!("{}", my_book.title);
}
```

### Deriving Traits (Debug, Clone, etc.)

To allow printing structs with `println!("{:?}", ...)` or cloning, you can derive traits:

```rust
#[derive(Debug, Clone)]
struct User {
username: String,
email: String,
}
```

### Implementing Methods on Structs

You can define methods inside `impl` blocks.

```rust
struct Rectangle {
width: u32,
height: u32,
}

impl Rectangle {
fn area(&self) -> u32 {
self.width \* self.height
}

text
fn is_square(&self) -> bool {
self.width == self.height
}
}

fn main() {
let rect = Rectangle { width: 10, height: 20 };

text
println!("Area: {}", rect.area());
println!("Is square? {}", rect.is_square());
}
```

### Associated Functions

These are functions associated with a struct but don’t take `self`.

```rust
impl Rectangle {
fn new(width: u32, height: u32) -> Self {
Self { width, height }
}
}

fn main() {
let square = Rectangle::new(10, 10);
println!("Square area: {}", square.area());
}
```

### Implementing Custom Display

To print your struct with `{}`, implement the `Display` trait:

```rust
use std::fmt;

struct User {
username: String,
email: String,
}

impl fmt::Display for User {
fn fmt(&self, f: &mut fmt::Formatter<'\_>) -> fmt::Result {
write!(f, "User: {}, Email: {}", self.username, self.email)
}
}

fn main() {
let user = User {
username: String::from("alice"),
email: String::from("alice@example.com"),
};
println!("{}", user);
}
```
