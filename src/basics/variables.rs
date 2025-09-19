// Rust By Example
// 变量绑定
// Rust 通过 `let` 关键字来提供变量绑定功能。

pub fn variables_example() {
    let an_integer = 1u32;
    let a_boolean = true;
    let unit = ();

    // 将 `an_integer` 复制到 `copied_integer`
    let copied_integer = an_integer;

    println!("An integer: {:?}", copied_integer);
    println!("A boolean: {:?}", a_boolean);
    println!("Meet the unit value: {:?}", unit);

    // 编译器会对未使用的变量绑定产生警告；可以给变量名加上下划线前缀来消除警告。
    let _unused_variable = 3u32;

    let noisy_unused_variable = 2u32;
    // 改正 ^ 在变量名前加上下划线：`_noisy_unused_variable`

    // 可变性
    // 变量绑定默认是不可变的（immutable），但加上 `mut` 关键字后变为可变的。
    let _immutable_binding = 1;
    let mut mutable_binding = 1;

    println!("Before mutation: {}", mutable_binding);

    // 正确
    mutable_binding += 1;

    println!("After mutation: {}", mutable_binding);

    // 错误！
    // _immutable_binding += 1;
    // 改正 ^ 将 `_immutable_binding` 声明为可变的：`mut _immutable_binding`

    // 作用域
    // 变量绑定存在于一个作用域（scope）中，该作用域在编译时确定。
    // 绑定在其作用域结束时被销毁。

    // 这个绑定只存在于 main 函数中
    let long_lived_binding = 1;

    // 这是一个代码块，有更小的作用域
    {
        // 这个绑定只存在于本代码块
        let short_lived_binding = 2;

        println!("inner short: {}", short_lived_binding);

        // 这个绑定会遮蔽（shadow）外部的绑定
        let long_lived_binding = 5_f32;

        println!("inner long: {}", long_lived_binding);
    }
    // 代码块结束

    // 错误！`short_lived_binding` 在这里不存在
    // println!("outer short: {}", short_lived_binding);
    // 改正 ^ 将 `short_lived_binding` 的声明移到 main 函数的作用域

    println!("outer long: {}", long_lived_binding);

    // 这个绑定同样会遮蔽之前的绑定
    let long_lived_binding = 'a';

    println!("outer long: {}", long_lived_binding);

    // 声明变量
    // Rust 的变量绑定有一个有趣的特性，它允许在绑定之前使用，只要它被初始化了。
    // 然而，这可能会导致一些问题。

    // 编译器知道 `initialized` 在这里被初始化了
    let initialized = 1;

    // 编译错误！编译器不知道 `uninitialized` 在这里是否被初始化
    // let uninitialized;
    // println!("uninitialized: {}", uninitialized);
    // 改正 ^ 在使用 `uninitialized` 之前对其进行初始化

    // 冻结
    // 当数据被不可变地借用（borrow）时，它也会被冻结（freeze）。
    // 在不可变借用存在期间，无法修改被冻结的数据。

    let mut _mutable_integer = 7i32;

    {
        // 从 `_mutable_integer` 借用
        let _large_integer = &_mutable_integer;

        // 错误！`_mutable_integer` 在此时被冻结
        // _mutable_integer = 50;
        // 改正 ^ 在 `_large_integer` 的借用结束之后再修改 `_mutable_integer`

        println!("Immutably borrowed: {}", _large_integer);
    }
    // `_large_integer` 离开作用域，借用结束

    // 正确！`_mutable_integer` 在这里没有被冻结
    _mutable_integer = 3;
}
