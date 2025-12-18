use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
    convert::TryInto,
};

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn get_largest_battery_pos(bank: String) -> u32 {
    let mut index_of_largest_num: u32 = 0;
    let mut largest_seen_num: char = '0';

    for i in 0..bank.len() {
        let num_to_check: char = bank.chars().nth(i).unwrap();

        if num_to_check == '9' {
            return i as u32;
        }

        if num_to_check > largest_seen_num {
            index_of_largest_num = i as u32;
            largest_seen_num = num_to_check;
        }
    }
    return index_of_largest_num;
}

fn main() {
    let lines = lines_from_file("../input.txt");
    let mut count: u64 = 0;
    for line in lines {
        // Get tenth digit
        let first_line = &line[..line.len()-2];
        let largest_battery_pos: u32 = get_largest_battery_pos(first_line.to_string());
        let tenth_digit: char = line.chars().nth(largest_battery_pos.try_into().unwrap()).unwrap();

        // Get oneth digit
        let sub_line = &line[largest_battery_pos.try_into().unwrap()..];
        let largest_battery_pos: u32 = get_largest_battery_pos(sub_line.to_string());
        let oneth_digit: char = line.chars().nth(largest_battery_pos.try_into().unwrap()).unwrap();

        // Get the largest num
        let mut largest_num: String = "".to_string();
        largest_num.push(tenth_digit);
        largest_num.push(oneth_digit);

        // Add em up
        count = count + (largest_num.parse::<u64>().unwrap());
    }
    println!("{}", count);
}
