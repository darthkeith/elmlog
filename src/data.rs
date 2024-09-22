type Heap = Option<Box<Node>>;

/// A node in a left-child right-sibling binary tree
struct Node {
    label: String,
    size: usize,
    child: Heap,
    sibling: Heap,
}

fn size(heap: &Heap) -> usize {
    match heap {
        Some(ref node) => node.size,
        None => 0,
    }
}

fn new(label: String, child: Heap, sibling: Heap) -> Node {
    Node {
        label: label,
        size: 1 + size(&child) + size(&sibling),
        child: child,
        sibling: sibling,
    }
}

