fn main(){
// Task 1: List all the data types available in Rust

// Integer types
use std::collections::linked_list;
let a: i8 = 1;
let b: i16 = 2;
let c: i32 = 3;
let d: i64 = 4;
let e: i128 = 5;
let f: isize = 6;

let g: u8 = 7;
let h: u16 = 8;
let i: u32 = 9;
let j: u64 = 10;
let k: u128 = 11;
let l: usize = 12;

// Float types
let m: f32 = 13.5;
let n: f64 = 14.5;

// Boolean type
let o: bool = true;

// Character type
let p: char = 'R';

// String types
let q = "Hello";           // &str
let r = String::from("Hi"); // String

// Array type
let s = [1, 2, 3];

// Tuple type
let t = (1, 2.0, 'A');



// Task 2: Use of Each Data Type

let age: i32 = 20; // for whole numbers
let pi: f64 = 3.14; // for decimal numbers
let is_rust_fun: bool = true; // true or false
let letter: char = 'R'; // single character
let name = "Rust"; // text as string slice
let colors = ["red", "green", "blue"]; // list of items
let person = ("John", 25); // group of values



// Task 3: Use various data types to perform mathematical operations

let x: i32 = 10;
let y: i32 = 5;
let sum = x + y;
let difference = x - y;
let product = x * y;
let quotient = x / y;

println!("Sum: {}", sum);
println!("Difference: {}", difference);
println!("Product: {}", product);
println!("Quotient: {}", quotient);



// Task 4: Perform mathematical operations using:

// 1. Float data type
let a: f32 = 5.5;
let b: f32 = 2.0;
let result = a + b;
println!("Float sum: {}", result);

// 2. i32 data type
let c: i32 = 10;
let d: i32 = 4;
let result2 = c * d;
println!("i32 multiplication: {}", result2);

// 3. i64 data type
let e: i64 = 1000;
let f: i64 = 2000;
let result3 = e + f;
println!("i64 addition: {}", result3);



// Task 5:
// 1. Create a string variable with the value "why you love rust"
let reason = "why you love rust";

// 2. Create another string variable to store your name
let name = "My name is Dukey_ICO";

// Combine both strings
let final_message = format!("{} {}", reason, name);

println!("{}", final_message);
}