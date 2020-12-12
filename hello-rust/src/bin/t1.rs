use std::{mem, vec};

use mem::size_of;

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

type BinaryOp<T> = Box<dyn Fn(T, T) -> T>;

pub struct SegTree<T> {
    buf: Vec<Box<Option<T>>>,
    size: usize,
    op: BinaryOp<T>,
}

impl<T: Clone> SegTree<T> {
    pub fn new(size: usize, op: BinaryOp<T>) -> Self {
        Self::cons(&vec![], size, op)
    }

    fn cons(data: &Vec<T>, size: usize, op: BinaryOp<T>) -> Self {
        use std::mem::size_of;

        let cap = 1 << (size_of::<usize>() * 8 - size.leading_zeros() as usize);
        debug_assert!(cap >= size);

        let mut buf: Vec<Box<Option<T>>> = vec![Box::new(None); cap];
        for i in 0..data.len() {
            buf[i].replace(data[i].clone());
        }

        SegTree { buf, size, op }
    }

    pub fn from(v: Vec<T>, op: BinaryOp<T>) -> Self {
        Self::cons(&v, v.len(), op)
    }
}

fn main() {}
