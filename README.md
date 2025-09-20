# Rust 学习指南

欢迎来到 Rust 学习指南！本指南旨在通过一系列代码示例，帮助你掌握 Rust 从基础到高级的核心概念。所有示例代码都可以在 `src` 目录下找到，并通过 `main.rs` 文件统一调用执行。

## 如何使用

1.  克隆或下载本仓库。
2.  在项目根目录下运行 `cargo run`。
3.  观察终端输出，并结合本指南和源代码进行学习。

## Rust 基础 (Rust Basics)

### 1. 所有权 (Ownership)

所有权是 Rust 最核心和最独特的概念，它让 Rust 能够在没有垃圾回收 (Garbage Collector) 的情况下保证内存安全。理解所有权对于编写高效、安全的 Rust 代码至关重要。

#### 所有权规则

所有权系统遵循以下三个核心规则：

1.  **每个值都有一个所有者 (owner)**。
2.  **一次只能有一个所有者**。
3.  **当所有者离开作用域 (scope) 时，值将被丢弃 (dropped)**。

#### 作用域 (Scope)

作用域是一个项（比如一个变量）在程序中有效的范围。

```rust
// in src/basics/ownership.rs
{
    let s = "hello"; // s 从此刻起有效
    // 使用 s
    println!("{}", s);
} // s 的作用域到此结束，s 不再有效
```

#### `String` 类型

为了更好地演示所有权，我们使用 `String` 类型。与基本类型的字符串字面量不同，`String` 类型的数据存储在堆上，它的大小在编译时是未知的，并且可以改变。

```rust
// in src/basics/ownership.rs
let mut s = String::from("hello");
s.push_str(", world!"); // 追加字符串
println!("{}", s); // 输出: hello, world!
```

#### 移动 (Move)

当我们将一个像 `String` 这样存储在堆上的数据从一个变量赋给另一个变量时，Rust 会进行“移动”而不是“浅拷贝”。这意味着所有权被转移了。

```rust
// in src/basics/ownership.rs
let s1 = String::from("hello");
let s2 = s1;
// println!("{}", s1); // 编译错误！s1 的所有权已经转移给 s2
println!("{}", s2);
```
这种机制可以防止“二次释放”（double free）的内存错误。

#### 克隆 (Clone)

如果你确实需要深度复制数据（包括堆上的数据），可以使用 `clone` 方法。

```rust
// in src/basics/ownership.rs
let s1 = String::from("hello");
let s2 = s1.clone();
println!("s1 = {}, s2 = {}", s1, s2); // s1 和 s2 都是有效的
```

#### 栈上数据的复制 (Copy)

对于完全存储在栈上的数据，例如整数、浮点数、布尔值和字符等，它们实现了 `Copy` trait。当这些类型的变量被重新赋值时，会创建一个新的副本，而不是移动所有权。

```rust
// in src/basics/ownership.rs
let x = 5;
let y = x;
println!("x = {}, y = {}", x, y); // x 和 y 都是有效的
```

#### 所有权与函数

将值传递给函数时，同样会发生移动或复制。

```rust
// in src/basics/ownership.rs
let s = String::from("hello");
takes_ownership(s); // s 的所有权被移动到函数中
// println!("{}", s); // 编译错误！s 在这里不再有效

let x = 5;
makes_copy(x); // x 被复制到函数中
println!("{}", x); // x 在这里仍然有效
```

函数也可以返回带有所有权的值。

#### 引用与借用 (References and Borrowing)

如果我们希望在不转移所有权的情况下让函数使用某个值，我们可以使用“引用”（references）。创建一个引用被称为“借用”（borrowing）。

```rust
// in src/basics/ownership.rs
let s1 = String::from("hello");
let len = calculate_length(&s1); // &s1 创建了一个指向 s1 的引用
println!("The length of '{}' is {}.", s1, len); // s1 在这里仍然有效
```

默认情况下，引用是不可变的。如果你想修改借用的值，需要创建“可变引用”（mutable reference）。

```rust
// in src/basics/ownership.rs
let mut s = String::from("hello");
change(&mut s);
println!("{}", s);
```

可变引用有一个重要的限制：**在特定的作用域内，对某一块数据，你只能拥有一个可变引用**。这个限制可以在编译时防止数据竞争。

### 2. 变量与可变性 (Variables & Mutability)

在 Rust 中，变量默认是 **不可变的 (immutable)**。这是 Rust 为了鼓励你编写更安全、更易于并发的代码而做出的设计选择。当你需要一个可以被修改的变量时，你必须显式地使用 `mut` 关键字使其变为 **可变的 (mutable)**。

#### 不可变变量

一旦一个值被绑定到一个不可变的变量上，你就不能再改变这个值。

```rust
// in src/basics/variables.rs
let x = 5;
println!("The value of x is: {}", x);
// x = 6; // 编译错误！不能对不可变变量进行二次赋值
```

#### 可变变量

通过在变量名前添加 `mut`，你可以允许一个变量的值被改变。

```rust
// in src/basics/variables.rs
let mut y = 5;
println!("The value of y is: {}", y);
y = 6;
println!("The value of y is now: {}", y);
```

#### 常量 (Constants)

