mod collections;

use collections::{LeftistTree, SegTree};
use rand::prelude::*;
use std::{
    collections::{BTreeMap, BinaryHeap, LinkedList},
    mem, time,
};

fn main() {
    // let a = Box::new(1);
    // let b = Box::leak(a);
    // let c = NonNull::from(b);
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

    println!(">>> test begin");

    let mut h1 = LeftistTree::new();
    let mut h2 = BinaryHeap::new();

    for _ in 0..n {
        let r = rng.gen_range(i32::MIN, i32::MAX);
        h1.push(r);
        h2.push(r);
    }
    for _ in 0..n {
        assert_eq!(h1.pop(), h2.pop());
        assert_eq!(h1.len(), h2.len());
    }

    let v: Vec<_> = (0..100).collect();
    let mut seg = SegTree::from(v, Box::new(|&x, &y| x + y));
    assert_eq!(seg.query(0, 11), 55);
    assert_eq!(seg.query(0, 100), (0..100).sum());

    seg.update(0, 10);
    assert_eq!(seg.query(0, 11), 65);

    let v: Vec<_> = (0..100).collect();
    let mut seg = SegTree::from(v, Box::new(|x: &i32, y: &i32| *x.max(y)));

    seg.update(50, 1);
    assert_eq!(seg.query(45, 50), 49);
    assert_eq!(seg.query(45, 200), 99);

    println!(">>> test success");
}
