pub mod my_enum {

    /// 枚举
    #[derive(Debug)]
    pub enum PokerCard {
        Clubs(u8),
        Spades(u8),
        Diamonds(char),
        Hearts(char),
    }

    pub fn get_card() {
        let card = PokerCard::Clubs(8);
        let res = match card {
            PokerCard::Clubs(num) => {
                println!("{}", num);
                num
            },
            PokerCard::Spades(num) => {
                println!("{}", num);
                num
            },
            _  => {
                // println!("{}", c);
                0
            },
        };
        if let PokerCard::Clubs(10) = card {
            println!("card is {:?}", card);
        }
        let x = Some(5);
        println!("option: {:?}", test_option(x));
    }

    fn test_option(x: Option<i32>) -> Option<i32> {
        match x {
            None => {
                None
            },
            Some(n) => {
                Some(n + 1)
            }
        }
    }

    impl PokerCard {

        /// TODO
        /// 枚举同样可以实现方法，但是还不清楚如何去使用存储在枚举类中的值
        pub fn somefn(&self) {

        }
        
    }
}