常量与不可变变量类似，但有一些区别。常量在程序运行期间其值永远不会改变。

-   必须使用 `const` 关键字声明，并且必须显式地注明值的类型。
-   只能被设置为常量表达式，而不能是函数调用的结果或任何其他只能在运行时计算出的值。
-   可以在任何作用域中声明，包括全局作用域。

```rust
const MAX_POINTS: u32 = 100_000;
```

#### 遮蔽 (Shadowing)

Rust 允许你声明一个与之前变量同名的新变量，这个过程称为“遮蔽”(shadowing)。新的变量会“遮蔽”掉旧的变量。

```rust
// in src/basics/variables.rs
let z = 5;

// 在新的作用域内遮蔽 z
{
    let z = z * 2;
    println!("The value of z in the inner scope is: {}", z); // 输出 10
}

println!("The value of z is: {}", z); // 输出 5
```

遮蔽与将变量标记为 `mut` 是不同的。遮蔽实际上是创建了一个全新的变量，因此我们可以改变值的类型，并且可以重复使用 `let` 关键字。

```rust
// in src/basics/variables.rs
let spaces = "   ";
let spaces = spaces.len(); // 从 &str 类型变为 usize 类型
```

## Rust 基础 (Basics)

基础部分涵盖了 Rust 的核心概念，是你学习这门语言的基石。

### 3. 函数 (Functions)

在 Rust 中，函数是执行特定任务的代码块。你已经见过一个最重要的函数：`main` 函数，它是很多程序的入口点。

#### 函数定义与调用

函数定义以 `fn` 关键字开始，后跟函数名、一对圆括号和一对花括号。

```rust
// in src/basics/functions.rs
pub fn functions_example() {
    println!("Hello from functions_example!");
    another_function(5, 6.7);
}

fn another_function(x: i32, y: f64) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}
```

- **参数 (Parameters)**: 在函数签名中，你必须声明每个参数的类型。
- **调用 (Calling)**: 通过函数名和传入相应的参数来调用函数。

#### 语句和表达式 (Statements and Expressions)

Rust 的函数体由一系列语句组成，可以以一个表达式结尾。

- **语句 (Statements)**: 是执行某些操作但不返回值的指令。例如，`let x = 5;` 是一个语句。
- **表达式 (Expressions)**: 会计算并产生一个值。例如，`5 + 6` 是一个表达式。

表达式可以作为语句的一部分，但语句不能作为表达式的一部分。在 Rust 中，`if`、`match`、函数调用等都是表达式。

```rust
// in src/basics/functions.rs
fn five() -> i32 {
    5 // 这是一个表达式，它的值就是函数的返回值
}
```

#### 函数返回值 (Return Values)

函数可以向调用它的代码返回值。我们不命名返回值，但要在箭头 (`->`) 后面声明它的类型。在 Rust 中，函数的返回值等同于函数体最后一个表达式的值。

```rust
// in src/basics/functions.rs
fn plus_one(x: i32) -> i32 {
    x + 1 // 没有分号，这是一个表达式
}
```

如果你在表达式末尾加上一个分号，它就会变成一个语句，不再返回值。这可能会导致编译错误。

### 4. 控制流 (Control Flow)

Rust 提供了多种控制流结构，允许你根据条件执行不同的代码路径，或者重复执行某些代码。

#### `if` 表达式

`if` 表达式允许你根据一个条件来决定执行哪段代码。条件必须是 `bool` 类型。

```rust
// in src/basics/control_flow.rs
let number = 3;

if number < 5 {
    println!("condition was true");
} else {
    println!("condition was false");
}
```

`if` 是一个表达式，这意味着它可以在 `let` 语句的右侧使用，来根据条件给变量赋值。`if` 和 `else` 块中的值的类型必须相同。

```rust
// in src/basics/control_flow.rs
let condition = true;
let number = if condition { 5 } else { 6 };
println!("The value of number is: {}", number);
```

#### 循环 (Loops)

Rust 提供了三种循环结构：`loop`、`while` 和 `for`。

##### `loop`

`loop` 关键字会创建一个无限循环。你需要使用 `break` 关键字来显式地退出循环。`loop` 循环可以返回值。

```rust
// in src/basics/control_flow.rs
let mut counter = 0;

let result = loop {
    counter += 1;

    if counter == 10 {
        break counter * 2; // 从循环返回值
    }
};

println!("The result of loop is: {}", result);
```

##### `while`

`while` 循环在每次迭代开始时检查一个条件。如果条件为 `true`，则执行循环体。如果为 `false`，则退出循环。

```rust
// in src/basics/control_flow.rs
let mut number = 3;

while number != 0 {
    println!("{}!", number);
    number -= 1;
}

println!("LIFTOFF!!!");
```

##### `for`

`for` 循环用于遍历一个集合中的每个元素，例如数组或范围 (Range)。这是 Rust 中最常用、最安全的循环结构，因为它避免了手动管理索引可能导致的错误。

```rust
// in src/basics/control_flow.rs
let a = [10, 20, 30, 40, 50];

for element in a.iter() {
    println!("The value is: {}", element);
}

// 使用范围和 .rev() 方法进行倒计时
for number in (1..4).rev() {
    println!("{}!", number);
}
println!("LIFTOFF!!!");
```

