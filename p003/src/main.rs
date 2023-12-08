const N: u64 = 600851475143;

fn prime_sieve(max: &u64) -> Vec<bool> {
    let max = max.clone();
    let mut is_prime: Vec<bool> = vec![true; max as usize];
    is_prime[0] = false;
    is_prime[1] = false;

    for i in 0..max as usize {
        if !is_prime[i] {
            continue;
        }
        for j in ((i*i)..max as usize).step_by(i) {
            is_prime[j] = false;
        }
    }

    is_prime
}

fn main() {
    let sqrt_n: u64 = (N as f64).sqrt() as u64 + 1;
    let largest_prime_factor = prime_sieve(&sqrt_n)
        .iter()
        .enumerate()
        .filter(|(_, &value)| value == true)
        .map(|(i, _)| i as u64)
        .filter(|&n| N % n == 0)
        .last()
        .unwrap();

    println!("{}", largest_prime_factor);
}
