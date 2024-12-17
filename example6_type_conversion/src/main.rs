//  Rust 使用 trait 解决类型之间的转换问题。

use std::result;

fn main() {
    example1();
    example2();
    example3();
}

fn example1() {  // From trait 允许一种类型定义 “怎么根据另一种类型生成自己”
    let my_str = "hello";
    let my_string = String::from(my_str);

    use std::convert::From;

    #[derive(Debug)]
    struct Number {
        value: i32,
    }

    impl From<i32> for Number {
        fn from(item: i32) -> Self {
            Number {value: item}
        }
    }

    let num = Number::from(30);
    println!("My number is {:?}", num);

    // Into trait 就是把 From trait 倒过来而已。也就是说，如果你为你的类型实现了 From，
    // 那么同时你也就免费获得了 Into。
    // 使用 Into trait 通常要求指明要转换到的类型，因为编译器大多数时候不能推断它。
    let int = 5;
    let num: Number = int.into();
    println!("My number is {:?}", num);
}


// 类似于 From 和 Into，TryFrom 和 TryInto 是类型转换的通用 trait。不同于 From/Into 的是，
// TryFrom 和 TryInto trait 用于易出错的转换，也正因如此，其返回值是 Result 型。
fn example2() {
    use std::convert::TryFrom;
    use std::convert::TryInto;

    #[derive(Debug, PartialEq)]
    struct EvenNumber(i32);

    impl TryFrom<i32> for EvenNumber {
        type Error = ();

        fn try_from(value: i32) -> Result<Self, Self::Error> {
            if value % 2 == 0 {
                Ok(EvenNumber(value))
            } else {
                Err(())
            }
        }
    }

    // TryFrom
    assert_eq!(EvenNumber::try_from(8), Ok(EvenNumber(8)));
    assert_eq!(EvenNumber::try_from(5), Err(()));

    // TryInto
    let result: Result<EvenNumber, ()> = 8i32.try_into();
    assert_eq!(result, Ok(EvenNumber(8)));
    let result: Result<EvenNumber, ()> = 5i32.try_into();
    assert_eq!(result, Err(()));
}


//  要把任何类型转换成 String，只需要实现那个类型的 ToString trait。然而不要直接这么做，
//  您应该实现fmt::Display trait，它会自动提供 ToString，并且还可以用来打印类型，
fn example3() {
    use std::fmt;

    struct Circle {
        radius: i32
    }

    impl fmt::Display for Circle {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Circle of radius {}", self.radius)
        }
    }

    let circle = Circle {radius: 6};
    println!("{}", circle.to_string()); 

    // 只要对目标类型实现了 FromStr trait，就可以用 parse 把字符串转换成目标类型。 
    // 标准库中已经给无数种类型实现了 FromStr。如果要转换到用户定义类型，只要手动实现 FromStr 就行。

    let parsed: i32 = "5".parse().unwrap();
    let turbo_parsed = "10".parse::<i32>().unwrap();

    let sum = parsed + turbo_parsed;
    println!("Sum: {:?}", sum);
}
