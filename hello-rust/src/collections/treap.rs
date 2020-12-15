use ptr::NonNull;
use std::{cmp::Ordering, marker::PhantomData, ops::Add, ptr};

type Link<T> = Option<NonNull<Node<T>>>;

// todo: add size
struct Node<T> {
    elem: T,
    pri: f32,
    left: Link<T>,
    right: Link<T>,
}

impl<T> Node<T> {
    fn new(elem: T) -> Self {
        Self {
            elem,
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

pub struct Treap<T> {
    root: Link<T>,
}

impl<T> Treap<T> {
    pub fn new() -> Self {
        Self { root: None }
    }

    pub fn iter(&self) -> Iter<T> {
        Iter {
            stack: vec![],
            pointer: self.root,
            _marker: PhantomData,
        }
    }

    pub fn into_iter(mut self) -> IntoIter<T> {
        let mut stk = vec![];
        self.root.take().map(|x| stk.push(x)); // must take out

        IntoIter {
            stack: stk,
            _marker: PhantomData,
        }
    }
}

impl<T: Ord> Treap<T> {
    pub fn from(v: Vec<T>) -> Self {
        if v.is_empty() {
            return Self::new();
        }

        let mut stk: Vec<NonNull<Node<T>>> = vec![];

        for e in v {
            let nd = Box::new(Node::new(e));
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

    fn split_node(mut node: Link<T>, key: &T) -> (Link<T>, Link<T>) {
        if let Some(nd) = &mut node {
            let mut n = unsafe { nd.as_mut() };
            if &n.elem < key {
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

    fn merge_node(mut node1: Link<T>, mut node2: Link<T>) -> Link<T> {
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

    pub fn get<'a, 'b>(&'a self, key: &'b T) -> Option<&'b T> {
        let mut p = self.root.as_ref();
        while let Some(r) = p {
            let b = unsafe { r.as_ref() };
            let cur = &b.elem;
            match cur.cmp(key) {
                Ordering::Less => p = b.right.as_ref(),
                Ordering::Greater => p = b.left.as_ref(),
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
        lt = Self::merge_node(lt, Node::new(elem).wrap());
        self.root = Self::merge_node(lt, ge);
        true
    }
}

impl<T> Drop for Treap<T> {
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

impl<T> IntoIterator for Treap<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.into_iter()
    }
}

impl<'a, T> IntoIterator for &'a Treap<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

pub struct Iter<'a, T> {
    pointer: Link<T>,
    stack: Vec<NonNull<Node<T>>>,
    _marker: PhantomData<&'a Node<T>>,
}

pub struct IntoIter<T> {
    stack: Vec<NonNull<Node<T>>>,
    _marker: PhantomData<Box<Node<T>>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

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
                Some(&(*p).elem)
            }
        } else {
            None
        }
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

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
            Some(b.elem)
        } else {
            None
        }
    }
}

pub trait Unit {
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
