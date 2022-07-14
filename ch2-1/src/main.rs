use std::convert::TryInto;
use num::complex::Complex;

fn main() {
    let a: i32 = 10;
    let b: u16 = 100;

    let b = b.try_into().unwrap();
    
    if a < b{
        println!("10 < 100:")
    }

    let result: f64 = 0.1 + 0.1;
    let desired: f64 = 0.2;
    let abs_diff = (desired - result).abs();

    println!("{}", abs_diff <= f64::EPSILON);

    // num::complex::Complex複素数も利用可能。
    let c = Complex{re:2.1, im:1.2};
    println!("{} + {}i", c.re, c.im)
}
