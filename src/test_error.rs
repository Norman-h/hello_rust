use std::fs::File;
use std::io;


pub mod my_test_error {
    use std::{fs::File, error::Error, fmt::Debug, io::{self, Read}};

    pub fn test_result() {
        let f = File::create("test.txt");
        match f {
            Ok(file) => {
                
            },
            Error => {
                println!("can not open file!");
            }
        };
    }

    pub fn read_from_file() -> Result<String, io::Error> {
        let mut s = String::new();
        File::open("test.txt")?.read_to_string(&mut s)?;
        Ok(s)
    }
}

#[cfg(test)]
mod tests {
    use super::my_test_error;

    #[test]
    fn test_summary() {
        let summ = my_test_error::read_from_file();
        println!("{:?}", summ);
    }
}