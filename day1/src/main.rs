use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn calculate(num: &i32) -> i32 {
    (num / 3) - 2
}

fn part_one(numbers: &[i32]) -> i32 {
    let fuel_needed = numbers.iter().map(calculate).sum();
    fuel_needed
}

fn part_two(numbers: &Vec<i32>) -> i32 {
    let mut total_fuel = 0;
    for num in numbers {
        let mut fuel: i32 = calculate(num);
        while fuel > 0 {
            total_fuel += fuel;
            fuel = calculate(&fuel);
        }
    }
    total_fuel
}

fn main() {
    let numbers: Vec<i32> = BufReader::new(File::open("day1.txt").expect("file not found"))
        .lines() // Go through each line
        .map(|line| {
            line.expect("could not parse line") // Unwrap the result of the line
                .parse() // Try to parse it to what we expect (i32 from the annotation)
                .expect("could not parse number") // Unwrap the result of the parse
        })
        .collect();

    println!("Part one total: {}", part_one(&numbers));
    println!("Part two total: {}", part_two(&numbers));
}
