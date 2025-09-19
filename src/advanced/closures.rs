pub fn closures_example() {
    // 闭包（Closures）
    // 闭包是匿名函数，可以捕获其环境中的值。它们在 Rust 中非常灵活和强大。

    // 闭包的定义和调用
    let expensive_closure = |num| {
        println!("calculating slowly...");
        std::thread::sleep(std::time::Duration::from_secs(2));
        num
    };

    let intensity = 10;
    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            expensive_closure(intensity)
        );
    } else {
        println!(
            "Today, take {} breaks!",
            expensive_closure(intensity)
        );
    }

    // 闭包捕获环境
    // 闭包可以捕获其定义作用域中的变量。捕获方式可以是：
    // 1. 不可变借用（&T）
    // 2. 可变借用（&mut T）
    // 3. 获取所有权（T）

    let x = 4;
    let equal_to_x = |z| z == x;
    let y = 4;
    assert!(equal_to_x(y));

    // `move` 关键字
    // 强制闭包获取其捕获变量的所有权。
    let x = vec![1, 2, 3];
    let equal_to_x = move |z| z == x;
    // println!("can't use x here: {:?}", x); // 错误！x 的所有权已转移
    let y = vec![1, 2, 3];
    assert!(equal_to_x(y));

    // 闭包作为函数参数
    // 闭包可以作为函数参数传递，通常通过 `Fn`, `FnMut`, `FnOnce` trait 来指定。
    // `Fn`: 闭包通过不可变引用捕获环境。
    // `FnMut`: 闭包通过可变引用捕获环境。
    // `FnOnce`: 闭包通过获取所有权捕获环境。

    fn apply_function<F>(f: F) -> i32
    where
        F: FnOnce() -> i32,
    {
        f()
    }

    let greeting = String::from("hello");
    let closure_with_move = move || {
        println!("{}", greeting);
        42
    };

    let result = apply_function(closure_with_move);
    println!("Closure result: {}", result);

    // 迭代器与闭包
    // 闭包经常与迭代器一起使用，用于对集合进行转换和过滤。
    let v1 = vec![1, 2, 3];
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    println!("v2: {:?}", v2);
}