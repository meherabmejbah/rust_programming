// My first hello, world! program in Rust.
fn main() {
    println!("Hello, World!");
}
// Variable and Datatypes
fn main() {
    let name = "Meherab Mejbah";
    let mut age = 12;
    age += 5;
    let is_learning_rust = true;
    let favorite_number = 1.0;
    let first_letter = 'R';

    println!("My name is: {}", name);
    println!("My age is: {}", age);
    println!("I am learning Rust: {}", is_learning_rust);
    println!("My favorite number is: {}", favorite_number);
    println!("First letter of my name is: {}", first_letter);

}
// Comparison Operators with if, else if and 
fn main() {
    let number = 10;
    if number > 5 {
        println!("The number is greater than 5");
    }
}

fn  main() {
    let age = 18;

    if age >= 21{
        println!("You have the permission to drink alcohol, no problem");
    } else if age >= 18{
        println!("You are an adult, but cannot permisson to drink alcohol");
    } else {
        println!("You are a minor");
    }           
}
// loop, for and while 
fn main() {
    let mut count = 0;

    loop {
        count += 1;
        if count == 3{
            println!("Breaking the loop at count: {}",count);
            break;
        }
    }
}

fn main() {
    let mut num = 1;

    while num <= 5{
        println!("Number is: {}", num);
        num += 1;
    }
}

fn main() {
    for num in 1..4{
        println!("Number is: {}",num);
    }
}
// match statement
fn main() {
    let traffic_light = "green";

    match traffic_light {
        "green" => println!("Go"),
        "yellow" => println!("Slow down"),
        "red" => println!("Stop"),
        _ => println!("Invalid colour!"),
    }
}
// Function in Rust
fn greet(name: &str) {
    println!("Hello, {}!",name);
}

fn main() {
    greet("Mejbah");
}



fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    let sum = add(5,5);
    println!("The sum is {}",sum);
}


fn outer_function() {           // এটা একটা বাইরের ফাংশন (outer function)
    fn inner_function() {       // এর ভেতরে আরেকটা ফাংশন বানানো হয়েছে (inner function)
        println!("This is an inner function.");  // এটা শুধু প্রিন্ট করে
    }

    inner_function();           // ভেতরের ফাংশনটাকে এখানে ডাকা হয়েছে
}

fn main() {                     // প্রোগ্রামের শুরু এখান থেকেই
    outer_function();           // বাইরের ফাংশনটাকে ডাকা হয়েছে
}


fn calculate(a: i32, b: i32) -> (i32, i32) {
    (a + b, a * b)
}


fn main() {
    let (sum, product) = calculate(3,4);
    println!("Sum: {}, Product: {}", sum,product);
}


fn factorial(n: u32) -> u32 {
    if n == 0 {
        1
    } else {
        n * factorial(n - 1)
    }
}


fn main() {
    let s = String::from("Hello, Rust!");
    println!("{}",s); // s is valid here
}
// s goes out of scope and is dropped    


fn main() {
    let s1 = String::from("Rust!");
    let s2 = s1; //s1 in now invalid, ownership is moved to s2
    println!("{}",s2);
    // println!("{}", s1); This will cause an error
}


fn print_string(s: &String) {
    println!("{}", s);
}

fn main() {
    let s = String::from("Hello!");
    print_string(&s); //Passing a reference
    println!("{}",s); // s is still valid here
}


fn modify_string(s: &mut String) {
    s.push_str(", World!");
}

fn main() {
    let mut s = String::from("Hello");
    modify_string(&mut s);
    println!("{}",s); // Modified string
}


fn takes_ownership(s: String) {
    println!("{}",s);
}

fn makes_copy(x: i32) {
    println!("{}",x);
}

fn main() {
    let s = String::from("Rust!");
    takes_ownership(s); // s is move, and no longer valid here
    let x = 5;
    makes_copy(x); // x is still valid because integers are Copy
}

fn main() {
    let result = factorial(5);
    println!("Factorial of 5 is: {}",result);
}


struct User {
    username: String,
    email: String,
    active: bool,
    sign_in_count: u64,
}
fn main() {
    let user1 = User {
        username: String::from("Rustacean!"),
        email: String::from("rust@gmail.com"),
        active: true,
        sign_in_count: 1,

    };

    println!("Username: {}",user1.username);
    println!("Email: {}",user1.email);
    println!("Active status: {}",user1.active);
    println!("All sing in count: {}", user1.sign_in_count);

}

fn create_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }

}

fn main() {
    let user1 = create_user(String::from("rustacean@rust.com"), String::from("Rustcean"));
    println!("Email: {}", user1.email);
}