### 5. 结构体 (Structs)

结构体 (`struct`) 是一种自定义数据类型，它允许你将多个相关的值组合在一起，形成一个更有意义的单元。这类似于面向对象语言中的对象的属性。

#### 定义和实例化结构体

使用 `struct` 关键字来定义一个结构体，并在花括号中为每个字段指定名称和类型。

```rust
// in src/basics/structs.rs
#[derive(Debug)] // 这个注解让我们可以使用 {:?} 或 {:#?} 来打印结构体
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// 创建一个 User 结构体的实例
let mut user1 = User {
    email: String::from("someone@example.com"),
    username: String::from("someusername123"),
    active: true,
    sign_in_count: 1,
};

// 可以通过点号访问实例的字段
user1.email = String::from("anotheremail@example.com");
```

- **字段初始化简写 (Field Init Shorthand)**: 如果函数参数或变量的名称与结构体字段的名称完全相同，你可以使用这种简写语法，避免重复。

```rust
// in src/basics/structs.rs
fn build_user(email: String, username: String) -> User {
    User {
        email,    // 等同于 email: email,
        username, // 等同于 username: username,
        active: true,
        sign_in_count: 1,
    }
}
```

- **结构体更新语法 (Struct Update Syntax)**: 当你想基于一个旧的实例创建一个新的实例，并且只改变其中几个字段时，这个语法非常有用。它使用 `..` 来指定未显式设置的字段应从另一个实例中获取。

```rust
// in src/basics/structs.rs
let user3 = User {
    email: String::from("user3@example.com"),
    ..user1 // user1 的其余字段将被复制到 user3
};
```

#### 元组结构体 (Tuple Structs)

元组结构体是一种看起来像元组的结构体。它有结构体名称，但字段没有名称，只有类型。当你想要给整个元组一个名字，并使其成为一个独立的类型，但又不需要为每个字段命名时，元组结构体很有用。

```rust
// in src/basics/structs.rs
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

let black = Color(0, 0, 0);
let origin = Point(0, 0, 0);
```
`Color` 和 `Point` 是不同的类型，即使它们内部都由三个 `i32` 组成。

#### 类单元结构体 (Unit-Like Structs)

你还可以定义没有任何字段的结构体！这被称为“类单元结构体”，因为它类似于 `()`，即单元类型。当你想在一个类型上实现某个 trait，但又不需要在类型中存储任何数据时，这种结构体非常有用。

```rust
// in src/basics/structs.rs
struct AlwaysEqual;

let subject = AlwaysEqual;
```

#### 方法 (Methods)

方法与函数类似，但它们在结构体（或枚举、trait）的上下文中定义，并且它们的第一个参数总是 `self`，代表调用该方法的实例。

使用 `impl` 块（implementation block）来为结构体定义方法。

```rust
// in src/basics/structs.rs
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // `&self` 是 `self: &Self` 的简写。
    // 这是一个借用，方法只是读取 Rectangle 的数据，而不是获取其所有权。
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // 方法可以有多个参数
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

let rect1 = Rectangle { width: 30, height: 50 };
println!("The area is {}", rect1.area());
```

#### 关联函数 (Associated Functions)

`impl` 块中也可以定义不以 `self` 作为第一个参数的函数。这些被称为“关联函数”，因为它们与结构体关联。它们通常用作构造函数。

```rust
// in src/basics/structs.rs
impl Rectangle {
    // 这是一个关联函数，因为它没有 `self` 参数
    // 它通常用于创建结构体的新实例
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}

// 使用 `::` 语法来调用关联函数
let sq = Rectangle::square(3);
```
`String::from` 就是一个我们已经使用过的关联函数。

### 7. 生命周期 (Lifetimes)

生命周期是 Rust 一个用来确保引用有效性的核心概念，也是 Rust 最独特的特性之一。它允许编译器在编译时检查引用的有效性，从而防止 **悬垂引用 (Dangling References)**，即引用指向了已经被释放的内存。

#### 为什么需要生命周期？

想象一个函数，它接收两个字符串切片的引用，并返回其中较长的那一个。

```rust
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

这段代码无法编译！编译器不知道返回的引用 `&str` 是指向 `x` 还是 `y`，因此它也无法判断这个返回的引用的生命周期应该与 `x` 的生命周期相同，还是与 `y` 的生命周期相同。

这就是生命周期注解发挥作用的地方。

#### 生命周期注解语法

生命周期注解不会改变任何引用的生命周期，而是描述了多个引用生命周期之间的关系，让编译器可以进行分析。

- 生命周期参数名必须以撇号 `'` 开头，通常是简短的全小写字母，如 `'a`、`'b`。
- `&'a i32`：一个带有生命周期 `'a` 的 `i32` 的引用。
- `&'a mut i32`：一个带有生命周期 `'a` 的可变 `i32` 的引用。

现在，让我们用生命周期注解来修复 `longest` 函数：

```rust
// in src/advanced/lifetimes.rs
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

- `<'a>`：这是一个泛型生命周期参数声明，告诉 Rust 我们将使用一个名为 `'a` 的生命周期。
- `x: &'a str` 和 `y: &'a str`：表示 `x` 和 `y` 这两个引用都必须至少活得和生命周期 `'a` 一样长。
- `-> &'a str`：表示函数返回的字符串切片也将至少活得和生命周期 `'a` 一样长。

