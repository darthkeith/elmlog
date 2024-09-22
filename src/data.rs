type Heap = Option<Box<Node>>;

/// A node in a left-child right-sibling binary tree
struct Node {
    label: String,
    child: Heap,
    sibling: Heap,
    size: usize,
}

fn heap_size(heap: &Heap) -> usize {
    match heap {
        Some(ref node) => node.size,
        None => 0,
    }
}

fn new(label: String, child: Heap, sibling: Heap) -> Box<Node> {
    let size = 1 + heap_size(&child) + heap_size(&sibling);
    let node = Node {
        label,
        child,
        sibling,
        size,
    };
    Box::new(node)
}

fn prepend(root: Heap, label: String) -> Heap {
    Some(new(label, None, root))
}
