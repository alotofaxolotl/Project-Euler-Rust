fn is_palindrome(n: &u32) -> bool {
    let n_str = n.to_string();
    let rev_n_str = n_str.chars().rev().collect::<String>();
    n_str == rev_n_str
}

fn main() {
    let mut palindromes: Vec<u32> = Vec::new();
    for i in 900..1000 as u32 {
        for j in i..1000 {
            let p = i * j;
            if is_palindrome(&p) {
                palindromes.push(p);
            }
        }
    }

    palindromes.sort();
    println!("{}", palindromes.last().unwrap());
}
