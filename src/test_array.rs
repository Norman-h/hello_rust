///在 Rust 中，最常用的数组有两种，第一种是速度很快但是长度固定的 array，第二种是可动态增长的但是有性能损耗的 Vector
pub mod my_test_array {
    
    ///数组类型是通过方括号语法声明，[i32; 5] i32 是元素类型，分号后面的数字 5 是数组长度
    pub fn slice_array(arr: &[i32; 5], slice_num: (usize, usize)) -> &[i32] {
        let (begin, end) = slice_num;
        &arr[begin..end]
    }

    /// array 底层就是不断的Copy出来的,
    /// 对于复杂类型来说，由于没有实现 Copy trait，所以使用 from_fn 一个个创建出来
    pub fn test_string_array() {
        // let array = [String::from("hello rust!"); 8];
        let array: [String; 8] = std::array::from_fn(|_i| String::from("hello rust!"));
        println!("{:#?}", array);
    }

    #[cfg(test)]
    mod tests {
        use super::{slice_array, test_string_array};
        #[test]
        fn slice_out_of_array() {
            test_string_array();
            let a = [1, 2, 3, 4, 5];
            let nice_slice = &a[1..4];
            let nice_slice_other = slice_array(&a, (1, 4));
            assert_eq!([2, 3, 4], nice_slice);
            assert_eq!([2, 3, 4], nice_slice_other)
        }
    }
}