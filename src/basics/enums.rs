pub fn enums_example() {
    // 枚举（Enums）
    // 枚举允许你通过列举可能的值来定义一个类型。

    // 定义一个 IP 地址的枚举
    enum IpAddrKind {
        V4,
        V6,
    }

    // 枚举值
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    // 带有值的枚举
    enum IpAddr {
        V4(String),
        V6(String),
    }

    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));

    // 不同的类型和数量的关联数据
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    // 为枚举定义方法
    impl Message {
        fn call(&self) {
            // 在这里定义方法体
            match self {
                Message::Quit => println!("Quit message"),
                Message::Move { x, y } => println!("Move to x: {}, y: {}", x, y),
                Message::Write(text) => println!("Write: {}", text),
                Message::ChangeColor(r, g, b) => println!("Change color to R: {}, G: {}, B: {}", r, g, b),
            }
        }
    }

    let m = Message::Write(String::from("hello"));
    m.call();

    // Option 枚举
    // `Option` 是一个标准库提供的枚举，用于处理值可能存在或不存在的场景。
    // 它有两个变体：`Some(T)`（表示有值）和 `None`（表示没有值）。
    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None;

    // match 控制流运算符
    // `match` 允许你将一个值与一系列模式进行比较，并根据匹配的模式执行代码。
    // 它是穷尽的（exhaustive），意味着必须处理所有可能的情况。
    fn value_in_cents(coin: Coin) -> u8 {
        match coin {
            Coin::Penny => {
                println!("Lucky penny!");
                1
            },
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                println!("State quarter from {:?}!", state);
                25
            },
        }
    }

    #[derive(Debug)] // 这样可以打印出州名
    enum UsState {
        Alabama,
        Alaska,
        // ... 还有很多
    }

    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState),
    }

    let coin = Coin::Quarter(UsState::Alaska);
    println!("Value in cents: {}", value_in_cents(coin));

    // `if let` 简化 `match`
    // `if let` 语法糖用于处理只有一个匹配模式的情况，而忽略其他所有情况。
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }

    // 等价于
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }

    let mut count = 0;
    let coin = Coin::Quarter(UsState::Alabama);
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }
    println!("Count: {}", count);
}