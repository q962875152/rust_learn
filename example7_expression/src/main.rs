// 表达式：是会计算出一个值的代码块。几乎所有的代码在 Rust 中都是表达式。
//  表达式不以分号结尾。如果在表达式后加上分号，就会变成语句。
// 语句：语句是执行某种操作，但不返回值的代码块。

// 最普遍的语句类型有两种：一种是声明绑定变量，另一种是表达式带上英文分号(;)：

fn main() {
    let x = 5u32;

    let y = {
        let x_squared = x * x;
        let x_cube = x_squared * x;

        x_cube + x_squared + x
    };

    let z = {
        2 * x;
    };

    println!("x is {:?}", x);
    println!("y is {:?}", y);
    println!("z is {:?}", z);
}
