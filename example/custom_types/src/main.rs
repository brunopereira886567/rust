use crate::List::*;

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

enum WebEvent {
    PageLoad,
    PageUnload,
    KeyPress(char),
    Paste(String),
    Click {x: i64, y: i64},
}

fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("page Loaded"),
        WebEvent::PageUnload => println!("page Unloaded"),
        WebEvent::KeyPress(c) => println!("pressed '{}'", c),
        WebEvent::Paste(s) => println!("psted '{}'", s),
        WebEvent::Click {x, y} => {
            println!("clicked at x={}, y={}", x, y)
        },
    }
}

enum VerboseEnum {
    Add,
    Subtract
}

impl VerboseEnum {
    fn run(&self, x:i32, y:i32) -> i32 {
        match self {
            Self::Add => x + y,
            Self::Subtract => x  - y,
        }
    }
}

type Operations = VerboseEnum;

enum Color {
    Red = 0xff0000,
    Green = 0x00ff00,
    Blue = 0x0000ff,
}
use crate::List::*;

enum List {
    Cons(u32, Box<List>),
    Nil,
}

impl List {
    fn new() -> List {
        Nil
    }

    fn prepend(self, elem: u32) -> List {
        Cons(elem, Box::new(self))
    }

    fn len(&self) -> u32 {
        match *self {
            Cons(_, ref tail) => 1 + tail.len(),
            Nil => 0
        }
    }

    fn stringify(&self) -> String {
        match *self {
            Cons(head, ref tail) => {
                format!("{}, {}", head, tail.stringify())
            },
            Nil => {
                format!("Nil")
            },
        }
    }
}

static LANGUAGE: &str = "Rust";
const THRESHOLD: i32 = 10;

fn is_big(n: i32) -> bool {
    n > THRESHOLD
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

    let pressed = WebEvent::KeyPress('X');
    let pasted  = WebEvent::Paste("MY TEXT".to_owned());
    let click   = WebEvent::Click { x: 90, y: 100 };
    let load    = WebEvent::PageLoad;
    let unload  = WebEvent::PageUnload;

    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);
    let x = Operations::Add.run(4, 5);
    println!("Sum x{} + y{} = {}", 4, 5, x);

    use crate::Color::*;

    let _color = Blue;

    let mut list = List::new();

    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);
    println!("linked list has length: {}", list.len());
    println!("{}", list.stringify());

    let n = 16;

    println!("{}", LANGUAGE);

    println!("{}", THRESHOLD);

    println!("{} {}", n, if is_big(n) { "Big"} else {"small"});

}