这个签名的意思是：**对于某个生命周期 `'a`，函数 `longest` 接收两个生命周期至少为 `'a` 的字符串切片，并返回一个生命周期也至少为 `'a` 的字符串切片。** 这实际上是告诉编译器，返回的引用的生命周期与传入参数中较短的那个生命周期相关联。

#### 结构体定义中的生命周期注解

如果一个结构体持有引用，那么它的定义也需要使用生命周期注解。

```rust
// in src/advanced/lifetimes.rs
#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}

let novel = String::from("Call me Ishmael. Some years ago...");
let first_sentence = novel.split('.').next().expect("Could not find a '.'");
let i = ImportantExcerpt {
    part: first_sentence,
};
```

`ImportantExcerpt` 结构体有一个字段 `part`，它持有一个字符串切片的引用。因此，我们需要在结构体名称后面声明一个生命周期参数 `'a`，并在 `part` 字段的类型中使用它。这表示 `ImportantExcerpt` 的实例不能比它所引用的字符串切片活得更长。

#### 生命周期省略规则 (Lifetime Elision Rules)

为了方便，Rust 编译器内置了一套“生命周期省略规则”。如果你的代码符合这些规则，你就不需要显式地写出生命周期注解。这些规则不是让编译器推断出更多的信息，而是为一些非常常见的模式提供了简写。

1.  **规则一**: 每一个是引用的参数都有它自己的生命周期参数。
2.  **规则二**: 如果只有一个输入生命周期参数，那么它被赋予所有输出生命周期参数。
3.  **规则三**: 如果方法有多个输入生命周期参数，但其中之一是 `&self` 或 `&mut self`，那么 `self` 的生命周期被赋予所有输出生命周期参数。

如果编译器在应用完这三条规则后，仍然无法确定所有引用的生命周期，它就会报错，并要求你手动添加注解。

#### 静态生命周期 (`'static`)

`'static` 是一个特殊的生命周期，它表示引用可以在程序的整个运行期间都有效。所有的字符串字面值都拥有 `'static` 生命周期，因为它们直接存储在程序的二进制文件中。

```rust
// in src/advanced/lifetimes.rs
let s: &'static str = "I have a static lifetime.";
```

#### 结合泛型、Trait Bounds 和生命周期

我们可以在一个函数签名中同时使用泛型类型参数、trait bounds 和生命周期。

```rust
// in src/advanced/lifetimes.rs
use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

这个函数结合了：
- 泛型生命周期 `'a`
- 泛型类型参数 `T`
- `T` 的 trait bound `Display`

生命周期是 Rust 实现内存安全的关键，虽然初看起来可能有些复杂，但它使得 Rust 程序在没有垃圾回收的情况下也能保证内存安全，并且没有运行时开销。

### 8. 特征 (Traits)

特征 (trait) 是 Rust 中一个强大的概念，它告诉编译器某个类型具有哪些可以与其他类型共享的功能。特征类似于其他语言中的接口 (interface)，但有一些不同之处。

#### 定义特征

特征定义是一组方法签名的集合，用于定义实现某些目的所必需的一组行为。

```rust
// in src/advanced/traits.rs
pub trait Summary {
    fn summarize(&self) -> String;

    // 也可以提供默认实现
    fn summarize_default(&self) -> String {
        String::from("(Read more...)")
    }
}
```

- 使用 `trait` 关键字定义一个特征。
- 特征中的方法可以有默认实现，实现该特征的类型可以选择重写或使用默认实现。

#### 为类型实现特征

一旦定义了特征，你就可以为你的数据类型（如结构体或枚举）实现它。这通过 `impl Trait for Type` 语法完成。

```rust
// in src/advanced/traits.rs
pub struct NewsArticle { /* ... */ }

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet { /* ... */ }

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}
```

现在，`NewsArticle` 和 `Tweet` 类型都拥有了 `summarize` 方法。

#### 特征作为参数

你可以使用 `impl Trait` 语法来接受任何实现了特定特征的类型作为参数。这允许我们编写可以操作多种不同类型参数的通用函数。

```rust
// in src/advanced/traits.rs
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
```

`notify` 函数可以接受任何实现了 `Summary` 特征的类型的引用，例如 `&NewsArticle` 或 `&Tweet`。

##### Trait Bound 语法

`impl Trait` 语法是 trait bound 语法的语法糖。上面的 `notify` 函数等同于：

```rust
// in src/advanced/traits.rs
pub fn notify_trait_bound<T: Summary>(item: &T) {
    println!("Breaking news (trait bound)! {}", item.summarize());
}
```

当有多个泛型参数和 trait bound 时，使用 `where` 子句可以使函数签名更清晰：

```rust
fn some_function<T, U>(t: &T, u: &U) -> i32
    where T: Display + Clone,
          U: Clone + Debug
{ /* ... */ }
```

#### 返回实现了特征的类型

你也可以在函数返回值中使用 `impl Trait` 语法，来返回一个实现了某个特征的类型，而无需指定具体的返回类型。

