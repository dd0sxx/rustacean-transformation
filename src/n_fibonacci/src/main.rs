fn main() {
    let mut x = 0;
    while x < 12 {
        let a = n_fibonacci(x);
        println!("{} Fibonacci number is {}", x, a);
        x += 1;
    }
}

fn n_fibonacci (n: u128) -> u128 {
    let mut f = 0;
    let mut previous = 0;
    let mut counter = 0;
    while counter <= n {
        if counter == 1 {
            f = 1;
        }
        let nextPrevious = f;
        f = f + previous;
        previous = nextPrevious;
        counter += 1;
    }
    f
}