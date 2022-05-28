fn main() {
    let scale = 2;
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );
    println!(
        "The area of the rectangle is {} square pixels.",
        area2((width1, height1))
    );
    
    let rect1 = Rectangle {
        width: dbg!(width1 * scale), //dbg returns ownership to width
        height: height1
    };
    println!("rect1 is {:?}", rect1);

    //another way to print
    dbg!(&rect1); // need reference to rect1 so dbg doesn't take ownership
    
    println!(
        "The area of the rectangle is {} square pixels.",
        area3(&rect1)
    );
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

//params are unrelated, without documentation nothing indicates that height and width are related
fn area (width: u32, height: u32) -> u32 {
    width * height
}

//refactoring with tuples
fn area2 (rect: (u32, u32)) -> u32 {
    rect.0 * rect.1
}

//refactoring with struct
fn area3 (rect: &Rectangle) -> u32 {
    rect.width * rect.height
}
