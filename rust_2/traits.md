# Traits in Rust

- Traits define shared behavior that types can implement. They're similar to interfaces in other languages but more powerful, enabling everything from basic method definitions to advanced type system features like operator overloading and generic programming.

## 1. What are Traits?

- Traits define a set of methods that types can implement. They enable:

  - `Code reuse`: Multiple types can share the same behavior

  - `Abstraction`: Work with different types through a common interface

  - `Polymorphism`: Different implementations of the same trait

  - `Generic constraints`: Require types to have specific capabilities

```rust
// Basic trait definition
trait Greet {
    fn greet(&self) -> String;
}

// Implementing the trait for different types
struct Person {
    name: String,
}

struct Robot {
    id: u32,
}

impl Greet for Person {
    fn greet(&self) -> String {
        format!("Hello, I'm {}", self.name)
    }
}

impl Greet for Robot {
    fn greet(&self) -> String {
        format!("BEEP BOOP. Robot {} online.", self.id)
    }
}

fn main() {
    let person = Person { name: "Alice".to_string() };
    let robot = Robot { id: 42 };

    println!("{}", person.greet());
    println!("{}", robot.greet());
}

output:
Hello, I'm Alice
BEEP BOOP. Robot 42 online.

```

## 2. Trait Methods

### 2.1 Required vs Default Methods

```rust
trait Animal {
    // Required method - must be implemented
    fn name(&self) -> &str;
    fn species(&self) -> &str;

    // Default implementation - can be overridden
    fn make_sound(&self) -> String {
        format!("{} makes a generic animal sound", self.name())
    }

    // Default method using other trait methods
    fn introduce(&self) -> String {
        format!("Hi, I'm {}, a {}. {}",
                self.name(),
                self.species(),
                self.make_sound())
    }
}

struct Dog {
    name: String,
}

struct Cat {
    name: String,
}

impl Animal for Dog {
    fn name(&self) -> &str {
        &self.name
    }

    fn species(&self) -> &str {
        "dog"
    }

    // Override the default implementation
    fn make_sound(&self) -> String {
        format!("{} barks: Woof!", self.name())
    }
}

impl Animal for Cat {
    fn name(&self) -> &str {
        &self.name
    }

    fn species(&self) -> &str {
        "cat"
    }

    // Override the default implementation
    fn make_sound(&self) -> String {
        format!("{} meows: Meow!", self.name())
    }
}

fn main() {
    let dog = Dog { name: "Buddy".to_string() };
    let cat = Cat { name: "Whiskers".to_string() };

    println!("{}", dog.introduce());
    println!("{}", cat.introduce());
}
output:
Hi, I'm Buddy, a dog. Buddy barks: Woof!
Hi, I'm Whiskers, a cat. Whiskers meows: Meow!

```

## 2.2 Associated Functions

```rust
trait Constructible {
    // Associated function (like static method)
    fn create() -> Self;
    fn create_with_value(value: i32) -> Self;

    // Instance method
    fn get_value(&self) -> i32;
}

struct Counter {
    value: i32,
}

impl Constructible for Counter {
    fn create() -> Self {
        Counter { value: 0 }
    }

    fn create_with_value(value: i32) -> Self {
        Counter { value }
    }

    fn get_value(&self) -> i32 {
        self.value
    }
}

fn main() {
    let counter1 = Counter::create();
    let counter2 = Counter::create_with_value(42);

    println!("Counter 1 value: {}", counter1.get_value());
    println!("Counter 2 value: {}", counter2.get_value());
}



output:
Counter 1 value: 0
Counter 2 value: 42

```

## 3. Generic Traits

```rust
trait Container<T> {
    fn add(&mut self, item: T);
    fn get(&self, index: usize) -> Option<&T>;
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

struct MyVec<T> {
    items: Vec<T>,
}

impl<T> MyVec<T> {
    fn new() -> Self {
        MyVec { items: Vec::new() }
    }
}

impl<T> Container<T> for MyVec<T> {
    fn add(&mut self, item: T) {
        self.items.push(item);
    }

    fn get(&self, index: usize) -> Option<&T> {
        self.items.get(index)
    }

    fn len(&self) -> usize {
        self.items.len()
    }
}

// Generic function using the trait
fn fill_container<T, C>(container: &mut C, items: Vec<T>)
where
    C: Container<T>,
{
    for item in items {
        container.add(item);
    }
}

fn main() {
    let mut vec = MyVec::new();
    fill_container(&mut vec, vec![1, 2, 3, 4, 5]);

    println!("Container length: {}", vec.len());
    println!("Is empty: {}", vec.is_empty());

    for i in 0..vec.len() {
        if let Some(item) = vec.get(i) {
            println!("Item {}: {}", i, item);
        }
    }
}


output:
Container length: 5
Is empty: false
Item 0: 1
Item 1: 2
Item 2: 3
Item 3: 4
Item 4: 5

```

