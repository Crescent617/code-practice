use std::{
    fmt, mem,
    ops::{AddAssign, Sub},
};


/////////////////////////////////////////////////////////////////////////
// Binary Index Tree
/////////////////////////////////////////////////////////////////////////

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

/////////////////////////////////////////////////////////////////////////
// Leftist Tree
/////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
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
        self.root.as_ref().map(|x| &x.val)
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
            if a.val < b.val {
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
            x.val
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
