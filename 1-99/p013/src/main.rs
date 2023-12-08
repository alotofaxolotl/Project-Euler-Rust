use std::fs::read_to_string;

fn main() {
    let mut nums: Vec<u64> = Vec::new();
    for line in read_to_string("data/input.txt").unwrap().lines() {
        nums.push(line[0..12].parse::<u64>().unwrap());
    }

    let result: &str = &nums.iter()
        .sum::<u64>()
        .to_string()[0..10];

    println!("{}", result);
}
