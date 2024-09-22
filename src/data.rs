/// A node in a left-child right-sibling binary tree
struct Node {
    label: String,
    size: usize,
    child: Option<Box<Node>>,
    sibling: Option<Box<Node>>,
}

fn size(maybe_node: &Option<Box<Node>>) -> usize {
    match maybe_node {
            Some(ref node) => node.size,
            None => 0,
    }
}

fn new(label: String, child: Option<Box<Node>>, sibling: Option<Box<Node>>) -> Node {
    Node {
        label: label,
        size: 1 + size(&child) + size(&sibling),
        child: child,
        sibling: sibling,
    }
}

