
fn casting(){
    let decimal = 65.4333_f32;
    // let integer: u8 = decimal; //ERROR
    let _integer = decimal as u8;

    let _character = _integer as char;
    // let _character = decimal as char; // invalid cast

    println!("Casting: {} -> {} -> {}", decimal, _integer, _character);
    println!("1000 as a u16 is: {}", 1000 as u16);
    println!("1000 mod 256 is : {}", 1000 % 256);
    println!(" 128 as a i16 is: {}", 128 as i16);
    unsafe {
        println!("300.0 is {}", 300.0_f32.to_int_unchecked::<u8>());
        println!("-100.0 as u8 is {}", (-100.0_f32).to_int_unchecked::<u8>());
        println!("nan as u8 is {}", f32::NAN.to_int_unchecked::<u8>());
    }
}

fn literals(){
    let x = 1u8;
    let y = 2u32;
    let z = 3f32;

    let i = 1;
    let f = 1.0;

    println!("size of `x` in bytes: {}", std::mem::size_of_val(&x));
    println!("size of `y` in bytes: {}", std::mem::size_of_val(&y));
    println!("size of `z` in bytes: {}", std::mem::size_of_val(&z));
    println!("size of `i` in bytes: {}", std::mem::size_of_val(&i));
    println!("size of `f` in bytes: {}", std::mem::size_of_val(&f));
}


fn main() {
    casting();
    literals();
}


