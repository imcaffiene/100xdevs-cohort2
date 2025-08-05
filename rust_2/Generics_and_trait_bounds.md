# Generics and Trait Bounds in Rust

- Generics allow you to write flexible, reusable code by enabling functions, structs, enums, and traits to work with multiple types. Trait bounds constrain these generic types to ensure they have specific capabilities, providing both flexibility and type safety.

## 1. Introduction to Generics

### 1.1 Why Use Generics?

- Generics solve the problem of code duplication when you need similar functionality for different types:

```rust
// Without generics - code duplication
fn max_i32(a: i32, b: i32) -> i32 {
    if a > b { a } else { b }
}

fn max_f64(a: f64, b: f64) -> f64 {
    if a > b { a } else { b }
}

// With generics - single implementation
fn max<T: PartialOrd>(a: T, b: T) -> T {
    if a > b { a } else { b }
}

fn main() {
    println!("Max of 5 and 3: {}", max(5, 3));           // i32
    println!("Max of 5.5 and 3.2: {}", max(5.5, 3.2));   // f64
    println!("Max of 'a' and 'z': {}", max('a', 'z'));    // char
}

output:
Max of 5 and 3: 5
Max of 5.5 and 3.2: 5.5
Max of 'a' and 'z': z
```

## 2. Generic Functions

```rust
use std::fmt::Display;

// Generic function with multiple trait bounds
fn compare_and_print<T>(a: T, b: T) -> T
where
    T: PartialOrd + Display + Copy,
{
    if a > b {
        println!("{} is greater than {}", a, b);
        a
    } else {
        println!("{} is less than or equal to {}", a, b);
        b
    }
}

// Generic function that works with collections
fn find_max<T>(items: &[T]) -> Option<&T>
where
    T: PartialOrd,
{
    if items.is_empty() {
        return None;
    }

    let mut max = &items[0];
    for item in items.iter().skip(1) {
        if item > max {
            max = item;
        }
    }
    Some(max)
}

// Generic function with multiple constraints
fn summarize<T>(items: &[T]) -> String
where
    T: Display + Clone,
{
    if items.is_empty() {
        return "Empty collection".to_string();
    }

    let first = &items[0];
    let last = &items[items.len() - 1];
    let count = items.len();

    format!("Collection with {} items: first = {}, last = {}",
            count, first, last)
}

fn main() {
    // Compare numbers
    let max_num = compare_and_print(15, 25);
    println!("Result: {}\n", max_num);

    // Find maximum in collections
    let numbers = vec![3, 7, 2, 9, 1, 5];
    let strings = vec!["apple", "zebra", "banana", "cherry"];

    if let Some(max) = find_max(&numbers) {
        println!("Max number: {}", max);
    }

    if let Some(max) = find_max(&strings) {
        println!("Max string: {}", max);
    }

    // Summarize collections
    println!("\n{}", summarize(&numbers));
    println!("{}", summarize(&strings));
    println!("{}", summarize(&Vec::<i32>::new()));
}

output:
15 is less than or equal to 25
Result: 25

Max number: 9
Max string: zebra

Collection with 6 items: first = 3, last = 5
Collection with 4 items: first = apple, last = cherry
Empty collection

```

## 3. Generic Structs

```rust
use std::fmt::Display;

#[derive(Debug)]
struct Container<T> {
    items: Vec<T>,
    name: String,
}

impl<T> Container<T> {
    fn new(name: String) -> Self {
        Container {
            items: Vec::new(),
            name,
        }
    }

    fn add(&mut self, item: T) {
        self.items.push(item);
    }

    fn len(&self) -> usize {
        self.items.len()
    }

    fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    fn get(&self, index: usize) -> Option<&T> {
        self.items.get(index)
    }
}

// Implementation with trait bounds
impl<T> Container<T>
where
    T: Display + Clone,
{
    fn display_all(&self) {
        println!("Container '{}' contents:", self.name);
        if self.items.is_empty() {
            println!("  (empty)");
        } else {
            for (i, item) in self.items.iter().enumerate() {
                println!("  {}: {}", i, item);
            }
        }
    }

    fn get_summary(&self) -> String {
        match self.items.len() {
            0 => format!("{}: empty", self.name),
            1 => format!("{}: 1 item ({})", self.name, self.items[0]),
            n => format!("{}: {} items (first: {}, last: {})",
                        self.name, n, self.items[0], self.items[n-1]),
        }
    }
}

// Implementation for numeric types
impl<T> Container<T>
where
    T: std::ops::Add<Output = T> + Copy + Default,
{
    fn sum(&self) -> T {
        self.items.iter().fold(T::default(), |acc, &x| acc + x)
    }
}

fn main() {
    // Container of integers
    let mut numbers = Container::new("Numbers".to_string());
    numbers.add(10);
    numbers.add(20);
    numbers.add(30);

    numbers.display_all();
    println!("Sum: {}", numbers.sum());
    println!("Summary: {}\n", numbers.get_summary());

    // Container of strings
    let mut words = Container::new("Words".to_string());
    words.add("Rust".to_string());
    words.add("is".to_string());
    words.add("awesome".to_string());

    words.display_all();
    println!("Summary: {}\n", words.get_summary());

    // Container of floating point numbers
    let mut floats = Container::new("Floats".to_string());
    floats.add(3.14);
    floats.add(2.71);
    floats.add(1.41);

    floats.display_all();
    println!("Sum: {:.2}", floats.sum());
    println!("Summary: {}", floats.get_summary());
}


output:
Container 'Numbers' contents:
  0: 10
  1: 20
  2: 30
Sum: 60
Summary: Numbers: 3 items (first: 10, last: 30)

Container 'Words' contents:
  0: Rust
  1: is
  2: awesome
Summary: Words: 3 items (first: Rust, last: awesome)

Container 'Floats' contents:
  0: 3.14
  1: 2.71
  2: 1.41
Sum: 7.26
Summary: Floats: 3 items (first: 3.14, last: 1.41)

```

