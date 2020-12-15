use std::collections::{BTreeSet, LinkedList};

#[allow(unused_macros)]
macro_rules! input {
    () => {{
        let mut line = String::new();
        std::io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");
        line.trim().to_owned()
    }};
}

#[allow(unused_macros)]
macro_rules! input_num {
    () => {{
        input!().parse().unwrap()
    }};
}

#[allow(unused_macros)]
macro_rules! input_nums {
    () => {{
        input!()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect()
    }};
}

#[allow(unused_macros)]
macro_rules! map {
    ($($key: expr => $val: expr), *) => {{
        let mut dict = std::collections::HashMap::new();
        $( dict.insert($key, $val); )*
        dict
    }};
}

#[derive(Debug)]
struct Solution;

use std::rc::Rc;

fn main() {
    let v = vec![0; 2];
    let x = Rc::new(v);
    let y = x.clone();

    unsafe {
        (*(Rc::as_ptr(&x) as *mut Vec<i32>))[0] = 1000;
    }
    dbg!(x);
    // LinkedList;
}
