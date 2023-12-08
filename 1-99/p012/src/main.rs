fn count_factors(n: &u32) -> u32 {
    let n = n.clone();
    let sqrt_n = (n as f64).sqrt() as u32 + 1;
    let mut factors: u32 = 0;

    for i in 1..=sqrt_n {
        if n % i == 0 {
            factors += 2;
        }
    }

    factors
}

fn main() {
    let mut i: u32 = 1;
    let mut t: u32 = 0;

    loop {
        t += i;
        i += 1;

        if count_factors(&t) > 500 {
            println!("{}", t);
            break;
        }
    }
}
