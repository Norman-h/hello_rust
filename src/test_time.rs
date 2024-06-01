use chrono::Utc;
use chrono::Local;

pub mod my_test_timer {

    #[cfg(test)]
    mod test{
        
        #[test]
        fn test_time() {
            let this_time = chrono::Utc::now();
            let this_local = chrono::Local::now();
            let test = this_local.to_rfc3339();
            println!("UTC now is {}!", this_time);
            println!("Local now is {}!", this_local);
            println!("Local string now is {}!", test);

        }
    }
}