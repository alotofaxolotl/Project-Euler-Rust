use std::collections::HashMap;

fn next_collatz(n: &u64) -> u64 {
    if n % 2 == 0 {
        return n / 2;
    }
    3 * n + 1
}

fn main() {
    let mut collatz_map: HashMap<u64, u64> = HashMap::new();
    for n in 1..1_000_000 as u64 {
        let mut chain_length: u64 = 1;
        let mut c = n;
        while c != 1 {
            c = next_collatz(&c);
            if collatz_map.contains_key(&c) {
                chain_length += collatz_map.get(&c).unwrap();
                break;
            }
            chain_length += 1;
        }
        collatz_map.insert(n, chain_length);
    }

    let max_n: u64 = collatz_map.iter()
        .max_by(|a, b| a.1.cmp(&b.1))
        .map(|(k, _v)| k)
        .unwrap()
        .to_owned();

    println!("{}", max_n);
}
