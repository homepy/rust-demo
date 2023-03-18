fn main() {
    {
        // 匹配字面值
        let x = 1;
        match x {
            1 => println!("one"),
            2 => println!("two"),
            3 => println!("three"),
            _ => println!("anything"),
        }
    }

    {
        // 匹配命名变量
        let x = Some(5);
        let y = 10;

        match x {
            Some(50) => println!("Got 50"),
            Some(y) => println!("Matched, y = {y}"), // "Matched, y = 5"
            _ => println!("Default case, x = {:?}", x),
        }

        println!("at the end: x = {:?}, y = {y}", x); // "at the end: x = Some(5), y = 10"
    }

    {
        // 多个模式
        let x = 1;
        match x {
            1 | 2 => println!("one or two"),
            3 => println!("three"),
            _ => println!("anything"),
        }
    }
    {
        // 通过 ..= 匹配值的范围 数值
        let x = 5;

        match x {
            1..=5 => println!("one through five"),
            _ => println!("something else"),
        }
    }
    {
        // 通过 ..= 匹配值的范围 字符
        let x = 'c';

        match x {
            'a'..='j' => println!("early ASCII letter"),
            'k'..='z' => println!("late ASCII letter"),
            _ => println!("something else"),
        }
    }

    /* 解构并分解值，（解构会move所有权） */
    // 解构结构体
    {
        let p = Point { x: 0, y: 7 };
        let Point { x: a, y: b } = p;
        assert_eq!(0, a);
        assert_eq!(7, b);
    }
    {
        let p = Point { x: 0, y: 7 };
        let Point { x, y } = p;
        assert_eq!(0, x);
        assert_eq!(7, y);
    }
    {
        let p = Point { x: 0, y: 7 };
        match p {
            Point { x, y: 0 } => println!("On the x axis at {x}"),
            Point { x: 0, y } => println!("On the y axis at {y}"),
            Point { x, y } => {
                println!("On neither axis: ({x}, {y})");
            }
        }
    }

    // 解构枚举
    {
        let msg = Message::ChangeColor(0, 160, 255);

        match msg {
            Message::Quit => {
                println!("The Quit variant has no data to destructure.");
            }
            Message::Move { x, y } => {
                println!("Move in the x direction {x} and in the y direction {y}");
            }
            Message::Write(text) => {
                println!("Text message: {text}");
            }
            Message::ChangeColor(r, g, b) => {
                println!("Change the color to red {r}, green {g}, and blue {b}",)
            }
        }
    }

    {
        // 解构嵌套的结构体和枚举
        let msg = MessageNew::ChangeColor(Color::Hsv(0, 160, 255));

        match msg {
            MessageNew::ChangeColor(Color::Rgb(r, g, b)) => {
                println!("Change color to red {r}, green {g}, and blue {b}");
            }
            MessageNew::ChangeColor(Color::Hsv(h, s, v)) => {
                println!("Change color to hue {h}, saturation {s}, value {v}")
            }
            _ => (),
        }
    }

    {
        // 解构结构体和元组
        let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });
    }

    /*  忽略模式中的值 */

    {
        // 使用 _ 忽略整个值
        fn foo(_: i32, y: i32) {
            println!("This code only uses the y parameter: {}", y);
        }
    }

    {
        // 使用嵌套的 _ 忽略部分值
        let mut setting_value = Some(5);
        let new_setting_value = Some(10);

        match (setting_value, new_setting_value) {
            (Some(_), Some(_)) => {
                println!("Can't overwrite an existing customized value");
            }
            _ => {
                setting_value = new_setting_value;
            }
        }

        println!("setting is {:?}", setting_value);
    }

    {
        // 忽略元组的多个部分
        let numbers = (2, 4, 8, 16, 32);

        match numbers {
            (first, _, third, _, fifth) => {
                println!("Some numbers: {first}, {third}, {fifth}")
            }
        }
    }

    {
        // 通过在名字前以一个 _ 开头来忽略未使用的变量
        let _x = 5; // 无编译器警告
        let y = 10; // 编译器警告
    }

    // 用 .. 忽略剩余值
    {
        let origin = Point3 { x: 0, y: 0, z: 0 };
        match origin {
            Point3 { x, .. } => println!("x is {}", x),
        }
    }

    {
        let numbers = (2, 4, 8, 16, 32);
        match numbers {
            (first, .., last) => {
                println!("Some numbers: {first}, {last}");
            }
        }
    }

    /*
    匹配守卫提供的额外条件
    匹配守卫（match guard）是一个指定于 match 分支模式之后的额外 if 条件，
    它也必须被满足才能选择此分支。匹配守卫用于表达比单独的模式所能允许的更为复杂的情况。
    */
    {
        let num = Some(4);
        match num {
            Some(x) if x % 2 == 0 => println!("The number {} is even", x),
            Some(x) => println!("The number {} is odd", x),
            None => (),
        }
    }

    {
        let x = Some(5);
        let y = 10;
        match x {
            Some(50) => println!("Got 50"),
            Some(n) if n == y => println!("Matched, n = {n}"),
            _ => println!("Default case, x = {:?}", x),
        }
        println!("at the end: x = {:?}, y = {y}", x);
    }

    {
        let x = 4;
        let y = false;
        match x {
            4 | 5 | 6 if y => println!("yes"),
            _ => println!("no"),
        }
    }

    {
        // @绑定
        enum Message {
            Hello { id: i32 },
        }
        let msg = Message::Hello { id: 5 };
        match msg {
            Message::Hello {
                id: id_variable @ 3..=7,
            } => println!("Found an id in range: {}", id_variable),
            Message::Hello { id: 10..=12 } => {
                println!("Found an id in another range")
            }
            Message::Hello { id } => println!("Found some other id: {}", id),
        }
    }
}
struct Point {
    x: i32,
    y: i32,
}
struct Point3 {
    x: i32,
    y: i32,
    z: i32,
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

enum MessageNew {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}

enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}
