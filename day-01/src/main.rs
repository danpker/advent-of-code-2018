use std::io::{self, Read};
use std::collections::HashSet;

fn part1(input_lines: &str) {
    let mut frequency: i64 = 0;

    for line in input_lines.lines() {
        let parsed_line = line.parse::<i64>().unwrap();
        frequency += parsed_line;
    }

    println!("Part 1 Answer: {}", frequency);
}

fn part2(input_lines: &str) {
    let mut frequency: i64 = 0;
    let mut frequencies = HashSet::new();

    frequencies.insert(frequency);

    loop {
        for line in input_lines.lines() {
            let parsed_line = line.parse::<i64>().unwrap();
            frequency += parsed_line;

            if !frequencies.insert(frequency) {
                println!("Part 2 Answer: {}", frequency);
                return
            }
        }
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    part1(&input);
    part2(&input);
}
