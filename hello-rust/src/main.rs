mod collections;

use collections::LeftistTree;
use rand::prelude::*;
use std::{collections::BinaryHeap, time};

fn main() {
    let n = 10000;
    let mut heap = LeftistTree::new();
    let mut rng = rand::thread_rng();

    let t = time::Instant::now();
    for _ in 0..n {
        heap.push(rng.gen_range(i32::MIN, i32::MAX));
    }
    for _ in 0..n {
        heap.pop();
    }
    println!("LTree use: {:?}", t.elapsed());

    let mut heap = BinaryHeap::new();

    let t = time::Instant::now();
    for _ in (0..n).rev() {
        heap.push(rng.gen_range(i32::MIN, i32::MAX));
    }
    for _ in 0..n {
        heap.pop();
    }
    println!("BHeap use: {:?}", t.elapsed());

    println!("test begin");
    let mut h1 = LeftistTree::new();
    let mut h2 = BinaryHeap::new();

    for _ in 0..n {
        let r = rng.gen_range(i32::MIN, i32::MAX);
        h1.push(r);
        h2.push(r);
    }
    for _ in 0..n {
        assert_eq!(h1.pop(), h2.pop());
    }
    println!("success");
}
