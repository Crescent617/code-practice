use std::{marker::PhantomData, ptr::NonNull};

pub trait BinaryTreeIter {
    type Node: BinaryTreeNode;

    fn root(&self) -> Option<NonNull<Self::Node>>;
    fn mut_root(&mut self) -> &mut Option<NonNull<Self::Node>>;
    // fn insert(&mut self, key: Self::Key, val: Self::Value);
    // fn remove(&mut self, key: &Self::Key) -> bool;
    fn iter(&self) -> Iter<Self::Node> {
        Iter {
            pointer: self.root(),
            stack: vec![],
            _marker: PhantomData,
        }
    }

    fn into_iter(&mut self) -> IntoIter<Self::Node> {
        let mut stack = vec![];
        self.mut_root().take().map(|x| stack.push(x));
        IntoIter {
            stack,
            _marker: PhantomData,
        }
    }
}

pub trait BinaryTreeNode {
    type Key;
    type Value;

    fn kv(&self) -> (&Self::Key, &Self::Value);
    fn move_kv(self) -> (Self::Key, Self::Value);
    fn left(&self) -> &Option<NonNull<Self>>;
    fn right(&self) -> &Option<NonNull<Self>>;
    fn mut_left(&mut self) -> &mut Option<NonNull<Self>>;
    fn mut_right(&mut self) -> &mut Option<NonNull<Self>>;
}

pub struct Iter<'a, N: BinaryTreeNode> {
    pointer: Option<NonNull<N>>,
    stack: Vec<NonNull<N>>,
    _marker: PhantomData<&'a N>,
}

pub struct IntoIter<N: BinaryTreeNode> {
    stack: Vec<NonNull<N>>,
    _marker: PhantomData<Box<N>>,
}

impl<'a, N: BinaryTreeNode> Iterator for Iter<'a, N> {
    type Item = (&'a N::Key, &'a N::Value);

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(x) = self.pointer {
            self.stack.push(x);
            self.pointer = unsafe { *x.as_ref().left() };
        }

        if let Some(last) = self.stack.pop() {
            unsafe {
                let p = last.as_ptr();
                self.pointer = *last.as_ref().right();
                Some((*p).kv())
            }
        } else {
            None
        }
    }
}

impl<N: BinaryTreeNode> Iterator for IntoIter<N> {
    type Item = (N::Key, N::Value);

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(x) = self.stack.last_mut() {
            if let Some(l) = unsafe { x.as_mut().mut_left().take() } {
                self.stack.push(l);
            } else {
                break;
            }
        }
        if let Some(last) = self.stack.pop() {
            let mut b = unsafe { Box::from_raw(last.as_ptr()) };
            b.mut_right().take().map(|x| self.stack.push(x));
            Some(b.move_kv())
        } else {
            None
        }
    }
}
