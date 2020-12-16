#![allow(dead_code)]

use std::fmt::Formatter;
use std::ptr::NonNull;
use std::{cmp::Ordering, fmt, fmt::Display};

const P: f32 = 0.5;
const MAX_LEVEL: usize = 32;

type Link<K, V> = Option<NonNull<Node<K, V>>>;

pub struct SkipListSet<T> {
    map: SkipListMap<T, ()>,
}

impl<T: Ord> SkipListSet<T> {
    pub fn new() -> Self {
        Self {
            map: SkipListMap::new(),
        }
    }

    pub fn insert(&mut self, key: T) -> bool {
        self.map.insert(key, ())
    }

    pub fn get(&self, key: &T) -> Option<&()> {
        self.map.get(key).map(|x| &())
    }

    pub fn len(&self) -> usize {
        self.map.len()
    }

    pub fn max_level(&self) -> usize {
        self.map.level
    }
}

pub struct SkipListMap<K, V> {
    heads: Vec<Link<K, V>>,
    length: usize,
    level: usize,
}

struct Node<K, V> {
    nexts: Vec<Link<K, V>>,
    key: K,
    val: V,
}

impl<K, V> Node<K, V> {
    fn new(key: K, val: V) -> Self {
        Self {
            nexts: vec![None; Self::random_level()],
            key,
            val,
        }
    }

    fn random_level() -> usize {
        // let num = rand::random::<u32>();
        // (1 + num.trailing_ones() as usize).min(MAX_LEVEL)
        let mut l = 1;
        while rand::random::<f32>() < P && l < MAX_LEVEL {
            l += 1;
        }
        l
    }
}

impl<K: Ord, V> SkipListMap<K, V> {
    pub fn new() -> Self {
        Self {
            heads: vec![None; MAX_LEVEL],
            length: 0,
            level: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.length
    }

    pub fn height(&self) -> usize {
        self.level
    }

    pub fn insert(&mut self, key: K, val: V) -> bool {
        if let Some(mut nd) = Self::get_node(&self.heads, self.level, &key) {
            unsafe {
                nd.as_mut().val = val;
            }
            false
        } else {
            self.length += 1;
            let node = Box::new(Node::new(key, val));
            self.level = self.level.max(node.nexts.len());
            unsafe {
                Self::insert_node(
                    &mut self.heads,
                    self.level,
                    NonNull::new_unchecked(Box::into_raw(node)),
                );
            }
            true
        }
    }

    fn insert_node(
        mut next_nodes: &mut [Link<K, V>],
        mut level: usize,
        mut node: NonNull<Node<K, V>>,
    ) {
        let node_level = unsafe { node.as_ref().nexts.len() };

        while level > 0 {
            if let Some(p) = next_nodes[level - 1] {
                unsafe {
                    match node.as_ref().key.cmp(&p.as_ref().key) {
                        Ordering::Equal => panic!(),
                        Ordering::Less => {
                            if level <= node_level {
                                node.as_mut().nexts[level - 1] = Some(p);
                                next_nodes[level - 1] = Some(node);
                            }
                            level -= 1;
                        }
                        Ordering::Greater => next_nodes = &mut (*p.as_ptr()).nexts,
                    }
                }
            } else {
                if level <= node_level {
                    next_nodes[level - 1] = Some(node);
                }
                level -= 1;
            }
        }
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        Self::get_node(&self.heads, self.level, key).map(|x| unsafe { &(*x.as_ptr()).val })
    }

    fn get_node(
        mut next_nodes: &[Link<K, V>],
        mut level: usize,
        key: &K,
    ) -> Option<NonNull<Node<K, V>>> {
        while level > 0 {
            if let Some(p) = next_nodes[level - 1] {
                match key.cmp(unsafe { &p.as_ref().key }) {
                    Ordering::Equal => return Some(p),
                    Ordering::Less => level -= 1,
                    Ordering::Greater => next_nodes = unsafe { &mut (*p.as_ptr()).nexts },
                }
            } else {
                level -= 1;
            }
        }
        None
    }

    pub fn remove(&mut self, key: &K) -> bool {
        todo!();
        if self.get(key).is_some() {
            false
        } else {
            true
        }
    }
}

impl<K, V> Drop for SkipListMap<K, V> {
    fn drop(&mut self) {
        let mut node = self.heads[0];
        while let Some(p) = node {
            unsafe {
                let b = Box::from_raw(p.as_ptr());
                node = b.nexts[0];
            }
        }
    }
}

impl<K: Display, V: Display> Display for SkipListMap<K, V> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for i in (0..self.level).rev() {
            let mut node = self.heads[i];
            while let Some(p) = node {
                unsafe {
                    // write!(f, "({} => {}) -> ", p.as_ref().key, p.as_ref().val)?;
                    write!(f, "{} -> ", p.as_ref().key)?;
                    node = p.as_ref().nexts[i];
                }
            }
            write!(f, "end\n")?;
        }
        write!(f, "end")
    }
}
