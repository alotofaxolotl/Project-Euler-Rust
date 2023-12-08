fn main() {
    let mut grid: Vec<Vec<u64>> = vec![vec![1 as u64; 21]; 21];
    for y in 1..=20 as usize {
        for x in 1..=20 as usize {
            grid[y][x] = grid[y-1][x] + grid[y][x-1];
        }
    }
    println!("{}", grid[20][20]);
}
