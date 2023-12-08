use std::fs::read_to_string;

fn main() {
    let mut nums: Vec<Vec<i32>> = Vec::new();
    for line in read_to_string("data/nums.txt").unwrap().lines() {
        nums.push(line.to_string()
                  .split(' ')
                  .map(|n| n.parse::<i32>().unwrap())
                  .collect());
    }

    let mut best: i32 = 0;
    for i in 0..nums.len() {
        for j in 0..nums[i].len() - 4 {
            let p: i32 = nums[i][j..j+4]
                .iter()
                .product();
            best = std::cmp::max(p, best);

            let mut p: i32 = 1;
            for x in 0..4 {
                p *= nums[j+x][i];
            }
            best = std::cmp::max(p, best);
        }
    }

    for i in 0..nums.len() - 4 {
        for j in 0..nums[i].len() - 4 {
            let mut p1: i32 = 1;
            let mut p2: i32 = 1;
            for x in 0..4 {
                p1 *= nums[i+x][j+x];
                p2 *= nums[i+x][j+3-x];
            }
            best = std::cmp::max(best, std::cmp::max(p1, p2));
        }
    }
    
    println!("{}", best);
}
