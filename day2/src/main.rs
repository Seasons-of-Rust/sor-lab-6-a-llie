use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn part_one(value_one: i32, value_two: i32, numbers: &[i32]) -> i32 {
    let mut numbers_copy = numbers.to_owned();
    numbers_copy[1] = value_one;
    numbers_copy[2] = value_two;
    let mut result: i32 = 0;
    for i in (0..numbers_copy.len()).step_by(4) {
        let mut stop_loop = false;

        let change_position = numbers_copy[i + 3];
        let x = numbers_copy[i + 1];
        let y = numbers_copy[i + 2];

        match numbers_copy[i] {
            1 => {
                numbers_copy[change_position as usize] =
                    numbers_copy[x as usize] + numbers_copy[y as usize];
            }
            2 => {
                numbers_copy[change_position as usize] =
                    numbers_copy[x as usize] * numbers_copy[y as usize];
            }
            99 => {
                result = numbers_copy[0];
                stop_loop = true;
            }
            _ => {}
        }
        if stop_loop {
            break;
        }
    }
    result
}

fn part_two(numbers: Vec<i32>) {
    for noun in 0..100 {
        for verb in 0..100 {
            let output = part_one(noun, verb, &numbers);
            if output == 19690720 {
                println!("{}", (100 * noun) + verb);
                std::process::exit(0);
            }
        }
    }
}

fn main() {
    let numbers: Vec<i32> = BufReader::new(File::open("dayTwo.txt").expect("file not found"))
        .lines() // Go through each line
        .next() // Only take the first line
        .unwrap() // Unwrap the option of whether there was a next line
        .unwrap() // Unwrap the string result of the lines
        .split(',') // Split by commas
        .map(|number| {
            number
                .parse() // Parse the number
                .expect("could not parse number") // Panic with a message if it fails
        })
        .collect();
    let part_one_vec = numbers.clone();

    println!("{}", part_one(12, 2, &part_one_vec));
    part_two(numbers);
}
