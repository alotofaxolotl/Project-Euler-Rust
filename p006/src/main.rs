fn main() {
    let sum_of_squares: i64 = (1..=100 as i64)
        .map(|x| x.pow(2))
        .sum();

    let mut square_of_sum: i64 = (1..=100 as i64)
        .sum();
    square_of_sum = square_of_sum.pow(2);

    println!("{}", (sum_of_squares - square_of_sum).abs());
}
