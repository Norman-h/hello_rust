use std::collections::HashMap;

#[cfg(test)]
mod test_vec_hash {
    use std::collections::HashMap;

    #[test]
    fn test_vec() {
        let mut vec = Vec::new();
        vec.push(1);
        let v = vec![1, 2, 3, 4];
        println!("v[0] = {}", v[0]);
        // println!("v[1] = {}", v.get(1));
        match v.get(1) {
            Some(x) => {
                println!("v[1] = {}", x);
            },
            None => {
                println!("don't have index 1 ");
            }
        }
        for i in &v {
            println!("{}", i);
        }
    }

    #[test]
    fn test_hashmap() {
        let mut hm = HashMap::new();
        hm.insert(1, 1.1);
        let teams_list = vec![
            ("Yello".to_string(), 100),
            ("White".to_string(), 10),
            ("Black".to_string(), 50),
        ];
        let mut teams_map: HashMap<_,_> = teams_list.into_iter().collect();
        println!("{:?}", teams_map); 
        for (k, v) in teams_map {
            println!("key {}, value {}", k, v);
        }
    }
}