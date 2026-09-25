fn main () {
human_id("Alice", 30, 165);
add(5, 10);

let y:i32 = add(5, 10);
println!("Y is {}", y);

let _X: i32 = {
    let price: i32 = 100;
    let tax: i32 = 10;
    price + tax
};

println!("X is {}", _X);

//calling bmi function
let weight: f64 = 92.0; // in kilograms
let height: f64 = 1.74; // in meters
let bmi: f64 = calculate_bmi(weight, height);
println!("My BMI is {:.2}", bmi);
}


//insert more than one value
fn human_id(name: &str, age: u32, height: u32) -> String {
    println!("My name is {} and I am {} years old and {}cm tall.", name, age, height);
    format!("{} is {} years old and {} cm tall.", name, age, height)
}

//expressions and statements
//expressions evaluate to a value, statements do not

fn add(a: i32, b: i32) -> i32 {
    a + b
}


//BMI
fn calculate_bmi(weight: f64, height: f64) -> f64 {
    weight / (height * height)
}