```rust
// in src/advanced/traits.rs
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}
```

这在返回闭包或迭代器时特别有用，因为它们的具体类型可能非常复杂或无法写出。

#### 使用特征对象来存储不同类型的值

如果你想创建一个包含不同类型值的集合（例如一个 `Vec`），而这些类型都实现了同一个特征，你可以使用 **特征对象 (Trait Objects)**。

特征对象指向一个实现了特定特征的类型的实例，以及一个用于在运行时查找该类型方法的表。你可以通过 `&dyn Trait` 或 `Box<dyn Trait>` 来创建特征对象。

```rust
// in src/advanced/traits.rs
pub fn notify_trait_object(item: &dyn Summary) {
    println!("Breaking news (trait object)! {}", item.summarize());
}

// 你可以创建一个包含 Box<dyn Summary> 的 vector
let components: Vec<Box<dyn Summary>> = vec![
    Box::new(tweet),
    Box::new(article),
];
```

特征是 Rust 实现 **多态性 (Polymorphism)** 的主要方式，它允许你编写灵活且可重用的代码，同时保持编译时的类型安全。

### 9. 泛型 (Generics)

泛型 (Generics) 是我们用来创建可重用代码的工具，它允许我们编写灵活的函数、结构体、枚举和方法，这些代码可以适用于多种不同的具体类型，同时仍然保持编译时的类型安全。

#### 在函数定义中使用泛型

我们可以定义一个函数，它接受任何类型的切片，并返回最大的元素。为了做到这一点，我们需要使用泛型。

```rust
// in src/advanced/generics.rs
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}
```

- `<T: PartialOrd + Copy>`: 这是泛型类型参数的声明。
    - `T`: 是我们定义的类型参数的名称。
    - `: PartialOrd + Copy`: 这是 `T` 的 **trait bound**。它约束了 `T` 必须是实现了 `PartialOrd`（用于比较大小）和 `Copy`（允许值被复制，例如从 `list` 中移动到 `largest` 变量）这两个 trait 的类型。

这个 `largest` 函数现在可以用于任何满足这些 trait bound 的类型，例如 `Vec<i32>` 或 `Vec<char>`。

#### 在结构体定义中使用泛型

我们也可以在结构体定义中使用泛型，以创建一个可以持有任何类型值的结构体。

```rust
// in src/advanced/generics.rs
#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

let integer = Point { x: 5, y: 10 };
let float = Point { x: 1.0, y: 4.0 };
```

这个 `Point<T>` 结构体可以用来存储任何类型的 `x` 和 `y`，但 `x` 和 `y` 必须是相同的类型 `T`。如果你需要存储不同类型的 `x` 和 `y`，你可以使用多个泛型参数：

```rust
// in src/advanced/generics.rs
#[derive(Debug)]
struct PointMix<T, U> {
    x: T,
    y: U,
}

let p = PointMix { x: 5, y: 10.4 };
```

#### 在方法定义中使用泛型

在 `impl` 块中，我们也可以使用泛型来为泛型类型定义方法。

```rust
// in src/advanced/generics.rs
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}
```

- `impl<T>`: 这表示我们正在为泛型类型 `Point<T>` 实现方法。

我们甚至可以为特定的具体泛型类型实现方法。例如，我们可以为一个 `Point<f32>` 类型的实例添加一个计算到原点距离的方法，而其他 `Point<T>` 实例则没有这个方法。

```rust
// in src/advanced/generics.rs
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
```

#### 泛型与性能

你可能会担心使用泛型会带来运行时开销。但好消息是，Rust 通过在编译时进行 **单态化 (monomorphization)** 来解决这个问题。单态化是编译器将泛型代码替换为使用时指定的具体类型的代码的过程。这意味着你写的泛型代码在运行时和手写具体类型的代码一样快。

### 10. 闭包 (Closures)

闭包 (Closures) 是可以捕获其环境的匿名函数。它们是 Rust 中一个非常灵活和强大的功能，尤其在与迭代器等函数式编程风格的功能结合使用时。

#### 闭包的定义

闭包的语法简洁，使用 `|` 包围参数列表，后跟一个表达式或代码块。

```rust
// in src/advanced/closures.rs
let expensive_closure = |num| {
    println!("calculating slowly...");
    std::thread::sleep(std::time::Duration::from_secs(2));
    num
};
```

- `|num|`: 这是闭包的参数列表。
- `{...}`: 这是闭包的主体。如果主体只是一个表达式，可以省略花括号。
- Rust 编译器会为闭包的参数和返回值推断类型，但你也可以显式地添加类型注解。

#### 捕获环境

闭包的强大之处在于它们可以“捕获”其定义所在作用域中的变量。闭包可以通过三种方式捕获变量，这三种方式直接对应函数获取参数的三种方式：

1.  **不可变借用 (`&T`)**: 闭包默认以这种方式捕获变量。
2.  **可变借用 (`&mut T`)**: 如果闭包修改了捕获的变量，它会以可变方式借用。
3.  **获取所有权 (`T`)**: 使用 `move` 关键字，强制闭包获取其捕获变量的所有权。

```rust
// in src/advanced/closures.rs
let x = 4;
let equal_to_x = |z| z == x; // 不可变借用 x
let y = 4;
assert!(equal_to_x(y));
```

