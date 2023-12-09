fn divisor_sum(n: &u32) -> u32 {
    let n = n.clone();
    let mut sum = 1;
    for i in 2..(n as f64).sqrt() as u32 + 1 {
        if n % i == 0 {
            sum += i + (n / i);
        }
    }
    sum
}

fn main() {
    let mut sum: u32 = 0;
    for n in 1..10000 as u32 {
        let d_s = divisor_sum(&n);
        if d_s == n {
            continue;
        }
        if divisor_sum(&d_s) == n {
            sum += n;
        }
    }
    println!("{}", sum);
}
