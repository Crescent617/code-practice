use std::{marker::PhantomData, ptr::NonNull};

pub trait BinaryTree {
    type Node;
}

pub trait BinaryTreeNode {
    type Key;
    type Value;
    fn left(&self) -> &Option<NonNull<Self>>;
    fn right(&self) -> &Option<NonNull<Self>>;
    fn mut_left(&mut self) -> &mut Option<NonNull<Self>>;
    fn mut_right(&mut self) -> &mut Option<NonNull<Self>>;
    fn kv(&self) -> (&Self::Key, &Self::Value);
    fn move_kv(self) -> (Self::Key, Self::Value);
}

type Edge<Node> = Option<NonNull<Node>>;

pub struct Iter<'a, N: BinaryTreeNode> {
    pointer: Edge<N>,
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
            unsafe {
                self.pointer = *x.as_ref().left();
            }
        }

        if let Some(last) = self.stack.pop() {
            let p = last.as_ptr();
            unsafe {
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
            unsafe {
                if let Some(l) = x.as_mut().mut_left().take() {
                    self.stack.push(l);
                } else {
                    break;
                }
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