##### `move` 关键字

当你希望闭包获取其捕获的变量的所有权时，可以在参数列表前使用 `move` 关键字。这通常在将闭包传递给新线程时使用，以确保数据在线程间安全地移动。

```rust
// in src/advanced/closures.rs
let x = vec![1, 2, 3];
let equal_to_x = move |z| z == x;
// println!("can't use x here: {:?}", x); // 错误！x 的所有权已转移到闭包中
let y = vec![1, 2, 3];
assert!(equal_to_x(y));
```

#### 闭包作为函数参数

闭包可以像其他类型一样作为函数的参数。为了接受一个闭包作为参数，我们需要使用泛型和 `Fn` 系列的 trait：

- `FnOnce`: 适用于能被调用一次的闭包。所有闭包都至少实现了这个 trait。
- `FnMut`: 适用于不会移动捕获的变量，但可能会修改它们的闭包。
- `Fn`: 适用于既不移动也不修改捕获变量的闭包。

```rust
// in src/advanced/closures.rs
fn apply_function<F>(f: F) -> i32
where
    F: FnOnce() -> i32, // 这个函数接受一个不带参数并返回 i32 的闭包
{
    f()
}
```

#### 迭代器与闭包

闭包最常见的用途之一是与迭代器一起使用。迭代器是实现了 `Iterator` trait 的类型，它提供了一系列的方法，其中许多都接受闭包作为参数，以对集合进行各种操作。

```rust
// in src/advanced/closures.rs
let v1 = vec![1, 2, 3];

// `map` 方法接受一个闭包，对迭代器的每个元素执行操作
let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

println!("v2: {:?}", v2); // 输出: v2: [2, 3, 4]
```

闭包和迭代器的组合提供了一种富有表现力且高效的方式来处理集合，是 Rust 函数式编程思想的核心体现。

### 11. 线程 (Threads)

Rust 通过标准库 `std::thread` 提供了创建和管理操作系统原生线程的能力，让你可以并发地执行代码。并发编程能够提高程序性能，但同时也带来了新的挑战，如竞态条件、死锁等。Rust 的所有权和类型系统在编译时就能帮助你避免许多常见的并发问题。

#### 创建新线程

使用 `thread::spawn` 函数可以创建一个新的线程。它接受一个闭包作为参数，新线程将执行这个闭包中的代码。

```rust
// in src/advanced/threads.rs
use std::thread;
use std::time::Duration;

let handle = thread::spawn(|| {
    for i in 1..10 {
        println!("hi number {} from the spawned thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
});

// 主线程会继续执行
for i in 1..5 {
    println!("hi number {} from the main thread!", i);
    thread::sleep(Duration::from_millis(1));
}

// 调用 join 会阻塞当前线程，直到 handle 所代表的线程执行完毕
handle.join().unwrap();
```

#### 使用 `move` 闭包

在创建线程时，我们经常需要将数据的所有权从主线程转移到子线程。这可以通过在闭包前使用 `move` 关键字来实现，它会强制闭包获取其使用的外部变量的所有权。

```rust
// in src/advanced/threads.rs
let v = vec![1, 2, 3];

// 使用 move 将 v 的所有权转移到子线程
let handle = thread::spawn(move || {
    println!("Here's a vector from the spawned thread: {:?}", v);
});

handle.join().unwrap();
```

#### 消息传递 (Message Passing)

Rust 提倡一种通过消息传递来在线程间进行通信的并发模型，即“不要通过共享内存来通信；而是通过通信来共享内存”。

`std::sync::mpsc` 模块提供了“多生产者，单消费者”(multiple producer, single consumer) 的通道 (channel)。

```rust
// in src/advanced/threads.rs
use std::sync::mpsc;

let (tx, rx) = mpsc::channel(); // tx 是发送端，rx 是接收端

let tx1 = tx.clone(); // 克隆发送端，实现多生产者

thread::spawn(move || {
    let vals = vec![
        String::from("hi"),
        String::from("from"),
        String::from("the"),
        String::from("thread"),
    ];
    for val in vals {
        tx1.send(val).unwrap();
        thread::sleep(Duration::from_secs(1));
    }
});

thread::spawn(move || {
    let vals = vec![
        String::from("more"),
        String::from("messages"),
        String::from("for"),
        String::from("you"),
    ];
    for val in vals {
        tx.send(val).unwrap();
        thread::sleep(Duration::from_secs(1));
    }
});

// 在主线程中接收消息
for received in rx {
    println!("Got: {}", received);
}
```

#### 共享状态并发 (Shared-State Concurrency)

尽管消息传递是很好的并发处理方式，但在某些情况下，我们可能仍然需要线程间共享状态。

##### `Mutex<T>`：互斥锁

`Mutex<T>` (mutual exclusion) 是一种智能指针，它确保在任何时刻只有一个线程可以访问其内部的数据。当一个线程需要访问数据时，它必须首先请求并获取 `Mutex` 的锁。当该线程完成数据操作后，它会释放锁，以便其他线程可以获取锁并访问数据。

##### `Arc<T>`：原子引用计数