## 4. Associated Types

- Associated types make traits cleaner when there's a logical relationship between the trait and a specific type.

```rust
trait Iterator {
    type Item;  // Associated type

    fn next(&mut self) -> Option<Self::Item>;
}

struct NumberRange {
    current: i32,
    end: i32,
}

impl NumberRange {
    fn new(start: i32, end: i32) -> Self {
        NumberRange { current: start, end }
    }
}

impl Iterator for NumberRange {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current < self.end {
            let current = self.current;
            self.current += 1;
            Some(current)
        } else {
            None
        }
    }
}

// Trait with multiple associated types
trait Graph {
    type Node;
    type Edge;

    fn add_node(&mut self, node: Self::Node);
    fn add_edge(&mut self, edge: Self::Edge);
    fn node_count(&self) -> usize;
}

struct SimpleGraph {
    nodes: Vec<String>,
    edges: Vec<(usize, usize)>,
}

impl Graph for SimpleGraph {
    type Node = String;
    type Edge = (usize, usize);

    fn add_node(&mut self, node: Self::Node) {
        self.nodes.push(node);
    }

    fn add_edge(&mut self, edge: Self::Edge) {
        self.edges.push(edge);
    }

    fn node_count(&self) -> usize {
        self.nodes.len()
    }
}

fn main() {
    // Using custom iterator
    let mut range = NumberRange::new(1, 5);
    print!("Numbers: ");
    while let Some(num) = range.next() {
        print!("{} ", num);
    }
    println!();

    // Using graph
    let mut graph = SimpleGraph {
        nodes: Vec::new(),
        edges: Vec::new(),
    };

    graph.add_node("A".to_string());
    graph.add_node("B".to_string());
    graph.add_node("C".to_string());
    graph.add_edge((0, 1));
    graph.add_edge((1, 2));

    println!("Graph has {} nodes", graph.node_count());
}

output:
Numbers: 1 2 3 4
Graph has 3 nodes
```

## 5. Trait Objects and Dynamic Dispatch

### 5.1 Basic Trait Objects

```rust
trait Draw {
    fn draw(&self);
}

struct Circle {
    radius: f64,
}

struct Rectangle {
    width: f64,
    height: f64,
}

struct Triangle {
    base: f64,
    height: f64,
}

impl Draw for Circle {
    fn draw(&self) {
        println!("Drawing a circle with radius {}", self.radius);
    }
}

impl Draw for Rectangle {
    fn draw(&self) {
        println!("Drawing a rectangle {}x{}", self.width, self.height);
    }
}

impl Draw for Triangle {
    fn draw(&self) {
        println!("Drawing a triangle with base {} and height {}", self.base, self.height);
    }
}

// Function that accepts any drawable object
fn draw_shape(shape: &dyn Draw) {
    shape.draw();
}

// Function that draws multiple shapes
fn draw_all(shapes: &[Box<dyn Draw>]) {
    for shape in shapes {
        shape.draw();
    }
}

fn main() {
    let circle = Circle { radius: 5.0 };
    let rectangle = Rectangle { width: 10.0, height: 20.0 };
    let triangle = Triangle { base: 8.0, height: 12.0 };

    // Using trait objects with references
    draw_shape(&circle);
    draw_shape(&rectangle);
    draw_shape(&triangle);

    // Using trait objects in collections
    let shapes: Vec<Box<dyn Draw>> = vec![
        Box::new(Circle { radius: 3.0 }),
        Box::new(Rectangle { width: 5.0, height: 7.0 }),
        Box::new(Triangle { base: 4.0, height: 6.0 }),
    ];

    println!("\nDrawing all shapes:");
    draw_all(&shapes);
}

output:
Drawing a circle with radius 5
Drawing a rectangle 10x20
Drawing a triangle with base 8 and height 12

Drawing all shapes:
Drawing a circle with radius 3
Drawing a rectangle 5x7
Drawing a triangle with base 4 and height 6
```

