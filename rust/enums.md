# Rust Enums: Complete Guide with Examples

Enums in Rust let you define a type by enumerating its possible **variants**.  
A value of an enum can be **only one** of its variants at any given time, making enums ideal for data that has a well-defined, finite set of possibilities.

---

## 1. Basic Enum Definition

```rust
enum EnumName {
Variant1,
Variant2,
Variant3,
}
```

### Example – Directions

```rust
#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn main() {
    let my_direction = Direction::Up;
    println!("{:?}", my_direction);

    // Using match with enum
    match my_direction {
        Direction::Up => println!("Going up!"),
        Direction::Down => println!("Going down!"),
        Direction::Left => println!("Going left!"),
        Direction::Right => println!("Going right!"),
    }
}

```

```bash
Output:
Up
Going up!
```

### IP Address Example

```rust
#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    println!("{:?}", four);
    println!("{:?}", six);

    route(IpAddrKind::V4);
    route(IpAddrKind::V6);
}

fn route(ip_kind: IpAddrKind) {
    match ip_kind {
        IpAddrKind::V4 => println!("Routing IPv4 address"),
        IpAddrKind::V6 => println!("Routing IPv6 address"),
    }
}
```

```bash
Output:

V4
V6
Routing IPv4 address
Routing IPv6 address
```

---

## 2. Enums with Data

Rust allows each enum variant to _carry data_.  
You can attach different kinds and amounts of information to each variant.

```rust
#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    let msg1 = Message::Quit;
    let msg2 = Message::Move { x: 10, y: 20 };
    let msg3 = Message::Write(String::from("Hello, Rust!"));
    let msg4 = Message::ChangeColor(255, 0, 128);

    process_message(msg1);
    process_message(msg2);
    process_message(msg3);
    process_message(msg4);
}

fn process_message(msg: Message) {
    match msg {
        Message::Quit => println!("Quit message received"),
        Message::Move { x, y } => println!("Move to coordinates ({}, {})", x, y),
        Message::Write(text) => println!("Text message: {}", text),
        Message::ChangeColor(r, g, b) => println!("Change color to RGB({}, {}, {})", r, g, b),
    }
}


Output:

Quit message received
Move to coordinates (10, 20)
Text message: Hello, Rust!
Change color to RGB(255, 0, 128)
```

---

## 3.1 Real-World Example – IP Addresses

```rust
#[derive(Debug)]
enum IpAddr {
V4(u8, u8, u8, u8),
V6(String),
}

fn route(ip: IpAddr) {
match ip {
IpAddr::V4(a, b, c, d) =>
println!("Routing IPv4: {}.{}.{}.{}", a, b, c, d),
IpAddr::V6(addr) =>
println!("Routing IPv6: {}", addr),
}
}

fn main() {
let home = IpAddr::V4(127, 0, 0, 1);
let loopback = IpAddr::V6(String::from("::1"));


route(home);
route(loopback);
}

```

### 3.2Practical Example: Login Status

```rust
#[derive(Debug)]
enum LoginStatus {
    Success(String),
    Error(String),
}

fn main() {
    let result1 = LoginStatus::Success(String::from("Welcome, John!"));
    let result2 = LoginStatus::Error(String::from("Incorrect password"));

    handle_login(result1);
    handle_login(result2);
}

fn handle_login(status: LoginStatus) {
    match status {
        LoginStatus::Success(message) => {
            println!("✅ Success: {}", message);
        },
        LoginStatus::Error(message) => {
            println!("❌ Error: {}", message);
        }
    }
}

Output:
✅ Success: Welcome, John!
❌ Error: Incorrect password
```

---

## 4. The `Option<T>` Enum

- The `Option<T>` enum is one of the most important enums in Rust. It represents a value that can either be something `(Some(T)) or nothing (None)`, replacing the concept of null values.

- `Option<T>` replaces the idea of “null” with an explicit type:

```rust
fn find_number(numbers: Vec<i32>, target: i32) -> Option<usize> {
    for (index, &number) in numbers.iter().enumerate() {
        if number == target {
            return Some(index);
        }
    }
    None
}

fn main() {
    let numbers = vec![10, 20, 30, 40, 50];

    let result1 = find_number(numbers.clone(), 30);
    let result2 = find_number(numbers, 99);

    match result1 {
        Some(index) => println!("Found at index: {}", index),
        None => println!("Not found"),
    }

    match result2 {
        Some(index) => println!("Found at index: {}", index),
        None => println!("Not found"),
    }

    // Using if let for cleaner code
    if let Some(index) = find_number(vec![1, 2, 3], 2) {
        println!("Found element at index: {}", index);
    }
}


Output:

Found at index: 2
Not found
Found element at index: 1

```

### Complex Enum Example: Command System

```rust
#[derive(Debug)]
enum Command {
    Play,
    Stop,
    Skip(u32),        // Skip to timestamp
    Back(u32),        // Go back to timestamp
    Resize {          // Struct-like variant
        width: u32,
        height: u32,
    },
    Volume(f32),      // Volume level 0.0 to 1.0
}

fn execute_command(cmd: Command) {
    match cmd {
        Command::Play => {
            println!("▶️  Playing media");
        },
        Command::Stop => {
            println!("⏹️  Stopping media");
        },
        Command::Skip(timestamp) => {
            println!("⏭️  Skipping to {} seconds", timestamp);
        },
        Command::Back(timestamp) => {
            println!("⏮️  Going back to {} seconds", timestamp);
        },
        Command::Resize { width, height } => {
            println!("🔄 Resizing to {}x{}", width, height);
        },
        Command::Volume(level) => {
            println!("🔊 Setting volume to {:.1}%", level * 100.0);
        }
    }
}

fn main() {
    let commands = vec![
        Command::Play,
        Command::Volume(0.8),
        Command::Skip(120),
        Command::Resize { width: 1920, height: 1080 },
        Command::Back(60),
        Command::Stop,
    ];

    for cmd in commands {
        execute_command(cmd);
    }
}


Output:

▶️  Playing media
🔊 Setting volume to 80.0%
⏭️  Skipping to 120 seconds
🔄 Resizing to 1920x1080
⏮️  Going back to 60 seconds
⏹️  Stopping media

```

---

## 5. The `Result<T, E>` Enum for Error Handling

```rust
// Simplified form of std::result::Result
enum Result<T, E> {
Ok(T),
Err(E),
}

fn parse*number(s: &str) -> Result<i32, &'static str> {
match s.parse::<i32>() {
Ok(n) => Ok(n),
Err(*) => Err("Not a valid integer"),
}
}

fn main() {
match parse_number("42") {
Ok(n) => println!("Got {}", n),
Err(e) => println!("Error: {}", e),
}
}
```

---

## 6. Implementing Methods on Enums

```rust
#[derive(Debug)]
enum Shape {
Rectangle { width: f64, height: f64 },
Circle { radius: f64 },
Triangle { base: f64, height: f64 },
}

impl Shape {
fn area(&self) -> f64 {
match self {
Shape::Rectangle { width, height } => width _ height,
Shape::Circle { radius } => std::f64::consts::PI _ radius _ radius,
Shape::Triangle { base, height } => 0.5 _ base \* height,
}
}


fn describe(&self) -> String {
match self {
Shape::Rectangle { width, height } =>
format!("Rectangle {} × {}", width, height),
Shape::Circle { radius } =>
format!("Circle radius {}", radius),
Shape::Triangle { base, height } =>
format!("Triangle base {}, height {}", base, height),
}
}
}

fn main() {
let shapes = vec![
Shape::Rectangle { width: 10.0, height: 5.0 },
Shape::Circle { radius: 3.0 },
Shape::Triangle { base: 4.0, height: 6.0 },
];

for s in shapes {
println!("{} – area {:.2}", s.describe(), s.area());
}
}

```

---

## 7. Key Benefits of Rust Enums

- **Type-safe** exhaustive handling via `match`.
- Variants can **carry data** of differing types and sizes.
- Enable the **Option** and **Result** patterns that remove runtime null and unchecked exceptions.
- **Memory-efficient**: requires storage for only the largest variant plus a discriminant.

---

## 8. Handy Patterns

```rust
// Using if let
if let Some(value) = maybe_value {
println!("Got {}", value);
}

// Match with guard
match number {
Some(n) if n > 0 => println!("Positive {}", n),
Some(n) if n < 0 => println!("Negative {}", n),
Some(0) => println!("Zero"),
None => println!("No number"),
}

```

---
