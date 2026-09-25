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

fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
}

//constants in Rust are similar to variables, but they are always immutable and must have a type annotation. Constants are defined using the const keyword and can be declared in any scope, including the global scope. They are typically used for values that should not change throughout the program.