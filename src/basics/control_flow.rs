pub fn control_flow_example() {
    // 控制流（Control Flow）
    // Rust 提供了多种控制流结构，包括 `if/else` 表达式、`loop`、`while` 和 `for` 循环。

    // if/else 表达式
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // if 可以在 let 语句中使用
    let condition = true;
    let number = if condition {
        5
    } else {
        6
    };

    println!("The value of number is: {}", number);

    // loop 循环
    // `loop` 关键字表示一个无限循环。可以使用 `break` 关键字退出循环，
    // 也可以使用 `continue` 关键字跳过当前迭代的剩余部分，进入下一次迭代。
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result of loop is: {}", result);

    // while 循环
    // `while` 循环在条件为真时重复执行代码块。
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("LIFTOFF!!!");

    // for 循环
    // `for` 循环用于遍历集合中的元素。这是 Rust 中最常用的循环结构。
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("The value is: {}", element);
    }

    // 倒计时
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}