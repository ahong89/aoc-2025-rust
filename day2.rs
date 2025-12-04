use std::fs::File;
use std::io::Read;

fn verify_num_p1(num: i64) -> bool {
    let digits: i32 = (num as f64).log10() as i32 + 1;
    if digits % 2 == 1 {
        return true;
    }
    let exp: u32 = (digits / 2) as u32;
    let front = num / (10 as i64).pow(exp);
    let back = num - (front * (10 as i64).pow(exp));
    return front != back;
}

fn solve_problem1() {
    let mut file = File::open("inputs/day2.txt").unwrap();
    let mut input_str = String::new();
    let result = file.read_to_string(&mut input_str);
    match result {
        Ok(_) => println!("Successfully read from file"),
        Err(error) => {
            println!("Error {}", error);
        }
    }

    let ranges: Vec<&str> = input_str.split(',').collect();
    let mut sum: i64 = 0;
    for range in ranges {
        let ranges_split: Vec<&str> = range.split('-').collect();
        // println!("ranges_split, {:?}", ranges_split);
        let bottom: i64 = ranges_split[0].parse().expect("Failed to parse bottom");
        let top: i64 = ranges_split[1].trim_matches('\n').parse().expect("Failed to parse top");
        for curr in bottom..=top {
            if !verify_num_p1(curr) {
                // println!("{}", curr);
                sum += curr;
            }
        }
    }
    println!("Final result: {}", sum);
}

fn verify_num_p2(num: i64) -> bool {
    let digits: i32 = (num as f64).log10() as i32 + 1;
    let str = num.to_string();
    for len in 1..digits {
        if digits % len != 0 {
            continue;
        }
        let mut broke = false;
        for itr in 0..(digits / len - 1) {
            let start1: usize = (itr * len) as usize;
            let start2: usize = ((itr + 1) * len) as usize;
            let end2: usize = ((itr + 2) * len) as usize;
            let sub1 = &str[start1..start2];
            let sub2 = &str[start2..end2];
            if sub1 != sub2 {
                broke = true;
                break;
            }
        }
        if !broke {
            return false;
        }
    }
    return true;
}

fn solve_problem2() {
    let mut file = File::open("inputs/day2.txt").unwrap();
    let mut input_str = String::new();
    let result = file.read_to_string(&mut input_str);
    match result {
        Ok(_) => println!("Successfully read from file"),
        Err(error) => {
            println!("Error {}", error);
        }
    }

    let ranges: Vec<&str> = input_str.split(',').collect();
    let mut sum: i64 = 0;
    for range in ranges {
        let ranges_split: Vec<&str> = range.split('-').collect();
        // println!("ranges_split, {:?}", ranges_split);
        let bottom: i64 = ranges_split[0].parse().expect("Failed to parse bottom");
        let top: i64 = ranges_split[1].trim_matches('\n').parse().expect("Failed to parse top");
        for curr in bottom..=top {
            if !verify_num_p2(curr) {
                println!("{}", curr);
                sum += curr;
            }
        }
    }
    println!("Final result: {}", sum);
}

fn main() {
    println!("\n-- Solving Problem 1 --");
    solve_problem1();
    println!("\n\n-- Solving Problem 2 --");
    solve_problem2();
}
