fn prime_sieve(max: usize) -> Vec<bool> {
    let mut is_prime: Vec<bool> = vec![true; max];
    is_prime[0] = false;
    is_prime[1] = false;

    for i in 2..max {
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
    let target_prime: u32 = prime_sieve(200_000)
        .iter()
        .enumerate()
        .filter(|(_, &value)| value == true)
        .map(|(i, _)| i as u32)
        .collect::<Vec<u32>>()[10000];

    println!("{}", target_prime);
}
