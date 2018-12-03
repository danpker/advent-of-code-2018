use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut frequency = 0;

    for line in stdin.lock().lines() {
        let read_line = line.unwrap();
        let parsed_line = read_line.parse::<i64>().unwrap();
        frequency += parsed_line;
    }

    println!("{}", frequency);
}
