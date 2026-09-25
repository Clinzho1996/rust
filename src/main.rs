fn main () {
human_id("Alice", 30, 165);
}


//insert more than one value
fn human_id(name: &str, age: u32, height: u32) -> String {
    println!("My name is {} and I am {} years old and {}cm tall.", name, age, height);
    format!("{} is {} years old and {} cm tall.", name, age, height)
}

//expressions and statements
//expressions evaluate to a value, statements do not
