#![allow(overflowing_literals)]

fn main() {
    let decimal = 65.4321_f32;

    // let integer: u8 = decimal;

    let integer: u8 = decimal as u8;

    println!("{}", integer);

    let character = integer as char;

    println!("{}", character);

    // let character = decimal as char;

    println!("Casting: {} -> {} -> {}", decimal, integer, character);

    println!("1000 as a u16 is: {:016b}", 5555 as u16);

    println!("1000 as a u8 is : {:016b}", 5555 as u8);

    println!("  -1 as a u8 is : {}", (-1i8) as i8);
    println!("  -1 as a u8 is : {}", (-1i8) as u8);
    println!("  -1 as a u8 is : {:016b}", (-1i8) as i8);
    println!("  -1 as a u8 is : {:016b}", (-1i8) as u8);

    println!("1000 mod 256 is : {}", 1000 % 256);

    println!(" 128 as a i16 is: {}", 128 as i16);
    println!(" 128 as a i8 is : {}", 128 as i8);
    println!(" 128 as a i16 is: {:016b}", 128 as i16);
    println!(" 128 as a i8 is : {:016b}", 128 as i8);
}
