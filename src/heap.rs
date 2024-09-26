pub struct Heap {
    root: Option<Box<Node>>,
}

/// A node in a left-child right-sibling binary tree
struct Node {
    label: String,
    child: Heap,
    sibling: Heap,
    size: usize,
}

/// Represents the number of roots in a heap (none, one, or multiple)
pub enum HeapStatus {
    Empty,
    SingleRoot,
    MultiRoot,
}

pub fn empty() -> Heap {
    Heap { root: None }
}

pub fn heap_size(heap: &Heap) -> usize {
    match heap.root {
        Some(ref node) => node.size,
        None => 0,
    }
}

fn new_node(label: String, child: Heap, sibling: Heap) -> Box<Node> {
    let size = 1 + heap_size(&child) + heap_size(&sibling);
    let node = Node {
        label,
        child,
        sibling,
        size,
    };
    Box::new(node)
}

pub fn prepend(heap: Heap, label: String) -> Heap {
    let root = Some(new_node(label, empty(), heap));
    Heap { root }
}

pub struct PreOrderIter<'a> {
    stack: Vec<&'a Node>,
}

impl<'a> Iterator for PreOrderIter<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(node) = self.stack.pop() {
            if let Some(sibling) = &node.sibling.root {
                self.stack.push(sibling);
            }
            if let Some(child) = &node.child.root {
                self.stack.push(child);
            }
            return Some(&node.label);
        }
        None
    }
}

pub fn iter(heap: &Heap) -> PreOrderIter {
    let mut stack = Vec::new();
    if let Some(root) = &heap.root {
        stack.push(root.as_ref());
    }
    PreOrderIter { stack }
}

pub fn status(heap: &Heap) -> HeapStatus {
    match &heap.root {
        None => HeapStatus::Empty,
        Some(node) => match &node.sibling.root {
            None => HeapStatus::SingleRoot,
            Some(_) => HeapStatus::MultiRoot,
        }
    }
}

