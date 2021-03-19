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

fn rect_area(rec: Rectangle) -> f32 {
    let result = rec.top_left.x * rec.bottom_right.y;
    result
}

fn square(point: Point, y: f32) -> Rectangle{
     let _area = Rectangle {
         bottom_right: Point {x: point.x , y: y}       ,
         top_left: Point {x: point.x , y: y}
    };
    _area
}

fn main() {

    let _name = String::from("Bruno Pereira");
    let _age = 32;
    let _bruno = Person{_name, _age};
    println!("{:?}", _bruno);
    
    let _point = Point{ x: 10.0, y: 5.0};
    let Point { x: top_edge, y: left_edge } = _point;
    let bottom_right = Point {x: 10.0, .._point};

    let _rectangle = Rectangle {
        top_left: Point {x: top_edge, y: left_edge},
        bottom_right: bottom_right        
    };

    println!("area of a rectangle:{}", rect_area(_rectangle));
    let _rectangle = square( Point {x: 10.0, y: 10.0}, 0.5);
    println!("area of a rectangle by square:{}", rect_area(_rectangle));

    
}