`Arc<T>` (Atomically Referenced Counter) 是一个线程安全的引用计数智能指针。当你需要在多个线程之间共享数据的所有权时，可以使用 `Arc<T>`。它会记录值的引用数量，只有当引用数量为零时，值才会被清理。

结合 `Arc<T>` 和 `Mutex<T>`，我们可以安全地在多个线程之间共享和修改数据。

```rust
// in src/advanced/threads.rs
use std::sync::{Mutex, Arc};

let counter = Arc::new(Mutex::new(0));
let mut handles = vec![];

for _ in 0..10 {
    let counter = Arc::clone(&counter);
    let handle = thread::spawn(move || {
        let mut num = counter.lock().unwrap(); // 获取锁
        *num += 1;
    }); // 锁在这里被释放
    handles.push(handle);
}

for handle in handles {
    handle.join().unwrap();
}

println!("Result: {}", *counter.lock().unwrap()); // 输出: Result: 10
```

通过这些并发原语，Rust 在编译时提供了强大的保证，帮助你编写出正确且高效的并发代码。

### 12. 宏 (Macros)

宏（Macros）是 Rust 元编程（metaprogramming）的主要方式之一，它允许你编写能够生成其他代码的代码。宏在编译时展开，这意味着它们在代码被编译成最终的二进制文件之前，会被替换为它们生成的代码。

Rust 中主要有两种类型的宏：

1.  **声明宏 (Declarative Macros)**：使用 `macro_rules!` 定义，它们通过匹配模式来生成代码，类似于 `match` 表达式。
2.  **过程宏 (Procedural Macros)**：更像函数，它们接收 Rust 代码作为输入，对其进行操作，然后产生新的代码作为输出。

#### 声明宏 (`macro_rules!`)

声明宏允许你定义简单的、基于模式匹配的宏。我们常用的 `println!` 和 `vec!` 就是声明宏。

下面是一个简化版的 `vec!` 宏的实现示例：

```rust
// in src/advanced/macros.rs

// 定义一个名为 my_vec 的宏
macro_rules! my_vec {
    // 匹配模式：零个或多个表达式 ($x)，由逗号分隔
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            // 对每个匹配到的表达式 $x 重复以下代码
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

let v = my_vec![1, 2, 3]; // 使用宏
println!("my_vec: {:?}", v);
```

- `$($x:expr),*` 定义了一个匹配模式，它会匹配零个或多个由逗号分隔的表达式。
- `$(...)*` 是一个重复结构，它会将其中的代码对每个匹配到的 `$x` 重复一次。

#### 过程宏 (Procedural Macros)

过程宏允许你编写更复杂的代码转换逻辑。它们有三种类型：

1.  **自定义 `#[derive]` 宏**：可以为结构体和枚举自动实现 trait。例如，`serde` 库的 `#[derive(Serialize, Deserialize)]` 就是一个典型的例子。
2.  **属性宏 (Attribute-like macros)**：允许你创建自定义的属性，可以附加到任何项上。例如，`#[route(GET, "/")]`。
3.  **函数式宏 (Function-like macros)**：看起来像函数调用，但可以操作传递给它们的原始 token。例如 `sql!` 宏可以验证 SQL 语句的语法。

过程宏的定义比声明宏更复杂，需要创建一个特殊的 `proc-macro` crate 类型的库。

#### 为什么使用宏？

-   **减少代码重复 (DRY)**：宏可以帮助你消除样板代码。
-   **创建领域特定语言 (DSLs)**：宏可以用来创建更具表现力的语法。
-   **可变参数函数**：像 `println!` 这样的函数需要宏来实现，因为它接受可变数量的参数。

宏是 Rust 一个非常强大的功能，但同时也应该谨慎使用，因为它们可能会使代码更难理解和调试。





### 6. 枚举 (Enums)

枚举 (`enum`) 允许你通过列举所有可能的值来定义一个类型。它是一种强大的方式，可以将相关的概念组合到一个类型中。

#### 定义枚举

一个枚举可以包含简单的变体，也可以包含关联数据。

```rust
// in src/basics/enums.rs

// 一个简单的枚举
enum IpAddrKind {
    V4,
    V6,
}

// 枚举的变体可以关联数据
enum IpAddr {
    V4(String),
    V6(String),
}

let home = IpAddr::V4(String::from("127.0.0.1"));

// 变体可以有不同类型和数量的关联数据
enum Message {
    Quit,
    Move { x: i32, y: i32 }, // 关联一个匿名结构体
    Write(String),            // 关联一个 String
    ChangeColor(i32, i32, i32), // 关联三个 i32
}
```

与结构体一样，你也可以使用 `impl` 块为枚举定义方法。

```rust
// in src/basics/enums.rs
impl Message {
    fn call(&self) {
        // 方法体
    }
}

let m = Message::Write(String::from("hello"));
m.call();
```

#### `Option` 枚举

`Option` 是 Rust 标准库中一个非常重要的枚举，它被用来编码一个值可以是“有东西”或“什么都没有”的场景。这解决了其他许多语言中普遍存在的“空值 (null)”问题。

```rust
// Option<T> 的定义
enum Option<T> {
    Some(T), // 表示存在一个 T 类型的值
    None,    // 表示不存在值
}

let some_number = Some(5);
let absent_number: Option<i32> = None;
```

