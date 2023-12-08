fn prime_sieve(max: usize) -> Vec<bool> {
    let mut is_prime: Vec<bool> = vec![true; max];
    is_prime[0] = false;
    is_prime[1] = false;

    for i in 0..max {
        if !is_prime[i] {
            continue;
        }
        for j in ((i*i)..max).step_by(i) {
            is_prime[j] = false;
        }
    }

    is_prime
}

fn main() {
    let sum: u128 = prime_sieve(2_000_000)
        .iter()
        .enumerate()
        .filter(|(_, &value)| value == true)
        .map(|(i, _)| i as u128)
        .sum();

    println!("{}", sum);
}
