use std::fs::File;
use std::io::Read;

fn solve_problem1() {
    let mut file = File::open("inputs/day3.txt").unwrap();
    let mut input_str = String::new();
    let result = file.read_to_string(&mut input_str);
    match result {
        Ok(_) => println!("Successfully read from file"),
        Err(error) => {
            println!("Error {}", error);
        }
    }

    let mut banks: Vec<&str> = input_str.split('\n').collect();
    let _ = banks.pop().unwrap();
    let mut sum: i32 = 0;
    for bank in banks {
        let mut batteries_s: Vec<&str> = bank.split("").collect();
        batteries_s.remove(0);
        batteries_s.remove(batteries_s.len() - 1);
        let batteries: Vec<i32> = batteries_s
            .iter()
            .map(|battery_s| {
                let battery_i: i32 = battery_s.parse().unwrap();
                battery_i
            })
            .collect();
        let mut max_l: i32 = -1;
        let mut max_r: i32 = -1;
        for (i, b) in batteries.iter().enumerate() {
            if *b > max_l && i != batteries.len() - 1 {
                max_l = *b;
                max_r = batteries[i + 1];
            } else if *b > max_r {
                max_r = *b;
            }
        }
        // println!("{}{}", max_l, max_r);
        if max_l == 0 {
            sum += max_r;
        } else {
            sum += max_l * 10 + max_r;
        }
    }
    println!("Final result: {}", sum);
}

fn solve_problem2() {
    let mut file = File::open("inputs/day3.txt").unwrap();
    let mut input_str = String::new();
    let result = file.read_to_string(&mut input_str);
    match result {
        Ok(_) => println!("Successfully read from file"),
        Err(error) => {
            println!("Error {}", error);
        }
    }

    let mut banks: Vec<&str> = input_str.split('\n').collect();
    let _ = banks.pop().unwrap();
    let mut sum: u128 = 0;
    for bank in banks {
        let mut batteries_s: Vec<&str> = bank.split("").collect();
        batteries_s.remove(0);
        batteries_s.remove(batteries_s.len() - 1);
        let batteries: Vec<i32> = batteries_s
            .iter()
            .map(|battery_s| {
                let battery_i: i32 = battery_s.parse().unwrap();
                battery_i
            })
            .collect();

        let mut max_digits: [i32; 12] = [0; 12];
        let mut indices: [usize; 13] = [0; 13];
        for digit_i in 0..12 {
            let r_padding = 11 - digit_i;
            let start = match digit_i {
                0 => 0,
                _ => indices[digit_i] + 1 as usize,
            };
            let end = batteries.len() - r_padding;
            for i in start..end { 
                if batteries[i] > max_digits[digit_i] {
                    max_digits[digit_i] = batteries[i];
                    indices[digit_i + 1] = i;
                }
            }
        }
        let mut number: u128 = 0;
        for i in 0..12 { 
            number += max_digits[i] as u128 * (10 as u128).pow(11 - i as u32);
        }
        // println!("{:?}", number);
        sum += number;
    }
    println!("Final result: {}", sum);

}

fn main() {
    println!("\n-- Solving Problem 1 --");
    solve_problem1();
    println!("\n\n-- Solving Problem 2 --");
    solve_problem2();
}
