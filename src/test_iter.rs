pub mod my_test_iter {
    /// 数组实现了 IntoIterator 特征，Rust 通过 for 语法糖，自动把实现了该特征的数组类型转换为迭代器
    /// IntoIterator 特征拥有一个 into_iter 方法，因此我们还可以显式的把数组转换成迭代器
    /// for 循环通过不停调用迭代器上的 next 方法，来获取迭代器中的元素
    /// - next 方法返回的是 Option 类型，当有值时返回 Some(i32)，无值时返回 None
    /// - 遍历是按照迭代器中元素的排列顺序依次进行的，因此我们严格按照数组中元素的顺序取出了 Some(1)，Some(2)，Some(3)
    /// - 手动迭代必须将迭代器声明为 mut 可变，因为调用 next 会改变迭代器其中的状态数据（当前遍历的位置等），
    /// 而 for 循环去迭代则无需标注 mut，因为它会帮我们自动完成
    pub fn iter() {
        let arr = [1, 2, 3, 4];
        for v in arr.into_iter() {
            println!("{}", v);
        }
        let mut arr_iter = arr.iter();
        println!("{}", arr_iter.next().unwrap());
    }

    #[cfg(test)]
    mod test {
        use super::iter;
        #[test]
        fn test_iter() {
            iter();
        }
    }
}