fn main() {
    let user1 = User {
    username: String::from("Rustcean"),
    email: String::from("rust@example.com"),
    active: true,
    sign_in_count: 1,
    };

    let user2 = User {
        email: String::from("new@example.com"),
        ..user1
    };

    println!("Username: {}", user2.username);
}


struct Colour(i32, i32, i32);

fn main() {
    let black = Colour(0, 0, 0);
    println!("Colour: {}, {}, {}", black.0, black.1, black.2);
}


struct Color(i32, i32, i32); // RGB color

fn main() {
    let red = Color(255, 0, 0);
    let green = Color(0, 255, 0);
    let blue = Color(0, 0, 255);

    println!("Red: {}, {}, {}", red.0, red.1, red.2);
    println!("Green: {}, {}, {}", green.0, green.1, green.2);
    println!("Blue: {}, {}, {}", blue.0, blue.1, blue.2);
}


struct AlwaysEqual;

fn main() {
    let _subject = AlwaysEqual;
}


struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect = Rectangle {width: 30, height: 50};
    println!("Area: {}", rect.area());
}


enum Direction {
    North,
    South,
    West,
    East,
}

fn main() {
    let move_direction = Direction::North;
    match move_direction {
        Direction::North => println!("Heading North"),
        Direction::South => println!("Heading South"),
        Direction::West => println!("Heading West"),
        Direction::East => println!("Heading East"),
    }
}


enum TrafficLight {
    Red,
    Yellow,
    Green,
}

fn action(light: TrafficLight) {
    match light {
        TrafficLight::Red => println!("Stop!"),
        TrafficLight::Yellow => println!("Get ready"),
        TrafficLight::Green => println!("Go!"),
    }
}

fn main() {
    let current_light = TrafficLight::Red;
    action(current_light);
}



enum Vehicle {
    Car(String),
    Bike(String),
}

impl Vehicle {
    fn drive(&self) {
        match self {
            Vehicle::Car(name) => println!("Driving a car: {}", name),
            Vehicle::Bike(name) => println!("Riding a bike: {}", name),

        }
    }
}

fn main() {
    let my_car = Vehicle::Car(String::from("Rolls-Royce"));
    let my_bike = Vehicle::Bike(String::from("Ducati"));

    my_car.drive();
    my_bike.drive();
}


#[derive(Debug)]
#[allow(dead_code)]
enum IpAddr {
    V4(String),
    V6(String),
}


fn main() {
    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));

    println!("Home address: {:?}", home);
    println!("Loopback address: {:?}", loopback);
}



#[allow(dead_code)]
enum PaymentMethod {
    CreditCard(String),
    DebitCard(String),
    Cash,
    Paypal,
}

fn print_payment_method(method: PaymentMethod) {
    match method {
        PaymentMethod::CreditCard(card_number) => println!("Paid with Credit Card: {}", card_number),
        PaymentMethod::DebitCard(card_number) => println!("Paid with DebitCard: {}", card_number),
        PaymentMethod::Cash => println!("Paid with Cash"),
        PaymentMethod::Paypal => println!("Paid with Paypal"),
    }
}

fn main() {
    let payment = PaymentMethod::CreditCard(String::from("1234-5678-9012-3456"));
    print_payment_method(payment);
}


fn main() {
    let mut numbers: Vec<i32> = Vec::new();

    numbers.push(1);
    numbers.push(2);
    numbers.push(3);

    println!("Numbers: {:?}", numbers);

    if let Some(last) = numbers.pop() {
        println!("Popped: {}", last);
    }

    println!("Numbers after pop: {:?}", numbers);
}


fn main() {
    let mut greeting = String::from("Hello");

    greeting.push_str(", World!");
    println!("{}", greeting);

    let substring = &greeting[0..5]; //"Hello"
    println!("Substring: {}", substring);
    
}

use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Dibbo"), 100);
    scores.insert(String::from("Rahi"), 100);
    scores.insert(String::from("Mejbah"), 90);

    println!("Scores: {:?}", scores);

    let dibbo_score = scores.get("Dibbo").unwrap();
    println!("Dibbo's score: {}", dibbo_score);
}


use std::collections::BTreeMap;

fn main() {
    let mut scores = BTreeMap::new();

    scores.insert("Alice", 50);
    scores.insert("Bob", 60);
    scores.insert("Charlie", 70);

    for (name, score) in &scores {
        println!("{}: {}", name, score);
    }
}



use std::collections::VecDeque;

fn main() {
    let mut queue: VecDeque<i32> = VecDeque::new();

    queue.push_back(1);
    queue.push_back(2);
    queue.push_front(0);

    println!("Queue: {:?}", queue);

    if let Some(front) = queue.pop_front() {
        println!("Removed from front: {}", front);
    }

    println!("Queue after pop: {:?}", queue);
}


