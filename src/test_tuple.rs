pub mod my_test_tuple {
    
    pub fn calculate_length(s: String) -> (String, usize) {
        let len = s.len();
        (s, len)
    }


    #[cfg(test)]
    mod tests {
        #[test]
        fn test_tuple() {
            let cat = ("Furry McFurson", 3.5);
            let (name, age) = cat;
            println!("{} is {} years old.", name, age);
            // 通过索引获取元组的成员
            assert_eq!(cat.1, 3.5);
            let _t0: (u8, i16) = (0, -1);
            // 元组的成员还可以是一个元组
            let _t1: (u8, (i16, u32)) = (0, (-1, 1));
            // 填空让代码工作
            let t: (u8, u16, i64, &str, String) = (1u8, 2u16, 3i64, "hello", String::from(", world"));
            println!("{}{}", t.3, t.4);
            // let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
            // println!("too long tuple: {:?}", too_long_tuple);
            let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12);
            println!("too long tuple: {:?}", too_long_tuple);
        }
    }
}
