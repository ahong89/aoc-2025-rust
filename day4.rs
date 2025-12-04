use std::fs::File;
use std::io::Read;

fn solve_problem1() {
    let file_name = "inputs/day4.txt";
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

    let mut lines: Vec<&str> = input_str.split('\n').collect();
    let _ = lines.pop();
    let mut matrix: Vec<Vec<bool>> = vec![vec![false; lines[0].len()]; lines.len()];
    for i in 0..matrix.len() {
        for j in 0..matrix[0].len() {
            matrix[i][j] = lines[i].chars().nth(j) == Some('@');
        }
    }
    let directions: [(i32, i32); 8] = [(1, 1), (1, 0), (1, -1), (0, 1), (0, -1), (-1, 1), (-1, 0), (-1, -1)];
    let mut accessible = 0;
    for i in 0..matrix.len() {
        for j in 0..matrix[0].len() {
            if !matrix[i][j] {
                continue;
            }
            let mut count = 0;
            for (i_delta, j_delta) in directions {
                let i_new = i as i32 + i_delta;
                let j_new = j as i32 + j_delta;
                if i_new < 0 || i_new >= matrix.len() as i32 || j_new < 0 || j_new >= matrix[0].len() as i32 {
                    continue;
                }
                if matrix[i_new as usize][j_new as usize] {
                    count += 1;
                }
            }
            if count < 4 {
                accessible += 1;
            }
        }
    }
    println!("Result: {}", accessible);
}

fn solve_problem2() {
    let file_name = "inputs/day4.txt";
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

    let mut lines: Vec<&str> = input_str.split('\n').collect();
    let _ = lines.pop();
    let mut matrix: Vec<Vec<bool>> = vec![vec![false; lines[0].len()]; lines.len()];
    for i in 0..matrix.len() {
        for j in 0..matrix[0].len() {
            matrix[i][j] = lines[i].chars().nth(j) == Some('@');
        }
    }
    let directions: [(i32, i32); 8] = [(1, 1), (1, 0), (1, -1), (0, 1), (0, -1), (-1, 1), (-1, 0), (-1, -1)];
    let mut removed = 0;
    let mut changed = true;
    while changed {
        changed = false;
        for i in 0..matrix.len() {
            for j in 0..matrix[0].len() {
                if !matrix[i][j] {
                    continue;
                }
                let mut count = 0;
                for (i_delta, j_delta) in directions {
                    let i_new = i as i32 + i_delta;
                    let j_new = j as i32 + j_delta;
                    if i_new < 0 || i_new >= matrix.len() as i32 || j_new < 0 || j_new >= matrix[0].len() as i32 {
                        continue;
                    }
                    if matrix[i_new as usize][j_new as usize] {
                        count += 1;
                    }
                }
                if count < 4 {
                    removed += 1;
                    matrix[i][j] = false;
                    changed = true;
                }
            }
        }
    }
    println!("Result: {}", removed);
}

fn main() {
    println!("\n-- Solving Problem 1 --");
    solve_problem1();
    println!("\n\n-- Solving Problem 2 --");
    solve_problem2();
}
