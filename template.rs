use std::fs::File;
use std::io::Read;

fn solve_problem1() {
    let file_name = "inputs/";
    println!("Reading from file: {}", file_name);
    let mut file = File::open(file_name).unwrap();
    let mut input_str = String::new();
    let result = file.read_to_string(&mut input_str);
    match result {
        Ok(_) => println!("Successfully read from {}", file_name),
        Err(error) => {
            println!("Error {}", error);
        }
    }
}

fn solve_problem2() {
    let file_name = "inputs/";
    println!("Reading from file: {}", file_name);
    let mut file = File::open(file_name).unwrap();
    let mut input_str = String::new();
    let result = file.read_to_string(&mut input_str);
    match result {
        Ok(_) => println!("Successfully read from {}", file_name),
        Err(error) => {
            println!("Error {}", error);
        }
    }
}

fn main() {
    println!("\n-- Solving Problem 1 --");
    solve_problem1();
    println!("\n\n-- Solving Problem 2 --");
    solve_problem2();
}