## 4. Generic Enums

```rust
// Custom Result-like enum
#[derive(Debug)]
enum MyResult<T, E> {
    Success(T),
    Failure(E),
}

impl<T, E> MyResult<T, E> {
    fn is_success(&self) -> bool {
        matches!(self, MyResult::Success(_))
    }

    fn is_failure(&self) -> bool {
        matches!(self, MyResult::Failure(_))
    }

    fn unwrap(self) -> T
    where
        E: std::fmt::Debug,
    {
        match self {
            MyResult::Success(value) => value,
            MyResult::Failure(error) => panic!("Called unwrap on failure: {:?}", error),
        }
    }

    fn unwrap_or(self, default: T) -> T {
        match self {
            MyResult::Success(value) => value,
            MyResult::Failure(_) => default,
        }
    }
}

// Generic enum for tree structures
#[derive(Debug)]
enum Tree<T> {
    Empty,
    Node {
        value: T,
        left: Box<Tree<T>>,
        right: Box<Tree<T>>,
    },
}

impl<T> Tree<T> {
    fn new() -> Self {
        Tree::Empty
    }

    fn leaf(value: T) -> Self {
        Tree::Node {
            value,
            left: Box::new(Tree::Empty),
            right: Box::new(Tree::Empty),
        }
    }

    fn node(value: T, left: Tree<T>, right: Tree<T>) -> Self {
        Tree::Node {
            value,
            left: Box::new(left),
            right: Box::new(right),
        }
    }
}

impl<T> Tree<T>
where
    T: std::fmt::Display,
{
    fn print_inorder(&self) {
        match self {
            Tree::Empty => {},
            Tree::Node { value, left, right } => {
                left.print_inorder();
                print!("{} ", value);
                right.print_inorder();
            }
        }
    }
}

fn main() {
    // Using custom Result
    let success: MyResult<i32, &str> = MyResult::Success(42);
    let failure: MyResult<i32, &str> = MyResult::Failure("Something went wrong");

    println!("Success: {:?}, is_success: {}", success, success.is_success());
    println!("Failure: {:?}, is_failure: {}", failure, failure.is_failure());

    let value = MyResult::Success(100).unwrap_or(0);
    let default_value = MyResult::Failure("error").unwrap_or(0);

    println!("Unwrapped value: {}", value);
    println!("Default value: {}", default_value);

    // Using generic tree
    let tree = Tree::node(
        5,
        Tree::node(
            3,
            Tree::leaf(1),
            Tree::leaf(4)
        ),
        Tree::node(
            8,
            Tree::leaf(6),
            Tree::leaf(9)
        )
    );

    println!("\nTree structure: {:?}", tree);
    print!("Inorder traversal: ");
    tree.print_inorder();
    println!();
}

output:
Success: Success(42), is_success: true
Failure: Failure("Something went wrong"), is_failure: true
Unwrapped value: 100
Default value: 0

Tree structure: Node { value: 5, left: Node { value: 3, left: Node { value: 1, left: Empty, right: Empty }, right: Node { value: 4, left: Empty, right: Empty } }, right: Node { value: 8, left: Node { value: 6, left: Empty, right: Empty }, right: Node { value: 9, left: Empty, right: Empty } } }
Inorder traversal: 1 3 4 5 6 8 9

```

## 5. Trait Bounds

