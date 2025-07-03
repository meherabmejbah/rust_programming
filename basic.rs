fn main() {
    println!("Hello, World!");
}

fn main() {
    let name = "Meherab Mejbah";
    let mut age = 12;
    age += 5;
    let is_learning_rust = true;
    let favorite_number = 1.0;
    let first_letter = 'R';

    println!("My name is: {}",name);
    println!("My age is: {}",age);
    println!("I am learning Rust: {}",is_learning_rust);
    println!("My favorite number is: {}",favorite_number);
    println!("First letter of my name is: {}",first_letter);

}


fn main() {
    let number = 10;
    if number > 5{
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



fn main() {
    let traffic_light = "Green";

    match traffic_light {
        "green" => println!("Go"),
        "yellow" => println!("Slow down"),
        "red" => println!("Stop"),
        _ => println!("Invalid colour!"),
    }
}



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
    let result = factorial(5);
    println!("Factorial of 5 is: {}",result);
}
