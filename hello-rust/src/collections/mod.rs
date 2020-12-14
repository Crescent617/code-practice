#![allow(dead_code)]

use std::{
    cell::RefCell,
    cmp::Ordering,
    mem,
    ops::{Add, AddAssign, Sub},
    ptr, rc, vec,
};

use rc::Rc;

/////////////////////////////////////////////////////////////////////////
// Binary Index Tree
/////////////////////////////////////////////////////////////////////////

// trait BITreeElem = Add + Sub + Default;

pub struct BITree<T>
where
    T: AddAssign + Sub<Output = T> + Default + Copy,
{
    buf: Vec<T>,
}

impl<T> BITree<T>
where
    T: AddAssign + Sub<Output = T> + Default + Copy,
{
    pub fn new(n: usize) -> Self {
        let mut buf = Vec::with_capacity(n + 1);
        for _ in 0..n + 1 {
            buf.push(T::default());
        }
        BITree { buf }
    }

    pub fn update(&mut self, i: usize, delta: T) {
        let mut i = i + 1;
        while i < self.buf.len() {
            self.buf[i] += delta;
            i += i - (i & i - 1);
        }
    }

    pub fn len(&self) -> usize {
        self.buf.len() - 1
    }

    fn prefix(&self, i: usize) -> T {
        let mut i = i + 1;
        let mut ans = T::default();
        while i > 0 {
            ans += self.buf[i];
            i = i & i - 1;
        }
        ans
    }

    /// **left** inclusive, **right** exclusive
    pub fn query(&self, left: usize, right: usize) -> T {
        self.prefix(right.saturating_sub(1)) - self.prefix(left.saturating_sub(1))
    }
}

/////////////////////////////////////////////////////////////////////////
// Leftist Tree
/////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
struct LTreeNode<T: Ord> {
    elem: T,
    left: LTreeLink<T>,
    right: LTreeLink<T>,
    dist: usize,
}

impl<T: Ord> LTreeNode<T> {
    fn new(elem: T) -> Self {
        Self {
            elem,
            left: None,
            right: None,
            dist: 0,
        }
    }
}

type LTreeLink<T> = Option<Box<LTreeNode<T>>>;

#[derive(Debug)]
pub struct LeftistTree<T: Ord> {
    root: LTreeLink<T>,
    size: usize,
}

impl<T: Ord> LeftistTree<T> {
    pub fn new() -> Self {
        Self {
            root: None,
            size: 0,
        }
    }

    pub fn peek(&self) -> Option<&T> {
        self.root.as_ref().map(|x| &x.elem)
    }

    pub fn merge(&mut self, other: Self) {
        self.size += other.size;
        self.root = Self::merge_node(self.root.take(), other.root);
    }

    pub fn len(&self) -> usize {
        self.size
    }

    fn get_node_dist(node: &LTreeLink<T>) -> usize {
        node.as_ref().map_or(0, |x| x.dist)
    }

    fn merge_node(a: LTreeLink<T>, b: LTreeLink<T>) -> LTreeLink<T> {
        if a.is_none() || b.is_none() {
            a.or(b)
        } else if let (Some(mut a), Some(mut b)) = (a, b) {
            if a.elem < b.elem {
                mem::swap(&mut a, &mut b);
            }

            a.right = Self::merge_node(a.right, Some(b));

            let ld = Self::get_node_dist(&a.left);
            let rd = Self::get_node_dist(&a.right);

            if rd > ld {
                mem::swap(&mut a.left, &mut a.right);
            }

            a.dist = rd.min(ld) + 1;
            Some(a)
        } else {
            panic!()
        }
    }

    pub fn push(&mut self, val: T) {
        self.size += 1;
        let node = Some(Box::new(LTreeNode::new(val)));
        self.root = Self::merge_node(self.root.take(), node);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.root.take().map(|x| {
            self.root = Self::merge_node(x.left, x.right);
            self.size -= 1;
            x.elem
        })
    }
}

