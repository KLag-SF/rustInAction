const BIAS: i32 = 127;
const RADIX:f32 = 2.0;

fn main() {
    let n: f32 = 42.42;

    let (sign, exp, frac) = to_parts(n);
    let (sign_, exp_, mant) = decode(sign, exp, frac);
    let n_ = from_parts(sign_, exp_, mant);

    println!("{} -> {}", n, n_);
    println!{"Field\t|Bit\t|RealNo"};
    println!("Sign\t|{:01b}\t|{}", sign, sign_);
    println!("Exp\t|{:08b}\t|{}", exp, exp_);
    println!("Mattise\t|{:023b}|{}", frac, mant);
}

fn to_parts(n: f32)-> (u32, u32, u32) {
    let bits = n.to_bits();
    let sign = (bits >> 31) & 1;
    let exp = (bits >> 23) & 0xff;
    let frac = bits & 0x7fffff;

    (sign, exp, frac)
}

fn decode(sign:u32, exp:u32, frac:u32) -> (f32, f32, f32){
    let signed_1 = (-1.0_f32).powf(sign as f32);
    let mut mantissa: f32 = 1.0;
    let exp = (exp as i32) - BIAS;
    let exp = RADIX.powf(exp as f32);

    for i in 0..23{
        let mask = 1 << i;
        let one_at_bit_i = frac & mask;
        if one_at_bit_i != 0{
            let i_ = i as f32;
            let weight = 2_f32.powf(i_ - 23.0);
            mantissa += weight;
        }
    }

    (signed_1, exp, mantissa)
}

fn from_parts(sign: f32, exp: f32, mantissa: f32) -> f32{
    sign * exp * mantissa
}