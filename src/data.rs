/// A node in a left-child right-sibling binary tree
struct Node {
    label: String,
    size: u32,
    child: Option<Box<Node>>,
    sibling: Option<Box<Node>>,
}

impl Node {
    fn new(label: String) -> Self {
        Node {
            label: label,
            size: 0,
            child: None,
            sibling: None,
        }
    }
}
