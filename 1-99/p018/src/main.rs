use std::fs::read_to_string;

fn main() {
    let mut triangle: Vec<Vec<i32>> = Vec::new();
    for line in read_to_string("data/triangle.txt").unwrap().lines() {
        triangle.push(line
            .split(' ')
            .map(|c| c.parse::<i32>().unwrap())
            .collect());
    }

    for i in (0..triangle.len() - 1).rev() {
        for j in 0..triangle[i].len() {
            triangle[i][j] += std::cmp::max(triangle[i+1][j], triangle[i+1][j+1]);
        }
    }

    println!("{}", triangle[0][0]);
}
