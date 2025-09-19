pub fn generics_example() {
    // 泛型（Generics）
    // 泛型是代码中具体类型或其他属性的抽象替代。它们允许你编写更灵活、可重用的代码。

    // 函数中的泛型
    // 我们可以编写一个函数，它适用于任何类型的数据，而不是只适用于特定类型。
    fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
        let mut largest = list[0];

        for &item in list.iter() {
            if item > largest {
                largest = item;
            }
        }

        largest
    }

    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("The largest char is {}", result);

    // 结构体定义中的泛型
    // 结构体可以使用泛型类型参数来定义，使其可以存储不同类型的数据。
    #[derive(Debug)]
    struct Point<T> {
        x: T,
        y: T,
    }

    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
    println!("Integer Point: {:#?}", integer);
    println!("Float Point: {:#?}", float);

    // 结构体定义中的多个泛型类型参数
    #[derive(Debug)]
    struct PointMix<T, U> {
        x: T,
        y: U,
    }

    let p = PointMix { x: 5, y: 10.4 };
    println!("Mixed Point: {:#?}", p);

    // 枚举定义中的泛型
    // 枚举也可以使用泛型类型参数。
    enum Option<T> {
        Some(T),
        None,
    }

    enum Result<T, E> {
        Ok(T),
        Err(E),
    }

    // 方法定义中的泛型
    // 可以在结构体或枚举的方法中使用泛型。
    impl<T> Point<T> {
        fn x(&self) -> &T {
            &self.x
        }
    }

    println!("integer.x = {}", integer.x());

    // 只有当 Point<f32> 类型时才有的方法
    impl Point<f32> {
        fn distance_from_origin(&self) -> f32 {
            (self.x.powi(2) + self.y.powi(2)).sqrt()
        }
    }

    println!("Distance from origin: {}", float.distance_from_origin());

    // 泛型参数的性能
    // Rust 通过在编译时执行泛型代码的单态化（monomorphization）来保证运行时性能。
    // 单态化是编译器将泛型代码替换为特定具体类型的代码的过程。
}