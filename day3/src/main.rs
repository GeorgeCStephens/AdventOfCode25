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

fn get_largest_battery_pos(bank: Vec<char>) -> i32 {
    let mut index: i32 = 0;
    let mut largest_seen_num : i32 = 0;
    let max_possible_num : i32 = 9;

    for battery in bank {
        let num_to_check: i32 = battery as i32 - 0x30;

        if num_to_check == max_possible_num {
            return index;
        }

        if num_to_check > largest_seen_num {
            largest_seen_num = num_to_check;
        }

        index = index + 1;
    }
    return index;
}

fn main() {
    let lines = lines_from_file("../input.txt");
    for line in lines {
        let bank:Vec<char> = line.chars().collect();
        let largest_battery_pos: i32 = get_largest_battery_pos(bank);
        println!("{} : {}", bank[largest_battery_pos], line);
    }
}