/////////////////////////////////////////////////////////////////////////
// Segment Tree
/////////////////////////////////////////////////////////////////////////

pub struct SegTree<T> {
    buf: Vec<Option<T>>,
    size: usize,
    offset: usize,
    _op: BinaryOp<T>,
}

type BinaryOp<T> = Box<dyn Fn(&T, &T) -> T>;

impl<T: Clone> SegTree<T> {
    pub fn new(size: usize, op: BinaryOp<T>) -> Self {
        Self::cons(vec![], size, op)
    }

    pub fn from(v: Vec<T>, op: BinaryOp<T>) -> Self {
        let n = v.len();
        Self::cons(v, n, op)
    }

    fn cons(data: Vec<T>, size: usize, op: BinaryOp<T>) -> Self {
        let bit_num = mem::size_of::<usize>() * 8;
        let offset = 1 << bit_num - size.leading_zeros() as usize;
        debug_assert!(offset >= size);

        let mut buf = Vec::with_capacity(offset * 2);
        for _ in 0..offset * 2 {
            buf.push(None);
        }

        let mut st = SegTree {
            buf,
            size,
            offset,
            _op: op,
        };

        if !data.is_empty() {
            for (i, x) in data.into_iter().enumerate() {
                st.buf[offset + i] = Some(x);
            }
            for i in (1..offset).rev() {
                st.buf[i] = st.combine(&st.buf[i * 2], &st.buf[i * 2 + 1]);
            }
        }

        st
    }

    pub fn len(&self) -> usize {
        self.size
    }

    fn combine(&self, a: &Option<T>, b: &Option<T>) -> Option<T> {
        if let (Some(x), Some(y)) = (a, b) {
            let op = &self._op;
            Some(op(x, y))
        } else if a.is_some() || b.is_some() {
            a.clone().or(b.clone())
        } else {
            None
        }
    }

    pub fn update(&mut self, idx: usize, val: T) {
        let mut i = idx + self.offset;
        self.buf[i] = Some(val);

        while i > 1 {
            i >>= 1;
            self.buf[i] = self.combine(&self.buf[i * 2], &self.buf[i * 2 + 1]);
        }
    }

    /// **left** inclusive, **right** exclusive
    pub fn query(&self, left: usize, right: usize) -> T {
        if left >= right {
            panic!()
        }

        let (mut l, mut r) = (left + self.offset, right + self.offset - 1);
        let mut ans = self.buf[l].clone();
        l += 1;

        while l <= r {
            if l % 2 == 1 {
                ans = self.combine(&ans, &self.buf[l]);
                l += 1;
            }
            if r % 2 == 0 {
                ans = self.combine(&ans, &self.buf[r]);
                r -= 1;
            }
            l >>= 1;
            r >>= 1;
        }

        ans.unwrap()
    }
}

/////////////////////////////////////////////////////////////////////////
// Treap
/////////////////////////////////////////////////////////////////////////

type TreapLink<T> = Option<Rc<RefCell<TreapNode<T>>>>;

struct TreapNode<T: Ord> {
    elem: T,
    pri: f32,
    left: TreapLink<T>,
    right: TreapLink<T>,
}

impl<T: Ord> TreapNode<T> {
    fn new(elem: T) -> Self {
        Self {
            elem,
            pri: rand::random(),
            left: None,
            right: None,
        }
    }

    fn wrap(self) -> TreapLink<T> {
        Some(Rc::new(RefCell::new(self)))
    }
}

struct Treap<T: Ord> {
    root: TreapLink<T>,
}

impl<T: Ord> Treap<T> {
    pub fn new() -> Self {
        Self { root: None }
    }

    pub fn from(v: Vec<T>) -> Self {
        if v.is_empty() {
            return Self::new();
        }

        let mut stk: Vec<Rc<RefCell<TreapNode<T>>>> = vec![];

        for e in v {
            let nd = TreapNode::new(e);
            let pri = nd.pri;
            let node = Rc::new(RefCell::new(nd));

            while let Some(x) = stk.last() {
                if x.borrow().pri < pri {
                    node.borrow_mut().left = stk.pop();
                } else {
                    break;
                }
            }

            if let Some(x) = stk.last() {
                x.borrow_mut().right = Some(Rc::clone(&node));
            }

            stk.push(node);
        }

        Self {
            root: Some(Rc::clone(&stk[0])),
        }
    }

