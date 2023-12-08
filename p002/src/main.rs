const LIMIT: u32 = 4_000_000;

fn main() {
   let mut fib: Vec<u32> = vec![1, 1];
   while fib[0] + fib[1] < LIMIT {
       fib.insert(0, fib[0] + fib[1]);
   }
   
   let even_sum: u32 = fib.iter()
       .filter(|&x| x % 2 == 0)
       .sum();

   println!("{}", even_sum);
}
