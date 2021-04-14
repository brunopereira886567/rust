fn mutability() {
    let _immutable = 1;
    let mut mutable = 1;
    println!("Before Mutation {} ", mutable);
    mutable += 1;
    println!("after Mutation {} ", mutable);

    // _immutable += 1;
}
fn scope_and_shadowing() {
    let long_lived_binding = 1;

    {
        let short_lived_binding = 2;
        println!("short:{}", short_lived_binding);
    }

    // println!("outer short: {}", short_lived_binding);
    println!("outer long: {}", long_lived_binding);

    let shadowed_binding = 1;
    {
        println!("before being shadowed: {}", shadowed_binding);
        let shadowed_binding = "abc";
        println!("shadowed in inner block: {}", shadowed_binding);
    }
    println!("outside inner block: {}", shadowed_binding);

    let shadowed_binding = 2;
    println!("outer block: {}", shadowed_binding);


    let sum = 0;
    let a = 1;
    {
        let b = 1;
        println!("{}", b);
    }

    // println!("{}", b); // ERROR

    println!("{}", sum); 
}

fn declare_first(){
    let a_binding;

    {
        let x = 2;
        a_binding = x * x;
    }

    println!("a binding: {}", a_binding);

    let another_binding;

    // println!("another binding: {}", another_binding); //ERROR

    another_binding = 1;

    println!("another binding: {}", another_binding);
}

fn freezing(){
    let mut _mutable_integer = 7i32;

    {
        let _mutable_integer = _mutable_integer;

        // Error! `_mutable_integer` is frozen in this scope
        // _mutable_integer = 50;
    }

    _mutable_integer = 3;
}


fn main() {
    mutability();
    scope_and_shadowing();
    declare_first();
    freezing();
}
