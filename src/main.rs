// Ownership, referencing and borrowing in Rust

// In Rust, ownership is a set of rules that governs how a Rust program manages memory. The ownership system is designed to ensure memory safety without needing a garbage collector.

//each value in Rust has a variable that’s called its owner. There can only be one owner at a time, and when the owner goes out of scope, the value will be dropped (memory is freed).

// fn main () {
//     let s1 = String::from("hello");
//     let s2 = s1;
//     println!("{}, world!", s2); //s1 is no longer valid
// }

// fn main() {
//     let s1 = String::from("hello");
//     let s2 = s1.clone(); // clone creates a deep copy of the value
//     println!("{}, world!", s1); // s1 is still valid
//     println!("{}, world!", s2); // s2 is also valid
// }

//borrowing allows you to have references to a value without taking ownership of it. This is done using the & symbol.

// fn main() {

//     let x: i32 = 5;

//     let y = &x; //y is a reference to the value of x

//     println!("The value of x is: {}", x);
//     println!("The value of y is: {}", y);
// }

//structs in Rust are a way to create custom data types that can hold multiple values. Structs can have fields of different types, and they can be used to model real-world entities.

// struct BankAccount {
//     owner: String,
//     account_number: String,
//     balance: f64,
// }

// fn withdraw(account: &mut BankAccount, amount: f64) {
//     if account.balance >= amount {
//         account.balance -= amount;
//         println!(" Account owned by {}", account.owner);
//         println!("Withdrawal successful. New balance: {}", account.balance);
//     } else {
//         println!(" Account owned by {}", account.owner);
//         println!("Insufficient funds. Current balance: {}", account.balance);
//     }
// }

// fn main() {
//     let mut account = BankAccount {
//         owner: "John Doe".to_string(),
//         account_number: "123456789".to_string(),
//         balance: 1000.0,
//     };

//     withdraw(&mut account, 1200.0);
//     withdraw(&mut account, 200.0);
// }

//variables and mutability in Rust are important concepts that determine how data can be modified. By default, variables in Rust are immutable, meaning their values cannot be changed after they are assigned. However, you can make a variable mutable by using the mut keyword.

// fn main() {
//     let mut x = 5;
//     println!("The value of x is: {}", x);
//     x = 6;
//     println!("The value of x is: {}", x);
// }

//constants in Rust are similar to variables, but they are always immutable and must have a type annotation. Constants are defined using the const keyword and can be declared in any scope, including the global scope. They are typically used for values that should not change throughout the program.

// fn main() {
//     const MAX_POINTS: u32 = 100_000;
//     println!("The value of MAX_POINTS is: {}", MAX_POINTS);
// }

// shadowing in Rust allows you to declare a new variable with the same name as a previous variable. The new variable shadows the previous one, effectively creating a new binding. This can be useful for transforming values or changing types while keeping the same variable name.

// fn main() {
//     let x = 5;
//     let x = x + 1;
//     let x = x * 2;
//     println!("The value of x is: {}", x);
// }

//comments in Rust are used to explain the code and make it more readable. Single-line comments start with //, while multi-line comments are enclosed in /* */. Comments are ignored by the compiler and do not affect the program's execution.

// fn main() {
//     // This is a single-line comment
//     let x = 5; // This is a multi-line comment
//     println!("The value of x is: {}", x);
// }

// control flow in Rust is used to determine the order in which statements are executed. Rust provides several control flow constructs, including if expressions, loops, and match expressions.

// if else statements allow you to execute different blocks of code based on a condition. The condition must evaluate to a boolean value (true or false).



// fn main() {
//     let number = 6;

//     if number % 4 == 0 {
//         println!("number is divisible by 4");
//     } else if number % 3 == 0 {
//         println!("number is divisible by 3");
//     } else {
//         println!("number is not divisible by 4 or 3");
//     }
// }

// loops allow you to execute a block of code multiple times. Rust provides two types of loops: while loops and for loops.

// fn main() {
//     let mut number = 3;

//     while number != 0 {
//         println!("{}!", number);

//         number -= 1;
//     }

//     println!("LIFTOFF!!!");
// }

// fn main() {
//     for number in (1..4).rev() {
//         println!("{}!", number);
//     }
//     println!("LIFTOFF!!!");
// }

// structs in Rust are a way to create custom data types that can hold multiple values. Structs can have fields of different types, and they can be used to model real-world entities.

/*  struct BankAccount {
    owner: String,
    account_number: String,
    balance: f64,
}

fn withdraw(account: &mut BankAccount, amount: f64) {
    if account.balance >= amount {
        account.balance -= amount;
        println!(" Account owned by {}", account.owner);
        println!("Withdrawal successful. New balance: {}", account.balance);
    } else {
        println!(" Account owned by {}", account.owner);
        println!("Insufficient funds. Current balance: {}", account.balance);
    }
}

use std::collections::HashMap;

fn main() {
    let mut account = BankAccount {
        owner: "John Doe".to_string(),
        account_number: "123456789".to_string(),
        balance: 1000.0,
    };

    withdraw(&mut account, 1200.0);
    withdraw(&mut account, 200.0);
} */ 

// enums in Rust are a way to define a type that can be one of several variants. Enums are useful for representing a value that can take on different forms, such as a state or a choice.

/* fn main() {
    enum Direction {
        Up,
        Down,
        Left,
        Right,
    }

    let player_direction = Direction::Up;
    let enemy_direction = Direction::Left;

    match player_direction {
        Direction::Up => println!("Player is moving up"),
        Direction::Down => println!("Player is moving down"),
        Direction::Left => println!("Player is moving left"),
        Direction::Right => println!("Player is moving right"),
    }

    match enemy_direction {
        Direction::Up => println!("Enemy is moving up"),
        Direction::Down => println!("Enemy is moving down"),
        Direction::Left => println!("Enemy is moving left"),
        Direction::Right => println!("Enemy is moving right"),
    }
} */

// error handling in Rust is done using the Result and Option types. The Result type is used for functions that can return an error, while the Option type is used for values that may or may not be present. Rust encourages handling errors explicitly, which helps prevent unexpected crashes and improves code reliability.

// fn main() {
//     let result = divide(11, 0);
//     match result {
//         Ok(value) => println!("Result: {}", value),
//         Err(error) => println!("Error: {}", error),
//     }
// }

// fn divide(a: i32, b: i32) -> Result<i32, String> {
//     if b == 0 {
//         Err("Cannot divide by zero".to_string())
//     } else {
//         Ok(a / b)
//     }
// }

// Collection types in Rust are used to store multiple values in a single variable. The most commonly used collection types are arrays, vectors, and hash maps. Arrays have a fixed size and can hold elements of the same type, while vectors can grow or shrink in size and can also hold elements of the same type. Hash maps store key-value pairs and allow for efficient lookups based on keys.

fn main() {
    let array: [i32; 5] = [1, 2, 3, 4, 5];
    let vector = vec![1, 2, 3, 4, 5];
    // let hash_map = HashMap::new();

    println!("Array: {:?}", array);
    println!("Vector: {:?}", vector);
    // println!("Hash map: {:?}", hash_map);
}

