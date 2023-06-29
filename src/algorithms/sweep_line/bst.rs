use std::rc::Rc;

use crate::geometry::segment::Segment;

struct Node<T> {
    left_rightmost: Option<T>,
    leaf: Option<T>,
    parent: Option<Rc<Node<T>>>,
    left: Option<Rc<Node<T>>>,
    right: Option<Rc<Node<T>>>,
}

struct SweepLineBST<T> {
    root: Option<Node<T>>,
}

impl<T> SweepLineBST<T> {
    pub fn new() -> Self {
        Self { root: None }
    }
}
