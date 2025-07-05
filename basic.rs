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