    fn split_node(node: TreapLink<T>, key: &T) -> (TreapLink<T>, TreapLink<T>) {
        if let Some(nd) = &node {
            let mut n = nd.borrow_mut();
            if &n.elem < key {
                let (lt, ge) = Self::split_node(n.right.take(), &key);
                n.right = lt;
                (Some(Rc::clone(nd)), ge)
            } else {
                let (lt, ge) = Self::split_node(n.left.take(), &key);
                n.left = ge;
                (lt, Some(Rc::clone(nd)))
            }
        } else {
            (None, None)
        }
    }

    fn merge_node(node1: TreapLink<T>, node2: TreapLink<T>) -> TreapLink<T> {
        if let (Some(a), Some(b)) = (&node1, &node2) {
            if a.borrow().pri > b.borrow().pri {
                let mut bow_a = a.borrow_mut();
                bow_a.right = Self::merge_node(bow_a.right.take(), Some(Rc::clone(b)));
                Some(Rc::clone(a))
            } else {
                let mut bow_b = b.borrow_mut();
                bow_b.left = Self::merge_node(Some(Rc::clone(a)), bow_b.left.take());
                Some(Rc::clone(b))
            }
        } else {
            node1.or(node2)
        }
    }

    pub fn get<'a, 'b>(&'a self, key: &'b T) -> Option<&'b T> {
        let mut p = self.root.clone();
        while let Some(r) = p {
            let b = r.borrow();
            let cur = &b.elem;
            match cur.cmp(key) {
                Ordering::Less => p = b.right.clone(),
                Ordering::Greater => p = b.left.clone(),
                Ordering::Equal => return Some(key),
            }
        }
        None
    }

    pub fn insert(&mut self, elem: T) -> bool {
        if self.get(&elem).is_some() {
            return false;
        }
        let (mut lt, ge) = Self::split_node(self.root.take(), &elem);
        lt = Self::merge_node(lt, TreapNode::new(elem).wrap());
        self.root = Self::merge_node(lt, ge);
        true
    }
}

impl<T: Copy + Ord> Treap<T> {
    pub fn to_vec(self) -> Vec<T> {
        let mut res = vec![];
        Self::_to_vec(self.root, &mut res);
        res
    }

    fn _to_vec(node: TreapLink<T>, output: &mut Vec<T>) {
        if let Some(nd) = node {
            let mut b = nd.borrow_mut();
            Self::_to_vec(b.left.take(), output);
            output.push(b.elem);
            Self::_to_vec(b.right.take(), output);
        }
    }
}

trait Unit {
    fn unit() -> Self;
}

impl<T: Add<Output = T> + Ord + Unit + Clone> Treap<T> {
    pub fn remove(&mut self, elem: &T) -> bool {
        if self.get(&elem).is_none() {
            return false;
        }
        let (l, r) = Self::split_node(self.root.take(), elem);
        let (_, r) = Self::split_node(r, &elem.clone().add(T::unit()));
        self.root = Self::merge_node(l, r);
        true
    }
}

impl Unit for i32 {
    fn unit() -> Self {
        1
    }
}

/////////////////////////////////////////////////////////////////////////
// Test
/////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;
    use rand::prelude::*;
    use std::{collections::BinaryHeap, time};

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
        let mut v: Vec<_> = (0..10).collect();
        let mut t = Treap::new();
        for i in v.iter().rev() {
            t.insert(*i);
        }
        assert!(t.remove(&0));
        assert!(t.remove(&5));
        v.remove(5);
        v.remove(0);
        assert_eq!(v, t.to_vec());

        let n = 10000;
        let mut tr = Treap::new();
        let mut rng = rand::thread_rng();

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
}
