fn main() {
    let mut digits: Vec<i32> = vec![1];
    let mut carry: i32;
    for _ in 1..=1000 {
        carry = 0;
        for d in 0..digits.len() {
            digits[d] = digits[d] * 2 + carry;
            carry = digits[d] / 10;
            digits[d] = digits[d] % 10;
        }
        if carry != 0 {
            digits.push(carry);
        }
    }

    println!("{}", digits.iter().sum::<i32>());
}
