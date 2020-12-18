use std::mem;

#[derive(Debug)]
struct Node<T: Ord> {
    elem: T,
    left: Link<T>,
    right: Link<T>,
    dist: usize,
}

impl<T: Ord> Node<T> {
    fn new(elem: T) -> Self {
        Self {
            elem,
            left: None,
            right: None,
            dist: 0,
        }
    }
}

type Link<T> = Option<Box<Node<T>>>;

#[derive(Debug)]
pub struct LeftistTree<T: Ord> {
    root: Link<T>,
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

    fn get_node_dist(node: &Link<T>) -> usize {
        node.as_ref().map_or(0, |x| x.dist)
    }

    fn merge_node(a: Link<T>, b: Link<T>) -> Link<T> {
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
        let node = Some(Box::new(Node::new(val)));
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
