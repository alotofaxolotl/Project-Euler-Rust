fn include_in_sum(n: &i32) -> bool {
    n % 3 == 0 || n % 5 == 0
}

fn main() {
    let result: i32 = (0..1000)
        .filter(|x| include_in_sum(x))
        .sum();

    println!("{}", result);
}