use std::collections::LinkedList;

fn main() {
    let mut list = LinkedList::new();

    list.push_back(1);
    list.push_back(2);
    list.push_front(3);

    for value in &list {
        println!("{}", value);
    }

    if let Some(front) = list.pop_front() {
        println!("Removed from front: {}", front);
    }

    println!("List after pop: {:?}", list);
}


use std::collections::HashMap;

fn main() {
    let mut inventory: Vec<String> = Vec::new();
    let mut quantities: HashMap<String, i32> = HashMap::new();

    // Add items
    inventory.push(String::from("Apples"));
    quantities.insert(String::from("Apples"), 10);

    // Update quantity
    *quantities.get_mut("Apples").unwrap() += 5;

    // Remove item
    inventory.retain(|item| item != "Bananas");

    // Print inventory
    for item in &inventory {
        let quantity = quantities.get(item).unwrap();
        println!("{}: {}", item, quantity);
    }
}


fn get_value(index: usize) -> Option<i32> {
    let values = vec![10,20,30];
    values.get(index).copied() // Return Some(value) or None
}

fn main() {
    // Safe usage with `unwarp_or`
    let value = get_value(5).unwrap_or(0); //Fallback value if index is out of bounds
    println!("Value: {}", value);
}


fn get_value(index: usize) -> Option<i32> {
    let values = vec![10,20,30];
    values.get(index).copied()
}

fn main() {
    match get_value(1) {
        Some(value) => println!("Value: {}", value),
        None => println!("No value found at that index."),

    }
}

use std::fs::File;
use std::io::{self, Read};

fn read_file(_file: &str) -> Result<String, io::Error> {
    let mut file = File::open(_file)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn main() {
    match read_file("hello.txt") {
        Ok(contents) => println!("File contents:\n{}", contents),
        Err(e) => eprintln!("Error reading file: {}", e),
    }
}


use std::fmt;
use std::fs::File;
use std::io::{self, Read};


#[derive(Debug)]

enum MyError {
    Io(io::Error),
    Parse(std::num::ParseIntError),
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MyError::Io(e) => write!(f, "I/O error {}", e),
            MyError::Parse(e) => write!(f, "Parse error: {}", e),

        }
    }
}

fn read_file(filename: &str) -> Result<String, MyError> {
    let mut file = File::open(filename).map_err(MyError::Io)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).map_err(MyError::Io)?;
    Ok(contents)
} 


fn main() {
    match read_file("input.txt") {
        Ok(contents) => println!("File contents:\n{}", contents),
        Err(e) => eprintln!("Error: {}", e),
    }
}



fn parse_numbers(input: Vec<&str>) -> Vec<Result<i32, std::num::ParseIntError>> {
    input.iter().map(|s| s.parse::<i32>()).collect()
}

fn main() {
    let inputs = vec!["43","53", "hello"];
    let results: Vec<_> = parse_numbers(inputs);

    for result in results {
        match result {
            Ok(num) => println!("Parsed numbers: {}", num),
            Err(e) => println!("Error parsing input: {}",e),
        }
    }
}

// Practice
use std::fmt;

#[derive(Debug)]
enum MyError {
    NotFound,
    InvalidInput,
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &
    mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MyError::NotFound => write!(f, "Resource no;t found"),
            MyError::InvalidInput => write!(f, "Invalid input provided"),            
        }
    }
}

fn perform_action() -> Result<(), MyError> {
    Err(MyError::InvalidInput)    
}

fn main() {
    match perform_action() {
        Ok(_) => println!("Action performed successfully!"),
        Err(e) => eprintln!("Error: {}", e),
    }
}

fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn main() {
    let numbers = vec![32,63,62,62,344,32,67,2];
    println!("The largest number is: {}", largest(&numbers));

    let chars = vec!['r','u','s','t'];
    println!("The largest char is: {}", largest(&chars));
}


struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let integer_point = Point {x: 5, y: 10};
    let float_point = Point {x: 1.0, y: 4.0};

    println!("Integer Point: ({}, {})", integer_point.x, integer_point.y);
    println!("Float Point: ({}, {})", float_point.x, float_point.y);
}

#[derive(Debug)]
enum Option<T> {
    Some(T),
    None,
}
 
fn main() {
    let number = Option::Some(4);
    let text = Option::Some("Generics in Rust");
    let no_value: Option<i32> = Option::None;

    println!("Number is: {:?}", number);
    println!("Text is {:?}", text);
    println!("Is their any value: {:?}", no_value);
}

use std::fmt::Debug;

struct Pair<T, U> {
    first: T,
    second: U,
}

impl<T: Debug, U: Debug> Pair<T, U> {
    fn show(&self) {
        println!("Pair contains: ({:?}, {:?})", self.first, self.second);
    }
}

