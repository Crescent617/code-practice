#![allow(unused_imports)]

pub mod bitree;
pub mod leftist_tree;
pub mod segment_tree;
pub mod skip_list;
pub mod stream;
pub mod treap;

use bitree::BITree;
use leftist_tree::LeftistTree;
use segment_tree::{PstSegTree, SegTree};
use skip_list::SkipListSet;
use treap::TreapMap;

#[allow(unused_macros)]
macro_rules! timeit {
    ($block: tt) => {{
        use std::time;
        let t = time::Instant::now();
        $block;
        let d = t.elapsed();
        println!("Time usage: {:?}", d);
        d
    }};

    ($name: expr, $block: tt) => {{
        use std::time;
        let t = time::Instant::now();
        $block;
        let d = t.elapsed();
        println!("{} use: {:?}", $name, d);
        d
    }};
}

#[cfg(test)]
mod tests {

    use super::*;
    use rand::prelude::*;
    use skip_list::SkipListMap;
    use std::{
        collections::{BTreeSet, BinaryHeap},
        time,
    };
    use treap::TreapSet;

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
        let mut seg = PstSegTree::new(0, 1000, |&x, &y| x + y);
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
        let mut v: Vec<_> = (0..100).zip(0..100).collect();
        let mut t = TreapMap::new();

        for (k, v) in v.iter().rev() {
            t.insert(*k, *v);
        }

        assert!(t.remove(&0));
        assert!(t.remove(&5));

        v.remove(5);
        v.remove(0);

        for (i, x) in t.iter().enumerate() {
            // println!("{}", x);
            assert_eq!((*x.0, *x.1), v[i]);
        }
        assert_eq!(v, t.into_iter().collect::<Vec<_>>());

        let mut rng = rand::thread_rng();
        let n = 100000;
        let mut tr = TreapSet::new();

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
        {
            // test diff
            let n = 2000;
            let mut skip = SkipListMap::new();
            let mut btree = BTreeSet::new();

            for _ in 0..n {
                let r = rng.gen_range(i32::MIN, i32::MAX);
                skip.insert(r, r);
                btree.insert(r);
                assert_eq!(skip.len(), btree.len());
            }

            for _ in 0..n / 2 {
                let r = &rng.gen_range(i32::MIN, i32::MAX);
                assert_eq!(skip.remove(r), btree.remove(r));
            }

            for _ in 0..n {
                let r = &rng.gen_range(i32::MIN, i32::MAX);
                assert_eq!(skip.get(r).is_some(), btree.get(r).is_some());
            }

            for i in skip {}
        }

        let n = 100000;

        let mut skip = SkipListSet::new();
        timeit!("SkipList insert", {
            for _ in 0..n {
                skip.insert(rng.gen_range(i32::MIN, i32::MAX));
            }
        });
        timeit!("SkipList get", {
            for _ in 0..n {
                skip.get(&rng.gen_range(i32::MIN, i32::MAX));
            }
        });
        timeit!("SkipList remove", {
            for _ in 0..n {
                skip.remove(&rng.gen_range(i32::MIN, i32::MAX));
            }
        });
        println!("SkipList height: {}", skip.height());
        println!("SkipList: {}", skip);

        let mut btree = BTreeSet::new();
        timeit!("BTree insert", {
            for _ in 0..n {
                btree.insert(rng.gen_range(i32::MIN, i32::MAX));
            }
        });
        timeit!("BTree get", {
            for _ in 0..n {
                btree.get(&rng.gen_range(i32::MIN, i32::MAX));
            }
        });
        timeit!("BTree remove", {
            for _ in 0..n {
                btree.remove(&rng.gen_range(i32::MIN, i32::MAX));
            }
        });
    }
}
