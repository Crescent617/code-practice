use std::{
    mem::swap,
    ops::{AddAssign, Sub},
};

// trait BITreeElem = Add + Sub + Default;

/// ```
/// let mut b = hello_rust::collections::BITree::new(10);
/// b.update(1, 10);
/// b.update(2, 1);
/// b.update(3, 15);
/// assert_eq!(b.query(2, 3), 1);
/// b.update(6, 2);
/// assert_eq!(b.query(3, 7), 17);
/// ```
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

#[derive(Debug, Clone)]
struct LTreeNode<T: Ord> {
    val: T,
    left: LTreeLink<T>,
    right: LTreeLink<T>,
    dist: usize,
}

impl<T: Ord> LTreeNode<T> {
    fn new(val: T) -> Self {
        Self {
            val,
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
}

impl<T: Ord> LeftistTree<T> {
    pub fn new() -> Self {
        Self { root: None }
    }

    pub fn peek(&self) -> Option<&T> {
        self.root.as_ref().map(|x| &x.val)
    }

    pub fn merge(&mut self, mut other: Self) {
        if let Some(r) = Self::merge_node(self.root.take(), other.root.take()) {
            self.root.replace(r);
        }
    }

    fn merge_node(a: LTreeLink<T>, b: LTreeLink<T>) -> LTreeLink<T> {
        if a.is_none() || b.is_none() {
            a.or(b)
        } else if let (Some(mut a), Some(mut b)) = (a, b) {
            if a.val < b.val {
                swap(&mut a, &mut b);
            }

            let mut d = 0;

            if let Some(r) = Self::merge_node(a.right.take(), Some(b)) {
                let rd = r.dist;
                let ld = if let Some(ref l) = a.left { l.dist } else { 0 };

                a.right.replace(r);

                if rd > ld {
                    swap(&mut a.left, &mut a.right);
                }
                d = rd.min(ld) + 1;
            }

            a.dist = d;
            Some(a)
        } else {
            panic!()
        }
    }

    pub fn push(&mut self, val: T) {
        let node = Some(Box::new(LTreeNode::new(val)));
        if let Some(r) = Self::merge_node(self.root.take(), node) {
            self.root.replace(r);
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        self.root.take().map(|mut x| {
            Self::merge_node(x.left.take(), x.right.take()).map(|x| self.root.replace(x));
            x.val
        })
    }
}