`Option<T>` 是如此有用，以至于它被包含在了 prelude（预导入模块）中，你不需要显式地引入它。它的变体 `Some` 和 `None` 也是如此。

#### `match` 控制流运算符

`match` 是 Rust 中一个极其强大的控制流运算符。它允许你将一个值与一系列的模式进行比较，并根据匹配的模式执行相应的代码。可以把它想象成一个“超级 `if`”。

`match` 的一个关键特性是它是 **穷尽的 (exhaustive)**。这意味着你必须为所有可能的情况编写分支。这可以防止你忘记处理某些情况，从而避免 bug。

```rust
// in src/basics/enums.rs
#[derive(Debug)]
enum UsState { Alabama, Alaska }

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState), // Quarter 变体关联了一个 UsState 值
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}
```

#### `if let` 语法

当你只关心 `match` 中的某一个分支时，使用 `match` 可能会有点冗长。`if let` 语法是 `match` 的一个语法糖，它允许你以一种不那么冗长的方式来处理只匹配一个模式而忽略其余模式的情况。

```rust
// in src/basics/enums.rs
let config_max = Some(3u8);

// 使用 match
match config_max {
    Some(max) => println!("The maximum is configured to be {}", max),
    _ => (), // 对于所有其他情况，我们什么都不做
}

// 使用 if let，更简洁
if let Some(max) = config_max {
    println!("The maximum is configured to be {}", max);
}
```

`if let` 也可以与 `else` 结合使用，`else` 块的代码与 `match` 中的 `_` 分支的代码相同。

### 5. 结构体 (Structs)

结构体 (`struct`) 是一种自定义数据类型，它允许你将多个相关的值组合在一起，形成一个更有意义的单元。这类似于面向对象语言中的对象的属性。

#### 定义和实例化结构体

使用 `struct` 关键字来定义一个结构体，并在花括号中为每个字段指定名称和类型。

```rust
// in src/basics/structs.rs
#[derive(Debug)] // 这个注解让我们可以使用 {:?} 或 {:#?} 来打印结构体
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// 创建一个 User 结构体的实例
let mut user1 = User {
    email: String::from("someone@example.com"),
    username: String::from("someusername123"),
    active: true,
    sign_in_count: 1,
};

// 可以通过点号访问实例的字段
user1.email = String::from("anotheremail@example.com");
```

- **字段初始化简写 (Field Init Shorthand)**: 如果函数参数或变量的名称与结构体字段的名称完全相同，你可以使用这种简写语法，避免重复。

```rust
// in src/basics/structs.rs
fn build_user(email: String, username: String) -> User {
    User {
        email,    // 等同于 email: email,
        username, // 等同于 username: username,
        active: true,
        sign_in_count: 1,
    }
}
```

- **结构体更新语法 (Struct Update Syntax)**: 当你想基于一个旧的实例创建一个新的实例，并且只改变其中几个字段时，这个语法非常有用。它使用 `..` 来指定未显式设置的字段应从另一个实例中获取。

```rust
// in src/basics/structs.rs
let user3 = User {
    email: String::from("user3@example.com"),
    ..user1 // user1 的其余字段将被复制到 user3
};
```

#### 元组结构体 (Tuple Structs)

元组结构体是一种看起来像元组的结构体。它有结构体名称，但字段没有名称，只有类型。当你想要给整个元组一个名字，并使其成为一个独立的类型，但又不需要为每个字段命名时，元组结构体很有用。

```rust
// in src/basics/structs.rs
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

let black = Color(0, 0, 0);
let origin = Point(0, 0, 0);
```
`Color` 和 `Point` 是不同的类型，即使它们内部都由三个 `i32` 组成。

#### 类单元结构体 (Unit-Like Structs)

你还可以定义没有任何字段的结构体！这被称为“类单元结构体”，因为它类似于 `()`，即单元类型。当你想在一个类型上实现某个 trait，但又不需要在类型中存储任何数据时，这种结构体非常有用。

```rust
// in src/basics/structs.rs
struct AlwaysEqual;

let subject = AlwaysEqual;
```

#### 方法 (Methods)

方法与函数类似，但它们在结构体（或枚举、trait）的上下文中定义，并且它们的第一个参数总是 `self`，代表调用该方法的实例。

使用 `impl` 块（implementation block）来为结构体定义方法。

```rust
// in src/basics/structs.rs
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // `&self` 是 `self: &Self` 的简写。
    // 这是一个借用，方法只是读取 Rectangle 的数据，而不是获取其所有权。
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // 方法可以有多个参数
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

let rect1 = Rectangle { width: 30, height: 50 };
println!("The area is {}", rect1.area());
```

#### 关联函数 (Associated Functions)

`impl` 块中也可以定义不以 `self` 作为第一个参数的函数。这些被称为“关联函数”，因为它们与结构体关联。它们通常用作构造函数。

```rust
// in src/basics/structs.rs
impl Rectangle {
    // 这是一个关联函数，因为它没有 `self` 参数
    // 它通常用于创建结构体的新实例
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}

// 使用 `::` 语法来调用关联函数
let sq = Rectangle::square(3);
```
`String::from` 就是一个我们已经使用过的关联函数。