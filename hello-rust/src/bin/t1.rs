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
use std::time;


fn main() {
    let t = time::Instant::now();
    let n = 100;
    for _ in 0..n {
        rand::random::<u8>() > 1;
    }
    println!("Int use: {:?}", t.elapsed());
    let t = time::Instant::now();
    for _ in 0..n {
        rand::random::<f32>() < 1.0;
    }
    println!("Float use: {:?}", t.elapsed());
}
