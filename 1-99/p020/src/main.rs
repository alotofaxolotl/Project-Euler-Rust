fn main() {
    let mut digits: Vec<i32> = vec![1];
    let mut carry: i32;
    for n in 2..=100 {
        carry = 0;
        for d in 0..digits.len() {
            digits[d] = digits[d] * n + carry;
            carry = digits[d] / 10;
            digits[d] = digits[d] % 10;
        }
        while carry != 0 {
            digits.push(carry % 10);
            carry /= 10;
        }
    }

    println!("{}", digits.iter().sum::<i32>());
}
