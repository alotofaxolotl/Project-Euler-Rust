fn main() {
    let mut gcd: u64 = 1;
    for i in 1..=20 as u64 {
        let _gcd = gcd;
        while gcd % i != 0 {    // while i does not divide the gcd
            gcd += _gcd;        // increment the gcd by gcd(1..i-1)
        }                       // now the numbers (1..1) divide gcd
    }
    println!("{}", gcd);
}
