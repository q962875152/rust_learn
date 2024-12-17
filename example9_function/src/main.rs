fn main() {
    example1();
    example2();
    example3();
    example4();
    example5();
    example6();
    example7();
}

fn is_divisible_by(lhs: u32, rhs: u32) -> bool {
    if rhs == 0 {
        return false;
    }

    lhs % rhs == 0
}

fn fizzbuzz(n: u32) -> () {  // 一个 “不” 返回值的函数。实际上会返回一个单元类型 `()`
    if is_divisible_by(n, 15) {
        println!("fizzbuzz");
    } else if is_divisible_by(n, 3) {
        println!("fizz");
    } else if is_divisible_by(n, 5) {
        println!("buzz");
    } else {
        println!("{}", n);
    }
}

fn fizzbuzz_to(n: u32) {  // 当函数返回 `()` 时，函数签名可以省略返回类型
    for n in 1..=n {
        fizzbuzz(n);
    }
}

struct Point {
    x: f64,
    y: f64,
}

// 实现的代码块，‘Point’的所有方法都在这里给出
impl Point {
    // 这是一个静态方法
    // 静态方法不需要被实例调用
    // 这类方法一般用作构造器
    fn origin() -> Point {
        Point {x: 0.0, y: 0.0}
    }

    // 另一个静态方法
    fn new(x: f64, y: f64) -> Point {
        Point {x: x, y: y}
    }
}

struct Rectangle {
    p1: Point,
    p2: Point,
}

impl Rectangle {
    // 这是一个实例方法
    // '&self'是'self:&Self'的语法糖，其中'Self'是方法调用者的类型
    fn area(&self) -> f64 {
        let Point {x: x1, y: y1} = self.p1;
        let Point {x: x2, y: y2} = self.p2;

        ((x1 - x2) * (y1 - y2)).abs()
    }

    fn perimeter(&self) -> f64 {
        let Point {x: x1, y: y1} = self.p1;
        let Point {x: x2, y: y2} = self.p2;

        2.0 * ((x1 -x2).abs() + (y1 - y2).abs())
    }

    fn translate(&mut self, x: f64, y: f64) {
        self.p1.x += x;
        self.p2.x += x;
        
        self.p1.y += y;
        self.p2.y += y;
    }
}

struct Pair(Box<i32>, Box<i32>);

impl Pair {
    // 'self'为'self: Self'的语法糖
    fn destroy(self) {
        let Pair(first, second) = self;

        println!("Destroying Pair({}, {})", first, second);
    }
}

fn example1() {
    let rectangle = Rectangle {
        p1: Point::origin(),
        p2: Point::new(3.0, 4.0),
    };

    // 注意第一个参数‘&self’是隐式传递的
    println!("Rectangle perimeter: {}", rectangle.perimeter());
    println!("Rectangle area: {}", rectangle.area());

    let mut square = Rectangle {
        p1: Point::origin(),
        p2: Point::new(1.0, 1.0),
    };

    // rectangle.translate(1.0, 1.0);  // 报错，‘rectangle’是不可变的，但这个方法需要一个可变对象

    square.translate(1.0, 1.0);

    let pair = Pair(Box::new(1), Box::new(2));

    pair.destroy();

    // pair.destroy(); // 报错！前面的'destroy'调用'消耗了'pair
}

// 闭包
fn example2() {
    //通过闭包和函数分别实现自增
    fn function(i: i32) -> i32 { i + 1 }

    let closure_annotated = |i: i32| -> i32 { i + 1};
    let closure_inferred = |i| i + 1;

    let i = 1;
    println!("function: {}", function(i));
    println!("closure_annotated: {}", closure_annotated(i));
    println!("closure_inferred: {}", closure_inferred(i));

    let one = || 1;
    println!("closure returning one: {}", one());
}

// 捕获
// 闭包优先通过引用来捕获变量，并且仅在需要时使用其他方式。
fn example3() {
    use std::mem;
    let color = String::from("green");

    // 这个闭包打印 `color`。它会立即借用（通过引用，`&`）`color` 并将该借用和
    // 闭包本身存储到 `print` 变量中。`color` 会一直保持被借用状态直到`print` 离开作用域。

    // `println!` 只需传引用就能使用，而这个闭包捕获的也是变量的引用，因此无需
    // 进一步处理就可以使用 `println!`。
    let print = || println!("'color': {}", color);

    print();

    let _reboorow = &color;
    print();

    // 在最后使用‘print’之后，移动或重新借用都是允许的
    let _color_moved = color;

    let mut count = 0;
    // 这个闭包使 `count` 值增加。要做到这点，它需要得到 `&mut count` 或者
    // `count` 本身，但 `&mut count` 的要求没那么严格，所以我们采取这种方式。
    // 该闭包立即借用 `count`。
    //
    // `inc` 前面需要加上 `mut`，因为闭包里存储着一个 `&mut` 变量。调用闭包时，
    // 该变量的变化就意味着闭包内部发生了变化。因此闭包需要是可变的。
    let mut inc = || {
        count += 1;
        println!("'count': {}", count);
    };

    inc();

    // let _reboorow = &count;  // 因为之后调用闭包，所以仍然可变借用‘count’,试图重新借用将导致错误

    inc();

    let _count_reborrowed = &mut count;  // 闭包不再借用‘&mut count’，因此可以正确地重新借用

    let movable = Box::new(3);

    // `mem::drop` 要求 `T` 类型本身，所以闭包将会捕获变量的值。这种情况下，
    // 可复制类型将会复制给闭包，从而原始值不受影响。不可复制类型必须移动
    // （move）到闭包中，因而 `movable` 变量在这里立即移动到了闭包中
    let consume = || {
        println!("'movable': {:?}", movable);
        mem::drop(movable);
    };

    consume();  // 消耗了该变量，所以该闭包只能调用一次。
    // consume();  // 报错


    let haystack = vec![1, 2, 3];
    // 在竖线 | 之前使用 move 会强制闭包取得被捕获变量的所有权：
    let contains = move |needle| haystack.contains(needle);

    println!("{}", contains(&1));
    println!("{}", contains(&4));
}


