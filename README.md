# Rust 学习指南

欢迎来到 Rust 学习指南！本指南旨在通过一系列代码示例，帮助你掌握 Rust 从基础到高级的核心概念。所有示例代码都可以在 `src` 目录下找到，并通过 `main.rs` 文件统一调用执行。

## 如何使用

1.  克隆或下载本仓库。
2.  在项目根目录下运行 `cargo run`。
3.  观察终端输出，并结合本指南和源代码进行学习。

## Rust 基础 (Basics)

基础部分涵盖了 Rust 的核心概念，是你学习这门语言的基石。

### 1. 变量 (Variables)

-   **文件:** `src/basics/variables.rs`

Rust 中的变量默认是 **不可变的 (immutable)**。这是 Rust 为了保证安全性和并发性而设计的核心原则之一。如果你需要一个可以改变的变量，你需要使用 `mut` 关键字显式地声明它。

```rust
let x = 5; // 不可变变量
let mut y = 10; // 可变变量
y = 20;
```

Rust 还支持 **变量遮蔽 (shadowing)**，允许你使用 `let` 关键字重复声明同名变量。这与将变量标记为 `mut` 是不同的，因为遮蔽会创建一个全新的变量，我们可以改变变量的类型。

```rust
let x = 5;
let x = x + 1; // x 现在是 6

let spaces = "   ";
let spaces = spaces.len(); // spaces 现在是 3，类型从 &str 变为 usize
```

### 2. 函数 (Functions)

-   **文件:** `src/basics/functions.rs`

函数是 Rust 代码的基本构建块。使用 `fn` 关键字定义函数。对于函数参数，你必须声明每个参数的类型。

Rust 是一种基于表达式的语言，这是一个需要理解的重要概念。**表达式 (expression)** 会计算并返回一个值，而 **语句 (statement)** 则执行某些操作但不返回值。函数体可以由一系列语句和一个可选的结尾表达式构成。

```rust
fn five() -> i32 {
    5 // 这是一个表达式，它的值就是 5
}

fn plus_one(x: i32) -> i32 {
    x + 1 // 同样是表达式，注意没有分号
}
```

### 3. 控制流 (Control Flow)

-   **文件:** `src/basics/control_flow.rs`

Rust 提供了 `if`, `else`, `loop`, `while`, `for` 等常见的控制流工具。

`if` 表达式非常强大，因为 `if` 是一个表达式，你可以在 `let` 语句的右侧使用它来根据条件赋一个值。

```rust
let condition = true;
let number = if condition { 5 } else { 6 };
```

`loop` 提供了一个无限循环，你可以使用 `break` 关键字来退出循环，`break` 还可以返回值。

`for` 循环是遍历集合（如数组、vector 或 range）最常用的方式。

```rust
let a = [10, 20, 30, 40, 50];

for element in a.iter() {
    println!("the value is: {}", element);
}
```

### 4. 所有权 (Ownership)

-   **文件:** `src/basics/ownership.rs`

**所有权是 Rust 最核心和最独特的概念。** 它使 Rust 能够在没有垃圾回收器 (Garbage Collector, GC) 的情况下保证内存安全。

所有权有三条基本规则：

1.  Rust 中的每一个值都有一个被称为其 **所有者 (owner)** 的变量。
2.  值在任一时刻有且只有一个所有者。
3.  当所有者离开 **作用域 (scope)** 时，这个值将被丢弃 (dropped)。

对于像 `String` 这样存储在堆上的数据，当变量赋值或函数传参时，会发生 **移动 (move)**，而不是浅拷贝。这意味着所有权被转移，原来的变量将不再有效。

```rust
let s1 = String::from("hello");
let s2 = s1; // s1 的所有权被移动到 s2
// println!("{}", s1); // 这里会编译失败！
```

如果你需要深度复制数据，可以使用 `clone()` 方法。

为了在不转移所有权的情况下使用值，我们可以使用 **引用 (references)**。通过在变量名前加 `&` 来创建一个引用，这个过程称为 **借用 (borrowing)**。

```rust
fn calculate_length(s: &String) -> usize { // s 是一个引用
    s.len()
} // s 离开作用域，但因为它不拥有值，所以什么都不会发生
```

默认情况下引用是不可变的。如果你想修改借用的值，你需要创建一个 **可变引用 (mutable reference)**，使用 `&mut`。

一个重要的限制是：**在特定作用域内，对某一块数据，要么只能有一个可变引用，要么只能有多个不可变引用，但不能同时存在。** 这个规则在编译时就防止了数据竞争。

### 5. 结构体 (Structs)

-   **文件:** `src/basics/structs.rs`

结构体允许你创建由多个相关值组成的自定义数据类型。

你还可以在结构体上定义 **方法 (methods)**。方法与函数类似，但它们在结构体的上下文中定义，并且它们的第一个参数总是 `self`，代表调用该方法的实例。

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}
```

### 6. 枚举 (Enums)

-   **文件:** `src/basics/enums.rs`

枚举允许你定义一个可以是一些列可能值之一的类型。Rust 的枚举非常强大，每个枚举成员都可以关联不同类型和数量的数据。

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
```

Rust 中一个非常重要的标准库枚举是 `Option<T>`，它用于处理一个值可能是某个值，也可能什么都没有 (null) 的情况。它有两个成员：`Some(T)` 和 `None`。这个设计让你必须在编译时处理 `null` 的情况，从而避免了许多在其他语言中常见的 `null` 引用错误。

