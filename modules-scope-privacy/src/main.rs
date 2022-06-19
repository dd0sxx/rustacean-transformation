use crate::garden::vegetables::Asparagus;

pub mod garden;

fn main() {
    let plant = Asparagus {quantity: 10, kind: String::from("White")};
    println!("I'm growing {} {} aspargus plants", plant.quantity, plant.kind);
}
