pub mod my_struct {
    #[derive(Debug)]
    pub struct User {
        active: bool,
        username: String,
        sign_in_count: u64,
    }

    pub fn create_struct() -> User {
        User { active: true, username: String::from("Norman"), sign_in_count: 01 }
    }

    pub fn build_struct(username: String, sign_in_count: u64) -> User {
        User { active: true, username, sign_in_count }
    }

    pub fn copy_struct(user: User) -> User {
        User { ..user }
    }

    /// 结构体中的函数方法
    /// 我们使用 `&self` 替代 `user: &User`，&self 其实是 self: &Self 的简写（注意大小写）。
    /// 
    /// - `Self`: 在一个 impl 块内，`Self` 指代被实现方法的结构体类型；
    /// - `self`: 指代此类型的实例，换句话说，`self` 指代的是 `User` 结构体实例。同时代表所有权转移到该方法中；
    /// - `&self`: 表示该方法对 `User` 的不可变借用;
    /// - `&mut self`: 表示可变借用
    impl User {
        ///这种定义在 impl 中且没有 `self` 的函数被称之为关联函数： 因为它没有 `self`，不能用 f.new() 的形式调用，
        /// 因此它是一个函数而不是方法，它又在 impl 中，与结构体紧密关联，因此称为关联函数。
        pub fn new(active: bool, username: String, sign_in_count: u64) -> User {
            User {
                active,
                username,
                sign_in_count
            }
        }

        /// 方法可以与成员变量同名，更多用于 getter 的访问器情况
        pub fn sign_in_count(&self) -> u64 {
            self.sign_in_count
        }
    }

    fn add<T: std::ops::Add<Output = T>>(a:T, b:T) -> T {
        a + b
    }

    ///泛型类
    pub struct Point<T: std::ops::Mul<Output = T>> {
        x: T,
        y: T,
    }

    impl<T: std::ops::Mul<Output = T>> Point<T> {
        pub fn x(&self) -> &T {
            &self.x
        }

        // pub fn area(&self) -> T {
        //     let res = self.x * self.y;
        //     res
        // }
    }

    ///特征：类似于 Java 的接口与 C++ 的纯虚类
    pub trait Summary {
        fn summarize(&self) -> String;

        ///特征内的方法提供默认实现，可选择是否自己实现该方法
        fn default(&self) {
            println!("this is default trait!");
        }
    }

    impl Summary for User {
        fn summarize(&self) -> String {
            format!("name: {}, sign_in_count: {}", self.username, self.sign_in_count)
        }
    }

    ///以特征作为参数传入，只有实现该特征的结构体才能被传入；
    /// 
    /// 特征约束：`&impl Summary` 是一个语法糖，真正的函数原型为： `pub fn notify<T: Summary>(item: &T)`
    /// 
    /// 多重约束： pub fn notify<T: Summary + Display>(item: &T) {}
    pub fn notify(item: &impl Summary) {
        println!("{}", item.summarize());
        item.default();
    }

    pub fn return_summary() -> impl Summary {
        User::new(true, String::from("Jack"), 007)
    }

}

#[cfg(test)]
mod tests {
    use super::my_struct;

    #[test]
    fn test_summary() {
        let summ = my_struct::return_summary();
        my_struct::notify(&summ);
    }
}