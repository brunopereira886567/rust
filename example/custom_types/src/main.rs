#[derive(Debug)]
struct Person{
    _name: String,
    _age:u8
}

struct Point{
    x:f32,
    y:f32
}

struct Rectangle{
    top_left: Point,
    bottom_right: Point
}

fn rect_area(rec: Rectangle) -> f32{
    let result: f32 ;
    result = rec.top_left.x * rec.bottom_right.y;
    result
}

fn square(point: Point, y: f32) -> Rectangle{
    let base = point.x;
     let _area = Rectangle {
        top_left: point,
        bottom_right: Point {x:y , y: base}       
    };
    _area
}

fn main() {
    let _name = String::from("Bruno Pereira");
    let _age = 32;
    let _bruno = Person{_name, _age};
    println!("{:?}", _bruno);
    
    let _point: Point = Point{ x: 10.0, y: 5.0};
    let Point { x: top_edge, y: left_edge } = _point;
    let bottom_right = Point {x: 10.0, .._point};

    let _rectangle = Rectangle {
        top_left: Point {x: top_edge, y: left_edge},
        bottom_right: bottom_right        
    };

    println!("area of a rectangle:{}", rect_area(_rectangle));
    println!("area of a rectangle by square:{}", rect_area(square( Point {x: 10.0, y: 5.0}, 0.5)));

}
