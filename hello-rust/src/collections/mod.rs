#![allow(unused_imports)]

mod bitree;
mod leftist_tree;
mod segment_tree;
mod skip_list;
mod treap;

use bitree::BITree;
use leftist_tree::LeftistTree;
use segment_tree::{PstSegTree, SegTree};
use skip_list::SkipListSet;
use treap::Treap;

#[cfg(test)]
mod tests {

    use super::*;
    use rand::prelude::*;
    use skip_list::SkipListMap;
    use std::{
        collections::{BTreeSet, BinaryHeap},
        time,
    };

    #[test]
    fn test_leftist_tree() {
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
    }

    #[test]
    fn test_segtree() {
        let mut seg = PstSegTree::new(0, 1000, Box::new(|&x, &y| x + y));
        let v = vec![10, 20, 30, 50, 100];
        for x in v.into_iter().enumerate() {
            seg.insert(x.0, x.1);
        }
        assert_eq!(seg.query_nth(0, 10, 1), Some(10));
        assert_eq!(seg.query_nth(0, 10, 5), Some(100));
        assert_eq!(seg.query_nth(0, 10, 100), None);

        let v: Vec<_> = (0..100).collect();
        let mut seg = SegTree::from(v, Box::new(|x: &i32, y: &i32| *x.max(y)));

        seg.insert(50, 1);
        assert_eq!(seg.query(45, 50), Some(49));
        assert_eq!(seg.query(0, 200), None);
    }

    #[test]
    fn test_bit() {
        let mut b = BITree::new(10);
        b.update(1, 10);
        b.update(2, 1);
        b.update(3, 15);
        assert_eq!(b.query(2, 3), 1);
        b.update(6, 2);
        assert_eq!(b.query(3, 7), 17);
    }

    #[test]
    fn test_treap() {
        let mut v: Vec<_> = (0..100).collect();
        let mut t = Treap::new();

        for i in v.iter().rev() {
            t.insert(*i);
        }

        assert!(t.remove(&0));
        assert!(t.remove(&5));

        v.remove(5);
        v.remove(0);

        for (i, &x) in t.iter().enumerate() {
            // println!("{}", x);
            assert_eq!(x, v[i]);
        }
        assert_eq!(v, t.into_iter().collect::<Vec<i32>>());

        let mut rng = rand::thread_rng();
        let n = 100000;
        let mut tr = Treap::new();

        let t = time::Instant::now();
        for _ in 0..n {
            tr.insert(rng.gen_range(i32::MIN, i32::MAX));
        }
        for _ in 0..n {
            tr.remove(&rng.gen_range(i32::MIN, i32::MAX));
        }
        println!("Treap use: {:?}", t.elapsed());

        let mut tr = std::collections::BTreeSet::new();

        let t = time::Instant::now();
        for _ in (0..n).rev() {
            tr.insert(rng.gen_range(i32::MIN, i32::MAX));
        }
        for _ in 0..n {
            tr.remove(&rng.gen_range(i32::MIN, i32::MAX));
        }

        println!("BTree use: {:?}", t.elapsed());
    }

    #[test]
    fn test_skip_list() {
        let mut rng = rand::thread_rng();
        let mut s = SkipListMap::new();
        for i in 0..100 {
            s.insert(i, i);
        }
        println!("{}", s);
        assert!(s.get(&99).is_some());
        assert!(s.get(&100).is_none());
        s.insert(10, 10);
        assert_eq!(s.len(), 100);

        {
            let n = 1000;
            let mut s = SkipListSet::new();
            let mut b = BTreeSet::new();

            for _ in 0..n {
                let r = rng.gen_range(i32::MIN, i32::MAX);
                s.insert(r);
                b.insert(r);
            }
            assert_eq!(s.len(), b.len());
            for _ in 0..n {
                let r = &rng.gen_range(i32::MIN, i32::MAX);
                assert_eq!(s.get(r).is_some(), b.get(r).is_some());
            }
        }

        let n = 100000;

        {
            let mut s = SkipListSet::new();
            let t = time::Instant::now();
            for _ in 0..n {
                s.insert(rng.gen_range(i32::MIN, i32::MAX));
            }
            let t1 = t.elapsed();
            println!("SkipList insert use: {:?}", t1);

            for _ in 0..n {
                s.get(&rng.gen_range(i32::MIN, i32::MAX));
            }
            println!("SkipList find use: {:?}", t.elapsed() - t1);
        }

        {
            let mut b = BTreeSet::new();
            let t = time::Instant::now();
            for _ in 0..n {
                b.insert(rng.gen_range(i32::MIN, i32::MAX));
            }
            let t1 = t.elapsed();
            println!("BTree insert use: {:?}", t1);

            for _ in 0..n {
                b.get(&rng.gen_range(i32::MIN, i32::MAX));
            }
            println!("BTree find use: {:?}", t.elapsed() - t1);
        }
    }
}
