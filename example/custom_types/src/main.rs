#[derive(Debug)]
struct Person{
    _name: String,
    _age:u8
}

//unit
struct Unit;


//tuple
struct Pair(i32, f32);

struct Point{
    x:f32,
    y:f32
}

struct Rectangle{
    top_left: Point,
    bottom_right: Point
}

fn main() {
    let _name = String::from("Bruno Pereira");
    let _age = 32;
    let _bruno = Person{_name, _age};
    println!("{:?}", _bruno);

    let _point: Point = Point{ x: 100.3, y: 0.4};
    println!("Points x{} and y{}", _point.x, _point.y);

    let bottom_right = Point {x: 0.5, .._point};
    println!("Bottom Right x{} and y{}", bottom_right.x, bottom_right.y);
}
