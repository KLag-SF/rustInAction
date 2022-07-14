use std::convert::TryInto;

fn main() {
    let a: i32 = 10;
    let b: u16 = 100;

    let b = b.try_into().unwrap();
    
    if a < b{
        println!("10 < 100:")
    }
}
