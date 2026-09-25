//primitive data types

//int, float, char, bool

//Integer types: i8, i16, i32, i64, i128, isize (signed integers)
//u8, u16, u32, u64, u128, usize (unsigned integers)

// fn main() {
//     let x: i32 = -42;
//     let y: u32 = 42;

//     println!("Signed integer: {}", x);
//     println!("Unsigned integer: {}", y);
// }

//floats: f32, f64

fn main() {
    let x: f32 = 3.0;
    let y: f64 = 3.0;

    println!("Float (f32): {}", x);
    println!("Float (f64): {}", y);

    //boolean
    let is_snowing: bool = true;
    println!("Is it snowing? {}", is_snowing);

    //character
    let letter: char = 'A';
    println!("Character: {}", letter);

    //compound data types: tuples and arrays, slices, and strings

    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Number: {:?}", numbers);

    let fruits: [&str; 3] = ["Apple", "Banana", "Cherry"];
    println!("Fruits: {:?}", fruits);

    //tuples
    let person: (&str, i32) = ("Alice", 30);
    println!("Name: {}, Age: {}", person.0, person.1);

    //slices
    let slice: &[i32] = &[1, 6, 5, 2, 3];
    println!("Slice: {:?}", slice);

    //strings vs string slices
    let mut stone_cold: String = String::from("Hell, ");
    stone_cold.push_str("Yeah, buddy!");
    println!("Stone Cold says: {}", stone_cold);

    //B- &str (string slices)
    let mut stone_cold: &str = "Hell, ";
    stone_cold = "Yeah, buddy!";
    println!("Stone Cold says: {}", stone_cold);
}