// Ownership, referencing and borrowing in Rust

// In Rust, ownership is a set of rules that governs how a Rust program manages memory. The ownership system is designed to ensure memory safety without needing a garbage collector.

//each value in Rust has a variable that’s called its owner. There can only be one owner at a time, and when the owner goes out of scope, the value will be dropped (memory is freed).

// fn main () {
//     let s1 = String::from("hello");
//     let s2 = s1;
//     println!("{}, world!", s2); //s1 is no longer valid
// }

fn main() {
    let s1 = String::from("hello");
    let s2 = s1.clone(); // clone creates a deep copy of the value
    println!("{}, world!", s1); // s1 is still valid
    println!("{}, world!", s2); // s2 is also valid
}