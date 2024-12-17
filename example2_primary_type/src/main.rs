use core::fmt;
use std::{default, path::Display};

fn main() {
    // example2();
    // example3();
    example4();
}

fn example1() {
    let logical: bool = true;

    let a_float: f64 = 1.0; // 常规说明
    let an_integer = 5i32; // 后缀说明

    let default_float = 3.0;
    let default_integer = 7;

    // 也可以根据上下文自动推断
    let mut inferred_type = 12; // 根据下一行的赋值推断为i64类型
    inferred_type = 4294967296i64;

    let mut mutable = 12;
    mutable = 21;

    // mutable = true;  // 报错，变量的类型不能改变

    let mutable = true;  // 但可以用遮蔽来覆盖前面的变量
}

fn example2() {
    println!("1 + 2 = {}", 1u32 + 2);
    println!("1 - 2 = {}", 1i32 - 2);
    // println!("1 - 2 = {}", 1u32 - 2);  // 编译错误，溢出

    println!("true AND false is {}", true && false);
    println!("true OR false is {}", true || true);
    println!("NOT true is {}", !true);

    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);

    println!("One million is written as {}", 1_000_000u32);
}

fn reverse(pair: (i32, bool)) -> (bool, i32) {
    // 可以使用'let'把一个元组的成员绑定到一些变量
    let (integer, boolean) = pair;

    (boolean, integer)
}

#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({} {})\n({} {})", self.0, self.1, self.2, self.3)
    }
}

fn transpose(matrix: Matrix) -> Matrix {
    let mut new_matrix: Matrix = Matrix(0.0, 0.0, 0.0, 0.0);
    new_matrix.0 = matrix.0;
    new_matrix.1 = matrix.2;
    new_matrix.2 = matrix.1;
    new_matrix.3 = matrix.3;
    new_matrix
}

fn example3() {
    let long_tuple = (1u8, 2u16, 3u32, 4u64,
                                                    -1i8, -2i16, -3i32, -4i64,
                                                    0.1f32, 0.2f64,
                                                    'a', true);

    println!("long tuple first value: {}", long_tuple.0);
    println!("long tuple second value:{}", long_tuple.1);

    let tuple_of_tuples = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);

    println!("tuple of tuples: {:?}", tuple_of_tuples);

    // 但很长的元组无法打印
    let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
    // println!("too long tuple: {:?}", too_long_tuple);  // 在 Rust 中，如果一个元组的元素都实现了Debug trait，
                                                          // 那么这个元组也会自动实现Debug trait。然而，如果元组
                                                          // 包含超过12个元素，Rust默认不会为它实现Debug trait，
                                                          // 因为这可能会导致调试信息过于冗长而难以阅读。
    
    let pair = (1, true);
    println!("pair is {:?}", pair);

    println!("the reversed pair is {:?}", reverse(pair));

    // 创建单元素元组需要一个额外的逗号，这是为了和被括号包含的字面量作区分
    println!("one element tuple: {:?}", (5u32,));
    println!("just an integer: {:?}", (5u32));

    let tuple = (1, "hello", 4.5, true);

    let (a, b, c, d) = tuple;
    println!("{:?}, {:?}, {:?}, {:?}", a, b, c, d);

    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("{}", matrix);
    println!("{}", transpose(matrix))
}

use std::mem;

fn analyze_slice(slice: &[i32]) {
    println!("first element of the slice: {}", slice[0]);
    println!("the slice has {} elements", slice.len());
}

fn example4() {
    let xs:[i32; 5] = [1, 2, 3, 4, 5];

    let ys:[i32; 500] = [0; 500];

    println!("first element of the array: {}", xs[0]);
    println!("second element of the array: {}", xs[1]);

    println!("array size: {}", xs.len());

    println!("array occupies {} bytes", mem::size_of_val(&xs));  // 获取数组在栈中分配的大小

    println!("borrow the whole array as a slice");
    analyze_slice(&xs);

    println!("borrow a section of the array as a slice");
    analyze_slice(&ys[1 .. 4]);

    // println!("{}", xs[5]);  越界的下标会引发致命错误（panic）
}