// 当以闭包作为输入参数时，必须指出闭包的完整类型，它是通过使用以下 trait 中的一种来指定的
// Fn：表示捕获方式为通过引用（&T）的闭包
// FnMut：表示捕获方式为通过可变引用（&mut T）的闭包
// FnOnce：表示捕获方式为通过值（T）的闭包

// 对闭包所要捕获的每个变量，编译器都将在满足使用需求的前提下尽量以限制最多的方式捕获。

// 例如用一个类型说明为 FnOnce 的闭包作为参数。这说明闭包可能采取 &T，&mut T 或 T 中的一种捕获方式，
// 但编译器最终是根据所捕获变量在闭包里的使用情况决定捕获方式。

// 这是因为如果能以移动的方式捕获变量，则闭包也有能力使用其他方式借用变量。注意反过来就不再成立：
// 如果参数的类型说明是 Fn，那么不允许该闭包通过 &mut T 或 T 捕获变量。

// `F` 必须为一个没有输入参数和返回值的闭包实现 `Fn`
fn apply<F>(f: F)  where 
    F: FnOnce()        {
    f();
}

fn apply_to_3<F>(f: F) -> i32 where
    F: Fn(i32) -> i32               {
    f(3)
}
fn example4() {
    use std::mem;

    let greeting = "hello";
    let mut farewell = "goodbye".to_owned();

    let diary = || {
        // 'greeting'通过引用捕获, 故需要闭包是'Fn'
        println!("I said {}.", greeting);

        // 下文改变了‘farewell’，因而要求闭包通过可变引用捕获它
        // 现在需要‘FnOnce’
        farewell.push_str("!!!");
        println!("Then I screamed {}.", farewell);
        println!("Now I can sleep. zzzzz");

        // 手动调用drop又要求闭包通过值获取‘farewell’.
        // 现在需要‘FnOnce’。
        mem::drop(farewell);
    };

    apply(diary);

    let double = |x| 2 * x;

    println!("3 doubled: {}", apply_to_3(double));
}

// 输入函数
fn call_me<F: Fn()>(f: F) {
    f()
}

fn function() {
    println!("I'm a function!");
}

fn example5() {
    let closure = || println!("I'm a closure!");

    call_me(closure);
    call_me(function);
}

// 输出参数
// 闭包作为输入参数是可能的，所以返回闭包作为输出参数（output parameter）也应该是可能的。
// 然而返回闭包类型会有问题，因为目前 Rust 只支持返回具体（非泛型）的类型。
// 按照定义，匿名的闭包的类型是未知的，所以只有使用impl Trait才能返回一个闭包。

// 除此之外，还必须使用 move 关键字，它表明所有的捕获都是通过值进行的。这是必须的，
// 因为在函数退出时，任何通过引用的捕获都被丢弃，在闭包中留下无效的引用。
fn create_fn() -> impl Fn() {
    let text = "Fn".to_owned();

    move || println!("This is a: {}", text)
}

fn create_fnmut() -> impl FnMut() {
    let text = "FnMut".to_owned();

    move || println!("This is a: {}", text)
}

fn create_fnonce() -> impl FnOnce() {
    let text = "FnOnce".to_owned();

    move || println!("This is a: {}", text)
}

fn example6() {
    let fn_plain = create_fn();
    let mut fn_mut = create_fnmut();
    let fn_once = create_fnonce();

    fn_plain();
    fn_mut();
    fn_once();
}

fn example7() {
    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];

    let mut iter = vec1.iter();
    let mut into_iter = vec2.into_iter();

    println!("Find 2 in vec1: {:?}", iter.find(|&&x| x == 2));
    println!("Find 2 in vec2: {:?}", into_iter.find(|&x| x == 2));

    let array1 = [1, 2, 3];
    let array2 = [4, 5, 6];
}