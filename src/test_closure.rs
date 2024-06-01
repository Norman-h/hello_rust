pub mod my_test_closure {
    fn sample(a: i16) -> i16 {
        let sum = |b| a + b;
        sum(10)
    }

    struct Cacher<T, E>
    where
        T: Fn(E) -> E,
        E: Copy,
    {
        query: T,
        value: Option<E>,
    }

    impl<T, E> Cacher<T, E>
    where
        T: Fn(E) -> E,
        E: Copy,
    {
        fn new(query: T) -> Cacher<T, E> {
            Cacher { query, value: None }
        }

        // 先查询缓存值 `self.value`，若不存在，则调用 `query` 加载
        fn value(&mut self, arg: E) -> E {
            match self.value {
                Some(v) => v,
                None => {
                    let v = (self.query)(arg);
                    self.value = Some(v);
                    v
                }
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::{sample, Cacher};

        #[test]
        fn test_sample() {
            assert_eq!(sample(5), 15);
        }

        #[test]
        fn test_cacher() {
            let mut cacher = Cacher::new(|a| a);
            // cacher.query = query;
            cacher.value = None;
            println!("{}", cacher.value(32));
        }
    }
}
