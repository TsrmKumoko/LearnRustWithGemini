pub fn ownership_example() {
    // 所有权（Ownership）
    // 所有权是 Rust 最独特的特性，它使 Rust 能够在没有垃圾回收器的情况下保证内存安全。
    // 所有权规则：
    // 1. Rust 中的每个值都有一个被称为其所有者（owner）的变量。
    // 2. 一次只能有一个所有者。
    // 3. 当所有者离开作用域时，该值将被丢弃。

    // 变量作用域
    {
        let s = "hello"; // s 从此刻起有效
        // 使用 s
        println!("{}", s);
    } // s 的作用域到此结束，s 不再有效

    // String 类型
    // String 类型是在堆上分配的，因此它能够存储在编译时未知的文本量。
    let mut s = String::from("hello");
    s.push_str(", world!"); // push_str() 在字符串后追加字面值
    println!("{}", s);

    // 移动（Move）
    // 当我们将一个变量赋给另一个变量时，所有权会发生转移。
    let s1 = String::from("hello");
    let s2 = s1;
    // println!("{}", s1); // 错误！s1 的所有权已经转移给 s2
    println!("{}", s2);

    // 克隆（Clone）
    // 如果我们确实需要深度复制 String 的堆数据，而不仅仅是栈数据，可以使用 clone 方法。
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);

    // 栈上数据的复制
    // 对于像整数这样在编译时已知大小的类型，它们完全存储在栈上，因此复制成本很低。
    let x = 5;
    let y = x;
    println!("x = {}, y = {}", x, y);

    // 所有权与函数
    let s = String::from("hello"); // s 进入作用域
    takes_ownership(s); // s 的值移动到函数里
    // 所以到这里 s 不再有效

    let x = 5; // x 进入作用域
    makes_copy(x); // x 应该移动到函数里，
    // 但 i32 是 Copy 的，所以没关系，
    // 在这里仍然可以使用 x

    // 返回值与作用域
    let s1 = gives_ownership(); // gives_ownership 将返回值
    // 移给 s1
    let s2 = String::from("hello"); // s2 进入作用域
    let s3 = takes_and_gives_back(s2); // s2 被移动到
    // takes_and_gives_back 中，它也将返回值
    // 移给 s3

    println!("s1: {}, s3: {}", s1, s3);

    // 引用与借用（References and Borrowing）
    // 引用允许我们在不转移所有权的情况下使用值。
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);

    // 可变引用
    // 可变引用允许我们修改我们借用的值。
    let mut s = String::from("hello");
    change(&mut s);
    println!("{}", s);

    // 可变引用的限制
    // 在特定作用域内，对某一块数据，只能有一个可变引用。
    // let r1 = &mut s;
    // let r2 = &mut s; // 错误！
    // println!("{}, {}", r1, r2);

    // 悬垂引用（Dangling References）
    // 在 Rust 中，编译器保证引用永远不会是悬垂引用。
    // let reference_to_nothing = dangle();
}

fn takes_ownership(some_string: String) { // some_string 进入作用域
    println!("{}", some_string);
} // 这里，some_string 移出作用域并调用 `drop`。占用的内存被释放

fn makes_copy(some_integer: i32) { // some_integer 进入作用域
    println!("{}", some_integer);
} // 这里，some_integer 移出作用域。不会有特殊操作

fn gives_ownership() -> String { // gives_ownership 会将
    // 返回值移动给
    // 调用它的函数
    let some_string = String::from("yours"); // some_string 进入作用域
    some_string // 返回 some_string 并移出给调用的函数
}

fn takes_and_gives_back(a_string: String) -> String { // a_string 进入作用域
    a_string // 返回 a_string 并移出给调用的函数
}

fn calculate_length(s: &String) -> usize { // s 是对 String 的引用
    s.len()
} // 这里，s 移出作用域。但因为它并不拥有其引用的值，
// 所以什么也不会发生

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// fn dangle() -> &String { // dangle 返回一个字符串的引用
//     let s = String::from("hello"); // s 是一个新字符串
//     &s // 我们返回 s 的引用
// } // 这里 s 离开作用域，它将被丢弃。它的内存会返还
// // 危险！