pub mod mytest_string {

    /// The pointer to the heap region you reserved.
    /// The length of the string, i.e. how many bytes are in the string.
    /// The capacity of the string, i.e. how many bytes have been reserved on the heap.


    /// String 转换成 &str 的几种方式
    /// 注意切片的方式在 UTF8 存在风险，因为必须落在字符的边界，比如说中文，所以不建议对字符串切片
    /// 同时，Rust String 也不允许进行索引操作
    pub fn test() {
        let s = String::from("hello world!");
        say_hello(&s);
        say_hello(s.as_str());
        say_hello(&s[0..2]);
    }

    fn say_hello(s : &str) {
        println!("{}", s);
    }

    /// String 的一些操作
    /// 
    /// - 追加: push (追加一个字符)  push_str （追加一个字符串)
    /// 
    /// - 替换：replace replacen  replace_range
    /// 
    /// - 插入：insert  insert_str
    /// 
    /// - 删除：pop  remove  truncate  clear
    /// 
    /// - 连接：+  +=  format!
    pub fn string_opera() {
        let mut s = String::from("hello");
        //追加
        s.push(' ');
        s.push_str("push!");
        println!("{}", &s);
        //替换
        let replace_s =  s.replace("push", "replace");
        println!("{}", replace_s);
        let string_replace = "I like rust. Learning rust is my favorite!";
        let new_string_replacen = string_replace.replacen("rust", "RUST", 2);
        println!("{}", new_string_replacen);
        s.replace_range(0..5, "HELLO"); // 与上面的两个替换函数不一致，直接替换当前字符串
        //插入
        s.insert(s.len(), '!');
        s.insert_str(0, "insert ");
        println!("{}", s);
        //删除
        let c = s.pop(); // 如果字符串为空，返回 None
        dbg!(c);
        dbg!(String::from("").pop());
        println!("remove {}", s.remove(0));
        s.truncate(s.len() - 1);
        println!("after truncate: {}", s);
        s.clear();
        //连接
        let mut str_append = s + " append!"; // s 所有权已被转移，不能再被使用
        str_append += "!";
        println!("{}", str_append);
        let result = format!("{} format {}", str_append, "!!!");
        println!("{}", result);
    }

    #[cfg(test)]
    mod tests {
        use std::mem::size_of;

        #[test]
        fn string_size() {
            assert_eq!(size_of::<String>(), 24);
        }
    }
}