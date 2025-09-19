pub fn macros_example() {
    // 宏（Macros）
    // 宏是元编程（metaprogramming）的一种形式，它允许你编写可以编写其他代码的代码。
    // 在 Rust 中，宏在编译时展开，这意味着它们在代码编译之前被替换为生成的代码。

    // 声明宏（Declarative Macros）
    // 使用 `macro_rules!` 宏来定义声明宏。它们类似于 `match` 表达式，
    // 匹配模式并替换为相应的代码。

    // 示例：一个简单的 `vec!` 宏实现
    macro_rules! my_vec {
        // 匹配零个或多个表达式，每个表达式后跟一个逗号
        ($($x:expr),*) => {
            {
                let mut temp_vec = Vec::new();
                $( // 对每个匹配到的表达式重复以下代码
                    temp_vec.push($x);
                )*
                temp_vec
            }
        };
    }

    let v = my_vec![1, 2, 3];
    println!("my_vec: {:?}", v);

    let v2: Vec<i32> = my_vec![];
    println!("my_vec2: {:?}", v2);

    // 过程宏（Procedural Macros）
    // 过程宏允许你编写看起来像函数但操作代码而不是值的宏。
    // 它们有三种类型：
    // 1. 自定义 `derive` 宏：为结构体和枚举实现 `derive` 属性。
    // 2. 属性宏：允许你创建自定义属性。
    // 3. 函数式宏：看起来像函数调用，但操作其参数的 token。

    // 过程宏需要单独的 crate 来定义。
    // 例如，`serde` crate 提供了 `#[derive(Serialize, Deserialize)]` 宏。

    // 示例：自定义 `derive` 宏（概念性，需要单独的 crate）
    // #[some_macro]
    // struct SomeStruct { /* ... */ }

    // 宏的优点：
    // - 减少重复代码（DRY 原则）。
    // - 领域特定语言（DSL）的实现。
    // - 条件编译。

    // 宏的缺点：
    // - 学习曲线陡峭。
    // - 调试困难。
    // - 可能导致代码难以阅读和理解。
}