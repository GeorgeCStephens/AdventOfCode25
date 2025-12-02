use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn main() {
    let mut count: u32 = 0;
    let mut dial: i32 = 50;

    let lines = lines_from_file("../input.txt");
    for line in lines {
        //Get the direction and amount
        let direction: String = line.chars().take(1).collect();
        let string_amount: String = line.chars().skip(1).collect();
        let amount = string_amount.parse::<i32>().unwrap();

        for _i in 1..amount+1 {
            if direction == "L" {
                dial = (dial - 1 + 100) % 100;
            } else {
                dial = (dial + 1) % 100;
            }
            if dial == 0 {
                count = count + 1;
            }
        }

        println!("{} : {}", direction, amount);
    }

    println!("Total count is: {}", count);
}
