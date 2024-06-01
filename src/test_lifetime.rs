pub mod my_test_lifetime {

    /// - 生命周期在函数中，如果与传入参数有关，那么生命周期标注 'a 为两个中生命周期最短的那个；
    pub fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }


}

#[cfg(test)]
mod lifetime_tests {
    use crate::test_lifetime::my_test_lifetime;

    #[test]
    fn test_longest() {
        let string1 = String::from("abcd");
        let string2 = "xyz";
    
        let result = my_test_lifetime::longest(string1.as_str(), string2);
        println!("The longest string is {}", result);
    }
}