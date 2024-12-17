fn main() {
    example1();
    example2();
    example3();
    example4();
}

fn example1() {  // ??????
    #![allow(overflowing_literals)]

    let decimal = 65.4321_f32;

    // let integer: u8 = decimal;  // 不提供隐式转换

    // 可以显示转换
    let integer = decimal as u8;
    let character = integer as char;

    println!("Casting: {} -> {} -> {}", decimal, integer, character);

    // 当把任何类型转换为无符号类型 T 时，会不断加上或减去 (std::T::MAX + 1)
    // 直到值位于新类型 T 的范围内。
    println!("1000 as a u16 is: {}", 1000 as u16);

    println!("1000 as a u16 is: {}", 1000 as u16);

    // 事实上的处理方式是：从最低有效位（LSB，least significant bits）开始保留
    // 8 位，然后剩余位置，直到最高有效位（MSB，most significant bit）都被抛弃。
    println!("1000 as a u8: {}", 1000 as u8);

    println!("-1 as a u8 is: {}", (-1i8) as u8);  // -1 + 256

    println!("1000 mod 256 is : {}", 1000 % 256);

    // 当转换到有符号类型时，（位操作的）结果就和 “先转换到对应的无符号类型，
    // 如果 MSB 是 1，则该值为负” 是一样的。
    // 当然如果数值已经在目标类型的范围内，就直接把它放进去。
    println!("128 as a i16 is: {}", 128 as i16);
    // 128 转成 u8 还是 128，但转到 i8 相当于给 128 取八位的二进制补码，其值是：
    println!("128 as a i8 is : {}", 128 as i8);

    println!("1000 as a u8 is : {}", 1000 as u8);
    println!("232 as a i8 is : {}", 232 as i8);
}


// 对数值字面量，只要把类型作为后缀加上去，就完成了类型说明。比如指定字面量 42 的类型是 i32，只需要写 42i32。
// 无后缀的数值字面量，其类型取决于怎样使用它们。如果没有限制，编译器会对整数使用 i32，对浮点数使用 f64。
fn example2() {
    // 带后缀的字面量，其类型在初始化时已经知道了。
    let x = 1u8;
    let y = 2u32;
    let z = 3f32;

    // 无后缀的字面量，其类型取决于如何使用它们。
    let i = 1;
    let f = 1.0;

    // `size_of_val` 返回一个变量所占的字节数
    println!("sizeof 'x' in bytes: {}", std::mem::size_of_val(&x));
    println!("sizeof 'y' in bytes: {}", std::mem::size_of_val(&y));
    println!("sizeof 'z' in bytes: {}", std::mem::size_of_val(&z));
    println!("sizeof 'i' in bytes: {}", std::mem::size_of_val(&i));
    println!("sizeof 'f' in bytes: {}", std::mem::size_of_val(&f));
}


// Rust 的类型推断引擎是很聪明的，它不只是在初始化时看看右值（r-value）的类型而已，
// 它还会考察变量之后会怎样使用，借此推断类型。
fn example3() {
    // 因为有类型说明，编译器知道 `elem` 的类型是 u8。
    let elem = 5u8;

    // 创建一个空向量,现在编译器还不知道 `vec` 的具体类型，只知道它是某种东西构成的向量（`Vec<_>`）
    let mut vec = Vec::new();

    // 啊哈！现在编译器知道 `vec` 是 u8 的向量了（`Vec<u8>`）
    vec.push(elem);

    println!("{:?}", vec);
}

fn example4() {
    type NanoSecond = u64;
    type Inch= u64;

    #[allow(non_camel_case_types)]
    type u64_t = u64;  // 可以用 type 语句给已有的类型取个新的名字。类型的名字必须遵循驼峰命名法，否则编译器将给出警告。原生类型是例外

    let nanoseconds: NanoSecond = 5 as u64_t;
    let inches: Inch = 2 as u64_t;

    println!("{} nanoseconds + {} inches = {} unit?",
            nanoseconds,
            inches,
            nanoseconds + inches);
}
