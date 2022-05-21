fn main() {
    let mut count = 0;
    //loop lable
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {}", count);

}

fn return_values() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            //return value after break keyword
            break counter * 2;
        }
    };

    println!("The result is {}", result);
}

fn while_loops() {
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("LIFTOFF!!!");
}

fn loop_through_array () {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {}", element);
    }
}

fn loop_in_range () {
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}