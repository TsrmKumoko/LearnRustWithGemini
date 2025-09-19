#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// 元组结构体（Tuple Structs）
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// 没有任何字段的类单元结构体
struct AlwaysEqual;

pub fn structs_example() {
    // 结构体（Structs）
    // 结构体是一种自定义数据类型，允许你将多个相关的值组合成一个有意义的组合。

    // 创建结构体实例
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    // 访问和修改结构体字段
    user1.email = String::from("anotheremail@example.com");

    println!("User 1: {:#?}", user1);

    // 使用字段初始化简写语法
    let user2 = build_user(String::from("user2@example.com"), String::from("user2"));
    println!("User 2: {:#?}", user2);

    // 使用结构体更新语法从其他实例创建实例
    let user3 = User {
        email: String::from("user3@example.com"),
        ..user1
    };
    println!("User 3: {:#?}", user3);

    // 使用没有命名字段的元组结构体来创建不同的类型
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    println!("Color: ({}, {}, {})", black.0, black.1, black.2);
    println!("Point: ({}, {}, {})", origin.0, origin.1, origin.2);

    // 没有任何字段的类单元结构体
    let subject = AlwaysEqual;

    // 示例：计算矩形的面积
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    // 带有更多参数的方法
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    // 关联函数（Associated Functions）
    // 关联函数是与类型关联的函数，但它们不作用于类型的实例。
    // `String::from` 就是一个关联函数。
    let sq = Rectangle::square(3);
    println!("Square: {:#?}", sq);
}

fn build_user(email: String, username: String) -> User {
    User {
        email, // 字段初始化简写
        username, // 字段初始化简写
        active: true,
        sign_in_count: 1,
    }
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// 方法（Methods）
// 方法与函数类似：它们使用 `fn` 关键字和名称声明，可以拥有参数和返回值。
// 但是方法在结构体（或枚举或 trait 对象）的上下文中定义，
// 并且它们的第一个参数总是 `self`，它代表调用该方法的结构体实例。
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // 关联函数
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}