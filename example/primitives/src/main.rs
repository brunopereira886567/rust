
fn signed_integers() {
    //signed integers
    let _signed_i8: [i8; 2] = [i8::MIN, i8::MAX];
    println!("_signed_i8: MIN:{}--MAX:{}", i8::MIN, i8::MAX);

    let _signed_16: [i16; 2] = [i16::MIN, i16::MAX];
    println!("_signed_16: MIN:{}--MAX:{}", i16::MIN, i16::MAX);

    let _signed_32: [i32; 2] = [i32::MIN, i32::MAX];
    println!("_signed_32: MIN:{}--MAX:{}", i32::MIN, i32::MAX);

    let _signed_64: [i64; 2] = [i64::MIN, i64::MAX];
    println!("_signed_64: MIN:{}--MAX:{}", i64::MIN, i64::MAX);

    let _signed_128: [i128; 2] = [i128::MIN, i128::MAX];
    println!("_signed_128: MIN:{}--MAX:{}", i128::MIN, i128::MAX);

    let _signed_isize:[isize; 2] = [isize::MIN, isize::MAX];
    println!("_signed_isize: MIN:{}--MAX:{}", isize::MIN, isize::MAX);


    let a:i8 = 10;
    let b:i8 = 117;
    println!("sum a + b = {}", a + b);
}

fn unsigned_integers() {
    //unsigned integers
    let _unsigned_u8: [u8; 2] = [u8::MIN, u8::MAX]; 
    println!("_unsigned_u8 MIN:{} MAX:{}", u8::MIN, u8::MAX);
    let _unsigned_u16: [u16; 2] = [u16::MIN, u16::MAX]; 
    println!("_unsigned_u16 MIN:{} MAX:{}", u16::MIN, u16::MAX);
    let _unsigned_u32: [u32; 2] = [u32::MIN, u32::MAX]; 
    println!("_unsigned_u32 MIN:{} MAX:{}", u32::MIN, u32::MAX);
    let _unsigned_u64: [u64; 2] = [u64::MIN, u64::MAX]; 
    println!("_unsigned_u64 MIN:{} MAX:{}", u64::MIN, u64::MAX);
    let _unsigned_u128: [u128; 2] = [u128::MIN, u128::MAX]; 
    println!("_unsigned_u128 MIN:{} MAX:{}", u128::MIN, u128::MAX);
    let _unsigned_usize: [usize; 2] = [usize::MIN, usize::MAX]; 
    println!("_unsigned_usize MIN:{} MAX:{}", usize::MIN, usize::MAX);
    println!("---------------------");
}
fn float(){
    println!("f32: MIN:{}", f32::MIN);
    println!();
    println!("f32: MIN:{}", f64::MIN);
    println!();
    println!("f32: MAX:{}", f32::MAX);
    println!();
    println!("f64: MAX:{}", f64::MAX);
    println!();
}

fn char(){
 let  char1:char = 'a'; 
 let  char2:char = 'º';   
 println!("{}", char1);
 println!("{}", char2);
}

fn bool(){
    let bool1: bool = true;
    let bool2: bool = false;
    println!("{}", bool1);
    println!("{}", bool2);
}

fn compound_types(){
    let _array: [i8; 5] = [1, 2, 3, 4, 5];  
    let _tuples: (i8, bool) = (2, false);

    let mut _tuples: (i8, String, bool, f32) = (1, "Tupla Teste".to_string(), false, 1.0);
    println!("value tupla 1 = {}", _tuples.0);
    println!("value tupla 2 = {}", _tuples.1);
    println!("value tupla 3 = {}", _tuples.2);
    println!("value tupla 4 = {}", _tuples.3);
    
}
fn array_and_slice(){
    //slice
    let mut x = [1, 2, 3];
    let x = &mut x[..];
    x[1] = 7 ;
    assert_eq!(x, &[1, 7, 3]);

    let _slice = [100, 200, 3300, 100000];
    assert_eq!(Some(&100), _slice.first());

    let _blanck_slice: &[i32] = &[];
    assert_eq!(None, _blanck_slice.first());

    //array

    let mut _array: [i8; 5] = [0; 5];
    _array[1] = 100;
    _array[2] = 127;
    assert_eq!([0,100,127, 0, 0], &_array[0..]);
    for x in _array.iter() { println!("{}", x) }
}

fn main() {
    signed_integers();
    unsigned_integers();
    float();
    char();
    bool();
    compound_types();
    array_and_slice();
}