fn main() {
    let pair = Pair { first: "Rust", second: 244 };
    pair.show();
}


trait Describe {
    fn description(&self) -> String;
}

struct Person {
    name: String,
    age : u32,
}

impl Describe for Person  {
    fn description(&self) -> String {
        format!("{} is {} years old.",self.name, self.age)
    }
}

fn main() {
    let person = Person {
        name: String::from("Dibbo"),
        age : 17,
    };

    println!("{}",person.description());
}


#[derive(Debug, Clone)]
struct Item {
    name: String,
    value: i32,
}

fn main() {
    let item1 = Item {
        name: String::from("Learning Rust!"),
        value: 100,
    };
    
    let item2 = item1.clone(); // Cloning item1

    println!("{:?}", item1);
    println!("{:?}", item2);
}


trait Describe {
    fn describe(&self) -> String;
}

struct Car {
    brand: String,
}

impl Describe for Car {
    fn describe(&self) -> String {
        format!("This is a car from {}", self.brand)
    }
}

struct Bike {
    brand: String,
}

impl Describe for Bike {
    fn describe(&self) -> String {
        format!("This is a bike from {}", self.brand)
    }
}

fn main() {
    let car = Car { brand: String::from("Toyota!")};
    let bike = Bike {brand: String::from("Ducati!")};

    println!("{}", car.describe());
    println!("{}", bike.describe());
}


mod greetings {
    pub fn hello() {
        println!("Hello, Rustaceans!");
    }
}

fn main() {
    greetings::hello();
}


mod outer {
    pub mod inner {
        pub fn greet() {
            println!("Hello from inner module!");
        }
    }
}


fn main() {
    outer::inner::greet();
}


mod my_utils {
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }
}

fn main() {
    let result = my_utils::add(4,6);
    println!("The result is: {}", result);
}


mod geometry {
    pub fn rectangle_area(length: f64, width: f64) -> f64 {
        length * width
    }
}


mod math_operations {
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    pub fn subtract(a: i32, b: i32) -> i32 {
        a - b
    }
}

fn main() {
    let sum = math_operations::add(3,5);
    let difference = math_operations::subtract(5, 1);

    println!("Sum {}", sum);
    println!("Difference {}", difference);

    let area = geometry::rectangle_area(3.0,5.0);
    println!("Area of rectangle: {}", area);
}

🧪 Unit Tests(Test)

fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] 
    fn test_add() {
        assert_eq!(add(2, 3), 5);
        assert_eq!(add(-1, 1), 0);
        assert_eq!(add(0, 0), 0);
    }
}

 Integration Tests(Test)

#[cfg(test)]
mod tests {
    #[test]
    fn test_integration() {
        let result = super::add(2, 3); // Import the function to test
        assert_eq!(result, 5);
    }
}


⚡ The should_panic Attribute
#[test]
#[should_panic]
fn it_panics_with_message() {
    assert_eq!("Hello,", "World!");
}


Documentation Tests?(Test)

/// Adds two numbers.
///
/// # Examples
///
/// ```
/// let result = add(2, 3);
/// assert_eq!(result, 5);
/// ```
fn add(a: i32, b: i32) -> i32 {
    a + b
}

// #[test]
fn main() {
    let result = add(2,3);
    assert_eq!(result, 5);
}


Hands-On Challenge

/// Subtract two numbers.
/// # Examples
/// 
/// ```
/// let result = subtract(5, 2);
/// assert_eq!(result, 3);
/// ```
fn subtract(a: i32, b: i32) -> i32 {
    a - b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_subtract() {
        assert_eq!(subtract(5, 2), 3);
        assert_eq!(subtract(10, 5), 5);
    }
}

// Macros in Rust(Macros)-15
// Declarative Macros (macro_rules!)
macro_rules! repeat {
    ($x:expr; $count:expr) => {
        for _ in 0..$count {
            println!("{}", $x);
        }
    };
}

fn main() {
    repeat!("Rust is fucking good!"; 5)
}

// Derive Macros
#[derive(Debug)]
struct Book {
    title: String,
    author: String,
}

fn main() {
    let book = Book {
        title: String::from("The Rust Book"),
        author: String::from("I don't no who is the author"),
    };
    println!("{:?}", book);
}

// Function-like Macros
macro_rules! create_function {
    ($func_name: ident) => {
        fn $func_name() {
            println!("Function {:?} called", stringify!($func_name));
        }
    } 
}
create_function!(hello_rust);

fn main() {
    hello_rust()
}

macro_rules! calculate {
    ($a:expr, $b:expr) => {
        println!("The result is: {}", $a + $b);
    };
}

fn main() {
    calculate!(5,10);
}
