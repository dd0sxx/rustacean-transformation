fn main() {
    let a = fahrenheit_to_celcius(32 as f32);
    println!("32 degrees fahrenheit in celsius is {}", a);
    let b = fahrenheit_to_celcius(100 as f32);
    println!("100 degrees fahrenheit in celsius is {}", b);
}


fn fahrenheit_to_celcius (f:f32) -> f32 {
    let c = f - 32 as f32;
    let ratio: f32 = 5.0/9.0;
    c * ratio
}
