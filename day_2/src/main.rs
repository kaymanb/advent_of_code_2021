use std::fs::File;
use std::io::{BufRead, BufReader};

fn calculate_position(filename: &str) -> (i32, i32) {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut depth = 0;
    let mut position = 0;
    let mut aim = 0;
    for line in reader.lines() {
        let line  = line.unwrap();
        let mut line = line.split_whitespace();
        let command = line.next().unwrap();
        let dst: i32 = line.next().unwrap().parse().unwrap();
        match command {
            "forward" => {
                position = position + dst;
                depth = depth + aim*dst;
            },
            "up" => aim = aim - dst,
            "down" => aim = aim + dst,
            _ => println!("Oops!"),
        }
    }

    return (depth, position)
}


fn main() {
    let (d, p) = calculate_position("input.txt");
    println!("{}", d*p)
}
