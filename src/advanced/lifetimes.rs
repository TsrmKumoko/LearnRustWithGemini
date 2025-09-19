pub fn lifetimes_example() {
    // 生命周期（Lifetimes）
    // 生命周期是 Rust 中一个用来确保引用有效性的概念。它通过将引用的有效作用域与
    // 数据的所有权作用域进行比较来防止悬垂引用。

    // 生命周期注解
    // 生命周期注解不会改变任何引用的生命周期，而是描述了多个引用生命周期之间的关系，
    // 而不影响生命周期本身。

    // 这是一个泛型生命周期参数 `'a` 的例子
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }

    let string1 = String::from("long string is long");
    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }

    // 结构体定义中的生命周期注解
    // 结构体可以持有引用，但这种情况下需要为结构体定义添加生命周期注解。
    #[derive(Debug)]
    struct ImportantExcerpt<'a> {
        part: &'a str,
    }

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    println!("ImportantExcerpt: {:#?}", i);

    // 生命周期省略规则
    // Rust 编译器使用三条规则来确定何时不需要显式地注解生命周期。
    // 1. 每一个是引用的参数都有它自己的生命周期参数。
    // 2. 如果只有一个输入生命周期参数，那么它被赋予所有输出生命周期参数。
    // 3. 如果方法有多个输入生命周期参数，不过其中之一是 `&self` 或 `&mut self`，
    //    那么 `self` 的生命周期被赋予所有输出生命周期参数。

    // 方法定义中的生命周期
    impl<'a> ImportantExcerpt<'a> {
        fn level(&self) -> i32 {
            3
        }

        fn announce_and_return_part(&self, announcement: &str) -> &str {
            println!("Attention please: {}", announcement);
            self.part
        }
    }

    // 静态生命周期（'static）
    // `'static` 生命周期是整个程序运行期间都有效的生命周期。
    // 所有的字符串字面值都拥有 `'static` 生命周期。
    let s: &'static str = "I have a static lifetime.";
    println!("{}", s);

    // 结合泛型类型参数、trait bounds 和生命周期
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

    let announcement = "Today's news";
    let result = longest_with_an_announcement(string1.as_str(), "short", announcement);
    println!("The longest string with announcement is: {}", result);
}