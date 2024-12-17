fn main() {
    example1();
    example2();
    example3();
    example4();
}

fn example1() {
    let an_integer = 1u32;  // 声明一个变量并将值绑定到变量。下同
    let a_boolean = true;
    let unit = ();

    let copied_integer = an_integer;  // copy

    println!("An integer: {:?}", copied_integer);
    println!("A boolean: {:?}", a_boolean);
    println!("Meet the unit value: {:?}", unit);

    let _unused_variable = 3u32;  // 编译器会对未使用的变量绑定产生警告；可以给变量名加上下划线前缀来消除警告。
}

fn example2() {
    let _immutable_binding = 1;
    let mut mutable_binding = 1;

    println!("Before mutation: {}", mutable_binding);

    mutable_binding += 1;

    println!("After mutation: {}", mutable_binding);

    // _immutable_binding += 1 // 错误
}


// 变量绑定有一个作用域（scope），它被限定只在一个代码块（block）中生存（live）。 代码块是一个被 {} 包围的语句集合。
// 另外也允许变量遮蔽（variable shadowing）。
fn example3() {  
    // 此绑定生存于 example3 函数中
    let long_lived_binding = 1;

    // 这是一个代码块，比 example3 函数拥有更小的作用域
    {
        let short_lived_bingding = 2;
        println!("inner long: {}", short_lived_bingding);

        // 此绑定*遮蔽*了外面的绑定
        let long_lived_binding = 5_f32;
        println!("inner long: {}", long_lived_binding);
    }
    // 代码块结束

    // println!("outer short: {}", short_lived_bingding);  // 报错！`short_lived_binding` 在此作用域上不存在
    
    println!("outer long: {}", long_lived_binding);

    let long_lived_binding = 'a';  // 此绑定同样*遮蔽*了前面的绑定

    println!("outer long: {}", long_lived_binding);
}


//  可以先声明（declare）变量绑定，后面才将它们初始化（initialize）。
//  但是这种做法很少用，因为这样可能导致使用未初始化的变量。
fn example4() {
    // 声明一个变量绑定
    let a_binding;

    {
        let x = 2;

        // 初始化一个绑定
        a_binding = x * x;
    }

    println!("a binding: {}", a_binding);

    let another_binding;

    // println!("another binding: {}", another_binding);  // 报错！使用了未初始化的绑定

    another_binding = 1;

    println!("another binding: {}", another_binding);
}

fn example5() {
    let mut _mutable_integer = 7i32;

    {
        // 被不可变的‘_mutable_integer’遮蔽
        let _mutable_integer = _mutable_integer;
        
        // 报错！‘_mutable_integer’在本作用域被冻结
        // _mutable_integer = 50;
    }

    _mutable_integer = 3;
}
