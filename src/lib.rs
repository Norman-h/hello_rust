pub mod test_closure;
pub mod test_enum;
pub mod test_error;
pub mod test_lifetime;
pub mod test_string;
pub mod test_struct;
pub mod test_vec_and_hash;
pub mod test_iter;
pub mod test_time;
pub mod test_tuple;
pub mod test_array;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
