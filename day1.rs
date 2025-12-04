use std::fs::File;
use std::io::Read;

fn solve_problem1() {
    let mut file = File::open("inputs/day1.txt").unwrap();
    let mut input_str = String::new();
    let result = file.read_to_string(&mut input_str);
    match result {
        Ok(_) => println!("Successfully read from file"),
        Err(error) => {
            println!("Error {}", error);
        }
    }

    let mut curr_pos = 50;
    let mut password = 0;
    let mut inputs: Vec<&str> = input_str.split('\n').collect();
    let _ = inputs.pop();
    // println!("Inputs: {:?}", inputs);
    for input in inputs {
        let num: i32 = input[1..].parse::<i32>().unwrap();
        // println!("Input: {}", num);
        if input.starts_with('L') {
            curr_pos -= num;
        } else {
            curr_pos += num;
        }
        while curr_pos < 0 || curr_pos > 99 {
            if curr_pos < 0 {
                curr_pos = 100 + curr_pos;
            } else if curr_pos > 99 {
                curr_pos = curr_pos - 100;
            }
        }
        if curr_pos == 0 {
            password += 1;
        }
    }
    println!("Final result: {}", password);
}

fn solve_problem2() {
    let mut file = File::open("inputs/day1.txt").unwrap();
    let mut input_str = String::new();
    let result = file.read_to_string(&mut input_str);
    match result {
        Ok(_) => println!("Successfully read from file"),
        Err(error) => {
            println!("Error {}", error);
        }
    }

    let mut curr_pos = 50;
    let mut password = 0;
    let mut inputs: Vec<&str> = input_str.split('\n').collect();
    let _ = inputs.pop();

    for input in inputs {
        let mut rem: i32 = input[1..].parse::<i32>().unwrap();
        while rem > 0 {
            if input.starts_with('L') {
                curr_pos -= 1;
            } else {
                curr_pos += 1;
            }
            if curr_pos % 100 == 0 {
                password += 1;
            }
            rem -= 1;
        }
    }
    println!("Final result: {}", password);
}

fn main() {
    println!("\n-- Solving problem 1 --");
    solve_problem1();
    println!("\n\n-- Solving problem 2 --");
    solve_problem2();
}
