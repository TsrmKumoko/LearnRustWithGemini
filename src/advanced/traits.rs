pub fn traits_example() {
    // 特征（Traits）
    // 特征定义了共享行为。它们类似于其他语言中的接口（interfaces）。
    // 你可以为任何数据类型实现特征。

    // 定义一个特征
    pub trait Summary {
        fn summarize(&self) -> String;

        // 默认实现
        fn summarize_default(&self) -> String {
            String::from("(Read more...)")
        }
    }

    // 为结构体实现特征
    pub struct NewsArticle {
        pub headline: String,
        pub location: String,
        pub author: String,
        pub content: String,
    }

    impl Summary for NewsArticle {
        fn summarize(&self) -> String {
            format!("{}, by {} ({})", self.headline, self.author, self.location)
        }
    }

    pub struct Tweet {
        pub username: String,
        pub content: String,
        pub reply: bool,
        pub retweet: bool,
    }

    impl Summary for Tweet {
        fn summarize(&self) -> String {
            format!("{}: {}", self.username, self.content)
        }
    }

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());
    println!("1 new tweet (default): {}", tweet.summarize_default());

    let article = NewsArticle {
        headline: String::from("Penguins win Stanley Cup in overtime!"),
        location: String::from("Pittsburgh, PA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best hockey team in the NHL.",
        ),
    };

    println!("New article available! {}", article.summarize());

    // 将特征作为参数
    // 通过 `impl Trait` 语法，函数可以接受任何实现了特定特征的类型作为参数。
    pub fn notify(item: &impl Summary) {
        println!("Breaking news! {}", item.summarize());
    }

    notify(&tweet);
    notify(&article);

    // Trait Bound 语法
    // `impl Trait` 语法糖的完整形式是 Trait Bound。
    pub fn notify_trait_bound<T: Summary>(item: &T) {
        println!("Breaking news (trait bound)! {}", item.summarize());
    }

    notify_trait_bound(&tweet);

    // 多个 Trait Bound
    // 可以要求类型实现多个特征。
    // pub fn notify_multi<T: Summary + Display>(item: &T) {
    //     println!("Breaking news! {}", item.summarize());
    // }

    // Where 从句
    // 当有多个 Trait Bound 时，`where` 从句可以使函数签名更清晰。
    // fn some_function<T, U>(t: &T, u: &U) -> i32
    //     where T: Display + Clone,
    //           U: Clone + Debug
    // {
    //     // ...
    // }

    // 返回实现了特征的类型
    // 函数可以返回一个实现了特定特征的类型。
    fn returns_summarizable() -> impl Summary {
        Tweet {
            username: String::from("horse_ebooks"),
            content: String::from("of course, as you probably already know, people"),
            reply: false,
            retweet: false,
        }
    }

    let summary_tweet = returns_summarizable();
    println!("Returned summarizable: {}", summary_tweet.summarize());

    // 使用大的或复杂的类型作为参数
    // 特征对象（Trait Objects）
    // 特征对象允许你编写可以处理实现了不同特征的多种类型的代码。
    // `Box<dyn Summary>` 是一个特征对象。
    pub fn notify_trait_object(item: &dyn Summary) {
        println!("Breaking news (trait object)! {}", item.summarize());
    }

    notify_trait_object(&tweet);
    notify_trait_object(&article);
}