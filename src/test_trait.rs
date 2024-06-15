pub mod mytest_tarit {
    use std::ops::{Add, Mul, Sub};

    #[derive(Debug)]
    pub struct Order {
        x: i32,
        y: i32
    }

    impl Add for Order {
        type Output = Self;
        fn add(self, rhs: Self) -> Self {
            Self {
                x: self.x + rhs.x,
                y: self.y + rhs.y
            }
        }
    }

    impl Sub for Order {
        type Output = Self;
        fn sub(self, rhs: Self) -> Self {
            Self {
                x: self.x - rhs.x,
                y: self.y - rhs.y
            }
        }
    }

    impl Mul for Order {
        type Output = Self;
        fn mul(self, rhs: Self) -> Self {
            Self {
                x: self.x * rhs.x,
                y: self.y * rhs.y
            }
        }
    }

    impl PartialEq for Order {
        fn eq(&self, other: &Self) -> bool {
            self.x == other.x && self.y == other.y
        }

        fn ne(&self, other: &Self) -> bool {
            self.x != other.x || self.y != other.y
        }
    }

    impl PartialOrd for Order {
        fn ge(&self, other: &Self) -> bool {
            self.x >= other.x
        }

        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            if self.x > other.x {
                Some(std::cmp::Ordering::Greater)
            } else if self.x < other.x {
                Some(std::cmp::Ordering::Less)
            } else {
                Some(std::cmp::Ordering::Equal)
            }
        }
    }

    #[cfg(test)]
    mod test {
        use super::Order;

        #[test]
        fn test_add() {
            let order1 = Order {x: 1, y: 2};
            let order2 = Order {x: 2, y: 2};
            let order3 = Order {x: 3, y: 4};
            assert_eq!(order3, order1 + order2)
        }

        #[test]
        fn test_sub() {
            let order1 = Order {x: 1, y: 2};
            let order2 = Order {x: 2, y: 2};
            let order3 = Order {x: 3, y: 4};
            assert_eq!(order1, order3 - order2)
        }

        #[test]
        fn test_cmp() {
            let order1 = Order {x: 1, y: 2};
            let order2 = Order {x: 2, y: 2};
            let order3 = Order {x: 2, y: 3};
            assert_eq!(true, order1 < order2);
            assert_eq!(true, order2 > order1);
            assert_eq!(true, order2 >= order1);
            assert_eq!(true, order2 >= order3);
        }
    }
}