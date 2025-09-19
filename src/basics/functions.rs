pub fn functions_example() {
    // 函数（Functions）
    // 函数使用 `fn` 关键字声明。函数的参数需要标注类型，
    // 如果函数返回一个值，返回类型必须在箭头 `->` 之后指定。

    println!("Hello from functions_example!");

    // 调用函数
    another_function(5, 'h');

    // 包含语句和表达式的函数体
    // 函数体由一系列语句和一个可选的结尾表达式构成。
    let x = 5;

    let y = {
        let x = 3;
        x + 1 // 这是一个表达式，没有分号
    };

    println!("The value of y is: {}", y);

    // 带有返回值的函数
    let five = five();
    println!("The value of five is: {}", five);

    let plus_one_result = plus_one(5);
    println!("The value of plus_one(5) is: {}", plus_one_result);
}

fn another_function(x: i32, unit_label: char) {
    println!("The value of x is: {}{}", x, unit_label);
}

fn five() -> i32 {
    5 // 返回 5
}

fn plus_one(x: i32) -> i32 {
    x + 1 // 这是一个表达式，所以没有分号
}