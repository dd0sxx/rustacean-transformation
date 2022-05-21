fn main() {
    let mut x = 0;
    while x < 12 {
        let a = recursive_fibonacci(x);
        println!("{} Fibonacci number is {}", x, a);
        x += 1;
    }
}

fn recursive_fibonacci (n: u128) -> u128 {
    if n <= 1 {
        return n
    } 
    return recursive_fibonacci(n-1) + recursive_fibonacci(n-2);
}