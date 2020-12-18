use ptr::NonNull;
use std::{cmp::Ordering, marker::PhantomData, ops::Add, ptr};

type Edge<K, V> = Option<NonNull<Node<K, V>>>;

// todo: add size
struct Node<K, V> {
    key: K,
    val: V,
    pri: f32,
    left: Edge<K, V>,
    right: Edge<K, V>,
}

impl<K, V> Node<K, V> {
    fn new(key: K, val: V) -> Self {
        Self {
            key,
            val,
            pri: rand::random(),
            left: None,
            right: None,
        }
    }

    fn wrap(self) -> Option<NonNull<Self>> {
        let b = Box::new(self);
        Some(NonNull::from(Box::leak(b)))
    }
}

pub struct TreapSet<T> {
    map: TreapMap<T, ()>,
}

impl<T: Ord> TreapSet<T> {
    pub fn new() -> Self {
        Self {
            map: TreapMap::new(),
        }
    }

    pub fn from(v: Vec<T>) -> Self {
        Self {
            map: TreapMap::from(v.into_iter().map(|x| (x, ())).collect()),
        }
    }

    pub fn insert(&mut self, elem: T) {
        self.map.insert(elem, ());
    }

    pub fn get(&self, elem: &T) -> Option<&()> {
        self.map.get(elem)
    }
}

impl<T: Add<Output = T> + Ord + Unit + Clone> TreapSet<T> {
    pub fn remove(&mut self, elem: &T) -> bool {
        self.map.remove(elem)
    }
}

pub struct TreapMap<K, V> {
    root: Edge<K, V>,
}

impl<K, V> TreapMap<K, V> {
    pub fn new() -> Self {
        Self { root: None }
    }

    pub fn iter(&self) -> Iter<K, V> {
        Iter {
            stack: vec![],
            pointer: self.root,
            _marker: PhantomData,
        }
    }

    pub fn into_iter(mut self) -> IntoIter<K, V> {
        let mut stk = vec![];
        self.root.take().map(|x| stk.push(x)); // must take out

        IntoIter {
            stack: stk,
            _marker: PhantomData,
        }
    }
}

impl<K: Ord, V> TreapMap<K, V> {
    pub fn from(v: Vec<(K, V)>) -> Self {
        if v.is_empty() {
            return Self::new();
        }

        let mut stk: Vec<NonNull<Node<K, V>>> = vec![];

        for (k, v) in v {
            let nd = Box::new(Node::new(k, v));
            let pri = nd.pri;
            let mut node = NonNull::from(Box::leak(nd));

            unsafe {
                while let Some(x) = stk.last() {
                    if x.as_ref().pri < pri {
                        node.as_mut().left = stk.pop();
                    } else {
                        break;
                    }
                }
                if let Some(x) = stk.last_mut() {
                    x.as_mut().right = Some(node);
                }
            }
            stk.push(node);
        }

        Self {
            root: stk.first().map(|x| *x),
        }
    }

    fn split_node(mut node: Edge<K, V>, key: &K) -> (Edge<K, V>, Edge<K, V>) {
        if let Some(nd) = &mut node {
            let mut n = unsafe { nd.as_mut() };
            if &n.key < key {
                let (lt, ge) = Self::split_node(n.right.take(), &key);
                n.right = lt;
                (node, ge)
            } else {
                let (lt, ge) = Self::split_node(n.left.take(), &key);
                n.left = ge;
                (lt, node)
            }
        } else {
            (None, None)
        }
    }

    fn merge_node(mut node1: Edge<K, V>, mut node2: Edge<K, V>) -> Edge<K, V> {
        if let (Some(a), Some(b)) = (&mut node1, &mut node2) {
            unsafe {
                if a.as_ref().pri > b.as_ref().pri {
                    let mut bow_a = a.as_mut();
                    bow_a.right = Self::merge_node(bow_a.right.take(), node2);
                    node1
                } else {
                    let mut bow_b = b.as_mut();
                    bow_b.left = Self::merge_node(node1, bow_b.left.take());
                    node2
                }
            }
        } else {
            node1.or(node2)
        }
    }

    pub fn get<'a, 'b>(&'a self, key: &'b K) -> Option<&'a V> {
        let mut p = self.root.as_ref();
        while let Some(r) = p {
            let b = unsafe { r.as_ref() };
            let cur = &b.key;
            match cur.cmp(key) {
                Ordering::Less => p = b.right.as_ref(),
                Ordering::Greater => p = b.left.as_ref(),
                Ordering::Equal => return Some(&b.val),
            }
        }
        None
    }

    pub fn insert(&mut self, key: K, val: V) -> bool {
        if self.get(&key).is_some() {
            return false;
        }
        let (mut lt, ge) = Self::split_node(self.root.take(), &key);
        lt = Self::merge_node(lt, Node::new(key, val).wrap());
        self.root = Self::merge_node(lt, ge);
        true
    }
}

impl<K, V> Drop for TreapMap<K, V> {
    fn drop(&mut self) {
        let mut s = vec![];
        self.root.map(|x| s.push(x));

        while let Some(x) = s.pop() {
            let node = unsafe { Box::from_raw(x.as_ptr()) };
            node.left.map(|u| s.push(u));
            node.right.map(|u| s.push(u));
        }
    }
}

impl<K, V> IntoIterator for TreapMap<K, V> {
    type Item = (K, V);
    type IntoIter = IntoIter<K, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.into_iter()
    }
}

impl<'a, K, V> IntoIterator for &'a TreapMap<K, V> {
    type Item = (&'a K, &'a V);
    type IntoIter = Iter<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

pub struct Iter<'a, K, V> {
    pointer: Edge<K, V>,
    stack: Vec<NonNull<Node<K, V>>>,
    _marker: PhantomData<&'a Node<K, V>>,
}

pub struct IntoIter<K, V> {
    stack: Vec<NonNull<Node<K, V>>>,
    _marker: PhantomData<Box<Node<K, V>>>,
}

impl<'a, K, V> Iterator for Iter<'a, K, V> {
    type Item = (&'a K, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(x) = self.pointer {
            self.stack.push(x);
            unsafe {
                self.pointer = x.as_ref().left;
            }
        }

        if let Some(last) = self.stack.pop() {
            let p = last.as_ptr();
            unsafe {
                self.pointer = last.as_ref().right;
                Some((&(*p).key, &(*p).val))
            }
        } else {
            None
        }
    }
}

impl<K, V> Iterator for IntoIter<K, V> {
    type Item = (K, V);

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(x) = self.stack.last_mut() {
            unsafe {
                if let Some(l) = x.as_mut().left.take() {
                    self.stack.push(l);
                } else {
                    break;
                }
            }
        }
        if let Some(last) = self.stack.pop() {
            let mut b = unsafe { Box::from_raw(last.as_ptr()) };
            b.right.take().map(|x| self.stack.push(x));
            Some((b.key, b.val))
        } else {
            None
        }
    }
}

pub trait Unit {
    fn unit() -> Self;
}

impl<K: Add<Output = K> + Ord + Unit + Clone, V> TreapMap<K, V> {
    pub fn remove(&mut self, key: &K) -> bool {
        if self.get(&key).is_none() {
            return false;
        }
        let (l, r) = Self::split_node(self.root.take(), key);
        let (_, r) = Self::split_node(r, &key.clone().add(K::unit()));
        self.root = Self::merge_node(l, r);
        true
    }
}

impl Unit for i32 {
    fn unit() -> Self {
        1
    }
}
