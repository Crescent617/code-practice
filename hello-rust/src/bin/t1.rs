use std::{
    collections::{BTreeMap, BTreeSet, LinkedList},
    ptr::NonNull,
};

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

use hello_rust::collections::leftist_tree::*;
use hello_rust::collections::rbtree::*;

fn main() {
    let mut heap = PstLeftistTree::new();
    for i in (0..10).rev() {
        heap.push(i);
    }
    dbg!(heap);
}
