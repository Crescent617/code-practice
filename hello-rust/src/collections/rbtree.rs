use ptr::NonNull;
use std::{cmp::Ordering, marker::PhantomData, ops::Add, ptr};

type Edge<K, V> = Option<NonNull<Node<K, V>>>;

enum Color {
    Red,
    Black,
}

use Color::*;

// todo: add size
struct Node<K, V> {
    key: K,
    val: V,
    parent: Edge<K, V>,
    left: Edge<K, V>,
    right: Edge<K, V>,
    color: Color,
}

impl<K, V> Node<K, V> {
    fn new(key: K, val: V, color: Color) -> Self {
        Self {
            key,
            val,
            parent: None,
            left: None,
            right: None,
            color,
        }
    }

    fn wrap(self) -> Option<NonNull<Self>> {
        let b = Box::new(self);
        Some(NonNull::from(Box::leak(b)))
    }

    fn flip_color(&mut self) -> Color {
        let new_color = match self.color {
            Red => Black,
            Black => Red,
        };
        std::mem::replace(&mut self.color, new_color)
    }
}