`match` 控制流运算符是处理枚举的强大工具。它会强制你处理所有可能的情况，确保代码的完备性。`if let` 是 `match` 的一种简写，当你只关心一种情况时非常有用。

## Rust 高级 (Advanced)

高级部分将带你深入了解 Rust 的一些强大特性，这些特性使得 Rust 在系统编程领域大放异彩。

### 1. 生命周期 (Lifetimes)

-   **文件:** `src/advanced/lifetimes.rs`

**生命周期是 Rust 中另一个核心且复杂的概念，它与所有权系统紧密配合，用于确保引用总是有效的。**

生命周期的主要目标是防止 **悬垂引用 (dangling references)**，即引用指向的内存已经被释放。Rust 编译器通过一个叫做 **借用检查器 (borrow checker)** 的工具，在编译时检查所有引用的生命周期是否合法。

**生命周期注解 (lifetime annotations)** 并不会改变引用的存活时间，而是描述了多个引用生命周期之间的关系。这使得编译器能够判断操作是否安全。

```rust
// 'a 是一个泛型生命周期参数
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

这个函数签名告诉编译器：返回的引用 `&'a str` 的生命周期，不会超过传入的两个引用 `x` 和 `y` 中较短的那个生命周期。这样就保证了返回的引用绝对不会指向一个已经无效的字符串。

结构体如果包含引用，也必须定义生命周期注解。

Rust 编译器有一套 **生命周期省略规则 (lifetime elision rules)**，在很多常见场景下，你不需要手动标注生命周期，编译器可以自动推断。这大大简化了日常编码。

### 2. 特征 (Traits)

-   **文件:** `src/advanced/traits.rs`

特征 (Trait) 告诉 Rust 编译器某个类型具有哪些可以与其他类型共享的功能。它类似于其他语言中的 **接口 (interface)**。你可以为任何类型实现一个 trait。

```rust
pub trait Summary {
    fn summarize(&self) -> String;
}

impl Summary for NewsArticle {
    // ...
}

impl Summary for Tweet {
    // ...
}
```

你可以编写一个函数，它接受任何实现了特定 trait 的类型作为参数，这被称为 **trait bound**。

```rust
pub fn notify(item: &impl Summary) { // impl Trait 语法
    println!("Breaking news! {}", item.summarize());
}

// 等价的 trait bound 语法
pub fn notify<T: Summary>(item: &T) { ... }
```

**特征对象 (Trait Objects)** 允许你在运行时处理多种实现了相同 trait 的不同具体类型。这通过 `dyn Trait` 语法实现，通常与 `Box` 等智能指针结合使用，以实现动态分发。

```rust
let items: Vec<Box<dyn Summary>> = vec![
    Box::new(tweet),
    Box::new(article),
];
```

### 3. 泛型 (Generics)

-   **文件:** `src/advanced/generics.rs`

泛型允许我们编写灵活且可重用的代码，它可以处理多种不同的具体类型。函数、结构体、枚举和方法都可以是泛型的。

```rust
fn largest<T: PartialOrd>(list: &[T]) -> &T { ... }

struct Point<T> {
    x: T,
    y: T,
}
```

Rust 在编译时通过 **单态化 (monomorphization)** 来处理泛型。这意味着编译器会根据你实际使用的具体类型，为泛型代码生成专门的版本。因此，使用泛型不会带来任何运行时开销。

### 4. 闭包 (Closures)

-   **文件:** `src/advanced/closures.rs`

闭包 (Closure) 是可以捕获其环境的匿名函数。它们非常灵活，尤其常用于迭代器和线程中。

```rust
let plus_one = |x: u32| -> u32 { x + 1 };
```

闭包可以三种方式捕获其环境中的变量：

1.  **不可变借用 (`&T`)**: `Fn` trait
2.  **可变借用 (`&mut T`)**: `FnMut` trait
3.  **获取所有权 (`T`)**: `FnOnce` trait

你可以使用 `move` 关键字强制闭包获取其捕获变量的所有权，这在多线程编程中非常有用。

### 5. 线程 (Threads)

-   **文件:** `src/advanced/threads.rs`

Rust 的所有权和类型系统为并发编程提供了强大的安全保障。你可以放心地编写多线程代码，因为大部分常见的并发错误（如数据竞争）在编译时就会被发现。

你可以使用 `thread::spawn` 来创建新线程。使用 `move` 闭包可以安全地将数据的所有权从主线程转移到子线程。

除了共享内存，Rust 还提供了 **消息传递 (message passing)** 的并发模型，通过 **通道 (channel)** 来实现线程间的通信，遵循“不要通过共享内存来通信，而要通过通信来共享内存”的哲学。

对于需要多线程共享状态的情况，Rust 提供了 `Mutex` (互斥锁) 和 `Arc` (原子引用计数) 等工具来保证安全的并发访问。

### 6. 宏 (Macros)

-   **文件:** `src/advanced/macros.rs`

宏 (Macro) 是一种元编程 (metaprogramming) 的方式，允许你编写能够生成其他代码的代码。Rust 中有两种宏：

1.  **声明宏 (Declarative Macros)**: 使用 `macro_rules!` 定义，类似于 `match` 表达式，用于代码生成。
2.  **过程宏 (Procedural Macros)**: 更为强大，允许你编写像函数一样操作代码的宏，主要有自定义 `derive`、属性宏和函数式宏三种形式。

像 `println!` 和 `vec!` 都是我们常用的宏。宏在编译时展开，因此不会有运行时开销。