## 5.2 Object-Safe Traits

```rust
// Object-safe trait (can be used as trait object)
trait Printable {
    fn print(&self);
    fn name(&self) -> &str;
}

// Not object-safe (has generic method)
trait NotObjectSafe {
    fn process<T>(&self, item: T) -> T;  // Generic method
}

struct Document {
    title: String,
    content: String,
}

struct Image {
    filename: String,
    width: u32,
    height: u32,
}

impl Printable for Document {
    fn print(&self) {
        println!("Printing document: {}", self.title);
        println!("Content: {}", self.content);
    }

    fn name(&self) -> &str {
        &self.title
    }
}

impl Printable for Image {
    fn print(&self) {
        println!("Printing image: {} ({}x{})", self.filename, self.width, self.height);
    }

    fn name(&self) -> &str {
        &self.filename
    }
}

// Printer that works with any printable item
struct Printer {
    items: Vec<Box<dyn Printable>>,
}

impl Printer {
    fn new() -> Self {
        Printer { items: Vec::new() }
    }

    fn add_item(&mut self, item: Box<dyn Printable>) {
        self.items.push(item);
    }

    fn print_all(&self) {
        println!("=== Print Queue ===");
        for (i, item) in self.items.iter().enumerate() {
            println!("Item {}: {}", i + 1, item.name());
            item.print();
            println!();
        }
    }
}

fn main() {
    let mut printer = Printer::new();

    printer.add_item(Box::new(Document {
        title: "Report".to_string(),
        content: "This is a sample report.".to_string(),
    }));

    printer.add_item(Box::new(Image {
        filename: "photo.jpg".to_string(),
        width: 1920,
        height: 1080,
    }));

    printer.print_all();
}

output:
=== Print Queue ===
Item 1: Report
Printing document: Report
Content: This is a sample report.

Item 2: photo.jpg
Printing image: photo.jpg (1920x1080)


```

## 6. Operator Overloading with Traits

```rust
use std::ops::{Add, Sub, Mul, Display};
use std::fmt;

#[derive(Debug, Clone, Copy)]
struct Vector2D {
    x: f64,
    y: f64,
}

impl Vector2D {
    fn new(x: f64, y: f64) -> Self {
        Vector2D { x, y }
    }

    fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
}

// Implement addition
impl Add for Vector2D {
    type Output = Vector2D;

    fn add(self, other: Vector2D) -> Vector2D {
        Vector2D {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

// Implement subtraction
impl Sub for Vector2D {
    type Output = Vector2D;

    fn sub(self, other: Vector2D) -> Vector2D {
        Vector2D {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

// Implement scalar multiplication
impl Mul<f64> for Vector2D {
    type Output = Vector2D;

    fn mul(self, scalar: f64) -> Vector2D {
        Vector2D {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
}

// Implement Display for pretty printing
impl Display for Vector2D {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({:.2}, {:.2})", self.x, self.y)
    }
}

// Implement equality comparison
impl PartialEq for Vector2D {
    fn eq(&self, other: &Self) -> bool {
        (self.x - other.x).abs() < f64::EPSILON &&
        (self.y - other.y).abs() < f64::EPSILON
    }
}

fn main() {
    let v1 = Vector2D::new(3.0, 4.0);
    let v2 = Vector2D::new(1.0, 2.0);

    println!("v1 = {}", v1);
    println!("v2 = {}", v2);
    println!("v1 magnitude = {:.2}", v1.magnitude());

    // Using overloaded operators
    let v3 = v1 + v2;
    let v4 = v1 - v2;
    let v5 = v1 * 2.0;

    println!("v1 + v2 = {}", v3);
    println!("v1 - v2 = {}", v4);
    println!("v1 * 2.0 = {}", v5);

    // Using equality
    let v6 = Vector2D::new(3.0, 4.0);
    println!("v1 == v6: {}", v1 == v6);
    println!("v1 == v2: {}", v1 == v2);
}

output:
v1 = (3.00, 4.00)
v2 = (1.00, 2.00)
v1 magnitude = 5.00
v1 + v2 = (4.00, 6.00)
v1 - v2 = (2.00, 2.00)
v1 * 2.0 = (6.00, 8.00)
v1 == v6: true
v1 == v2: false
```
