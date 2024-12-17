fn main() {
    example1();
    example2();
    example4();
    example5();
    example6();
    example7();
}

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

struct Unit;

struct Pair(i32, f32);

struct Point {
    x: f32,
    y: f32,
}

#[allow(dead_code)]
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

impl Rectangle {
    fn rect_area(&self) -> f32 {
        let Point{x: x1, y: y1} = self.top_left;
        let Point{x: x2, y: y2} = self.bottom_right;

        (x1 - x2).abs() * (y1 - y2).abs()
    }

    fn square(&self, point: Point, length: f32) -> Rectangle {
        let top_left_ = Point {x: point.x - length, y: point.y + length};
        Rectangle {top_left: top_left_, bottom_right: point}
    }
}

enum WebEvent {
    PageLoad,
    PageUnload,
    KeyPress(char),
    Paste(String),
    Click {x: i64, y: i64}
}

enum VeryVerboseEnumOfThingsToDoWithNumbers {
    Add,
    Subtract,
}

type Operations = VeryVerboseEnumOfThingsToDoWithNumbers;

impl VeryVerboseEnumOfThingsToDoWithNumbers {
    fn run(&self, x: i32, y: i32) -> i32 {
        match self {
            Self::Add => x + y,
            Self::Subtract => x - y,
        }
    }
}

fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("page loaded"),
        WebEvent::PageUnload => println!("page unloaded"),
        WebEvent::KeyPress(c) => println!("pressed '{}'.", c),
        WebEvent::Paste(s) => println!("pasted \"{}\".", s),
        WebEvent::Click { x, y } => {
            println!("clicked at x={}, y={}.", x, y);
        },
    }
}

fn example1() {
    let name = String::from("Peter");
    let age = 27;
    let peter = Person {name, age};

    println!("{:?}", peter);

    let point: Point = Point {x: 10.3, y: 0.4};  // 实例化结构体

    println!("point coordinates: ({}, {})", point.x, point.y);

    let bottom_right = Point {x: 5.2, ..point};  // 使用结构体更新语法创建新的point, 这样可以用到之前的piont的字段
    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

    let Point {x: left_edge, y: top_edge} = point;

    let _rectangle = Rectangle {
        top_left: Point {x: left_edge, y: top_edge},
        bottom_right: bottom_right,
    };

    println!("_rectangle area is {}", _rectangle.rect_area());

    let _unit = Unit;

    let pair = Pair(1, 0.1);

    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);
}

fn example2() {
    let pressed = WebEvent::KeyPress('x');
    let pasted  = WebEvent::Paste("my text".to_owned());
    let click   = WebEvent::Click {x: 20, y: 80};
    let load    = WebEvent::PageLoad;
    let unload  = WebEvent::PageUnload;

    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);
}

fn example3() {
    let x = Operations::Add;
}

enum Status {
    Rich,
    Poor,
}

enum Work {
    Civilian,
    Soldier,
}

fn example4() {
    use Status::{Poor, Rich};  // 显式地'use'各个名称使他们直接可用，而不需要指定它们来自‘Status’
    use Work::*;  // 自动地‘use’‘work’内部的各个名称

    let status = Poor;
    let work = Civilian;

    match status {  // 这里没有使用完整路径
        Rich => println!("The rich have lots of money!"),
        Poor => println!("The poor have no money..."),
    }

    match work {  // 这里也没有用完整路径
        Civilian => println!("Civilians work!"),
        Soldier => println!("Soldiers fight!"),
    }
}

// enum的C风格用法

// 拥有隐式辨别值（implicit discriminator，从 0 开始）的 enum
enum Number {
    Zero,
    One,
    Two,
}

// 拥有显式辨别值（explicit discriminator）的 enum
enum Color {
    Red = 0xFF0000,
    Green = 0x00FF00,
    Blue = 0x0000FF,
}

fn example5() {
    // 'enum' 可以转换成整型
    println!("zero is {}", Number::Zero as i32);
    println!("one is {}", Number::One as i32);

    println!("roses are #{:06x}", Color::Red as i32);
    println!("violets are #{:06x}", Color::Blue as i32);
}

// 链表
use List::*;

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
                // 'format!' 和 'print!'类似，但返回的是一个堆分配的字符串，而不是打印结果到控制台上
                format!("{}, {}", head, tail.stringify())
            },
            Nil => {
                format!("Nil")
            },
        }
    }
}

fn example6() {
    let mut list = List::new();

    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);

    println!("linked list has length: {}", list.len());
    println!("{}", list.stringify());
}

static LANGUAGE: &'static str = "Rust";  // 具有 'static 生命周期的，可以是可变的变量（须使用 static mut 关键字）。
const THRESHOLD: i32 = 10;  // 不可改变的值（通常使用这种）

fn is_big(n: i32) -> bool {
    n > THRESHOLD
}

fn example7() {
    let n = 16;

    println!("This is {}", LANGUAGE);
    println!("The threshold is {}", THRESHOLD);
    println!("{} is {}", n, if is_big(n) {"big"} else {"small"});

    // THRESHOLD = 5; // 报错，不能修改一个‘const’常量
}

