use std::fs::File;
use std::io::Read;
use std::cmp;

fn parse_ranges(input_lines: &Vec<&str>) -> Vec<(u128, u128)> {
    let mut input_ranges: Vec<(u128, u128)> = Vec::new();
    for line in input_lines {
        if *line == "" {
            input_ranges.sort_by_key(|x| x.0);
            return input_ranges;
        }
        let line_split: Vec<&str> = line.split('-').collect();
        let start: u128 = line_split[0].parse().unwrap();
        let end: u128 = line_split[1].parse().unwrap();
        input_ranges.push((start, end));
    }
    return input_ranges;
}

fn reduce_ranges(input_ranges: &Vec<(u128, u128)>) -> Vec<(u128, u128)> {
    let mut ranges: Vec<(u128, u128)> = Vec::new();
    let mut curr_start = input_ranges[0].0;
    let mut curr_end = input_ranges[0].1;
    for (start, end) in input_ranges {
        if *start > curr_end { // current range is complete
            ranges.push((curr_start, curr_end));
            curr_start = *start;
            curr_end = *end;
        } else {
            curr_end = cmp::max(curr_end, *end);
        }
    }
    ranges.push((curr_start, curr_end));
    return ranges;
}

fn parse_available(input_lines: &Vec<&str>, start_index: usize) -> Vec<u128> {
    let mut available: Vec<u128> = Vec::new();
    for i in start_index..(input_lines.len() - 1) {
        available.push(input_lines[i].parse().unwrap());
    }
    return available;
}

fn solve_problem1() {
    let file_name = "inputs/day5.txt";
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

    let input_lines: Vec<&str> = input_str.split('\n').collect();
    let input_ranges: Vec<(u128, u128)> = parse_ranges(&input_lines);
    let available: Vec<u128> = parse_available(&input_lines, input_ranges.len() + 1);
    let ranges: Vec<(u128, u128)> = reduce_ranges(&input_ranges);

    let mut fresh_count: i32 = 0;
    for a in available {
        for (start, end) in &ranges {
            if *start <= a && *end >= a {
                fresh_count += 1;
                break;
            }
        }
    }
    println!("Final result: {}", fresh_count);
}

fn solve_problem2() {
    let file_name = "inputs/day5.txt";
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

    let input_lines: Vec<&str> = input_str.split('\n').collect();
    let input_ranges: Vec<(u128, u128)> = parse_ranges(&input_lines);
    let ranges: Vec<(u128, u128)> = reduce_ranges(&input_ranges);
    let mut fresh_count: u128 = 0;
    for (start, end) in &ranges {
        fresh_count += end - start; 
    }
    fresh_count += ranges.len() as u128;
    println!("Final result: {}", fresh_count);
}

fn main() {
    println!("\n-- Solving Problem 1 --");
    solve_problem1();
    println!("\n\n-- Solving Problem 2 --");
    solve_problem2();
}
