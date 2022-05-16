fn main() {
    //Scalar Types
    //A scalar type represents a single value. Rust has four primary scalar types: integers, floating-point numbers, Booleans, and characters. You may recognize these from other programming languages. Let’s jump into how they work in Rust.
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x is {}", x);
    }
    println!("The value of x is {}", x);

    // floating point
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32

    //numeric operations
    
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;
    
    // multiplication
    let product = 4 * 30;
    
    // division
    let quotient = 56.7 / 32.2;
    let floored = 2 / 3; // Results in 0
    
    // remainder
    let remainder = 43 % 5;

    //booleans
    let t = true;
    let f: bool = false; // with explicit type annotation

    //character type
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';

    //Compound Types

    //Tuple type
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {}", y);
    // can also access through dot notation & index of value
    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;

    //Array type
    let a = [1, 2, 3, 4, 5];
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    //You can also initialize an array to contain the same value for each element by specifying the initial value, followed by a semicolon, and then the length of the array in square brackets, as shown here:
    let a = [3; 5]; // [3, 3, 3, 3, 3]
    let first = a[0];
    let second = a[1];
}