```rust
use std::fmt::Display;

// Function with single trait bound
fn print_it<T: Display>(item: T) {
    println!("Item: {}", item);
}

// Function with multiple trait bounds
fn compare_and_display<T>(a: T, b: T)
where
    T: PartialOrd + Display + Copy,
{
    if a > b {
        println!("{} > {}", a, b);
    } else if a < b {
        println!("{} < {}", a, b);
    } else {
        println!("{} = {}", a, b);
    }
}

// Function with complex trait bounds
fn process_items<T, F>(items: Vec<T>, mut processor: F) -> Vec<String>
where
    T: Display + Clone,
    F: FnMut(&T) -> String,
{
    items.iter().map(|item| {
        let processed = processor(item);
        format!("Original: {}, Processed: {}", item, processed)
    }).collect()
}

fn main() {
    // Simple trait bound usage
    print_it(42);
    print_it("Hello, Rust!");
    print_it(3.14);

    // Multiple trait bounds
    compare_and_display(10, 20);
    compare_and_display('z', 'a');
    compare_and_display(5.5, 5.5);

    // Complex trait bounds
    let numbers = vec![1, 2, 3, 4, 5];
    let results = process_items(numbers, |x| format!("squared: {}", x * x));

    for result in results {
        println!("{}", result);
    }
}

output:
Item: 42
Item: Hello, Rust!
Item: 3.14
10 < 20
z > a
5.5 = 5.5
Original: 1, Processed: squared: 1
Original: 2, Processed: squared: 4
Original: 3, Processed: squared: 9
Original: 4, Processed: squared: 16
Original: 5, Processed: squared: 25

```

## 6. Generic Traits

```rust
// Generic trait definition
trait Converter<T> {
    fn convert(&self) -> T;
}

// Struct that implements the generic trait
struct NumberWrapper(i32);

impl Converter<String> for NumberWrapper {
    fn convert(&self) -> String {
        self.0.to_string()
    }
}

impl Converter<f64> for NumberWrapper {
    fn convert(&self) -> f64 {
        self.0 as f64
    }
}

impl Converter<bool> for NumberWrapper {
    fn convert(&self) -> bool {
        self.0 != 0
    }
}

// Generic trait with associated types
trait Iterator2 {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}

// Counter implementation
struct Counter {
    current: usize,
    max: usize,
}

impl Counter {
    fn new(max: usize) -> Self {
        Counter { current: 0, max }
    }
}

impl Iterator2 for Counter {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current < self.max {
            let current = self.current;
            self.current += 1;
            Some(current)
        } else {
            None
        }
    }
}

// Generic trait with default implementations
trait Summarizable<T> {
    fn items(&self) -> &[T];

    fn summarize(&self) -> String
    where
        T: std::fmt::Display,
    {
        match self.items().len() {
            0 => "Empty collection".to_string(),
            1 => format!("Single item: {}", self.items()[0]),
            n => format!("{} items: {} to {}",
                        n, self.items()[0], self.items()[n-1]),
        }
    }

    fn count(&self) -> usize {
        self.items().len()
    }
}

// Implementing the trait for Vec
impl<T> Summarizable<T> for Vec<T> {
    fn items(&self) -> &[T] {
        self.as_slice()
    }
}

fn main() {
    let number = NumberWrapper(42);

    // Convert to different types
    let as_string: String = number.convert();
    let as_float: f64 = number.convert();
    let as_bool: bool = number.convert();

    println!("Number 42 as:");
    println!("  String: {}", as_string);
    println!("  Float: {}", as_float);
    println!("  Boolean: {}", as_bool);

    let zero = NumberWrapper(0);
    let zero_as_bool: bool = zero.convert();
    println!("  Zero as boolean: {}", zero_as_bool);

    // Using custom iterator
    let mut counter = Counter::new(5);
    print!("Counter values: ");
    while let Some(value) = counter.next() {
        print!("{} ", value);
    }
    println!();

    // Using trait with default implementations
    let numbers = vec![10, 20, 30, 40, 50];
    let words = vec!["apple", "banana", "cherry"];
    let empty: Vec<i32> = vec![];

    println!("\nSummaries:");
    println!("Numbers: {}", numbers.summarize());
    println!("Words: {}", words.summarize());
    println!("Empty: {}", empty.summarize());

    println!("\nCounts:");
    println!("Numbers count: {}", numbers.count());
    println!("Words count: {}", words.count());
    println!("Empty count: {}", empty.count());
}


Number 42 as:
  String: 42
  Float: 42
  Boolean: true
  Zero as boolean: false
Counter values: 0 1 2 3 4
Summaries:
Numbers: 5 items: 10 to 50
Words: 3 items: apple to cherry
Empty: Empty collection

Counts:
Numbers count: 5
Words count: 3
Empty count: 0
```
