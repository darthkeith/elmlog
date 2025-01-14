use serde::{Serialize, Deserialize};

/// A node in a left-child right-sibling binary tree, containing a string.
///
/// The `size` field stores the size of the node's binary subtree.
/// The binary tree represents a forest of multi-way trees, where each node can
/// have any number of children and siblings (the roots are siblings).
#[derive(Serialize, Deserialize)]
pub enum Node {
    Empty,
    Node {
        label: String,
        child: Box<Node>,
        sibling: Box<Node>,
        size: usize,
    }
}

// Represents a node in the path from the root to the node in focus, indicating
// the direction taken through the node. The structure forms part of a forest
// with a linked chain leading to the root.
enum ReturnNode {
    Child { label: String, prev: Box<ReturnNode>, sibling: Node },
    Sibling { label: String, prev: Box<ReturnNode>, child: Node },
    Empty,
}

// Zipper represention of a forest focused on a node.
struct ForestZipper {
    focus: Node,
    prev: ReturnNode,
}

/// Describes whether a node is a root, first child, or non-root right sibling.
pub enum NodeType {
    Root,
    Child,
    Sibling,
}

/// Describes the position of a node in a forest (used for display).
pub struct NodePosition {
    pub node_type: NodeType,
    pub is_last: bool,
}

// Represents a node containing references and tree position info.
struct NodeRef<'a> {
    label: &'a str,
    child: &'a Node,
    sibling: &'a Node,
    pos: NodePosition,
}

// Return a reference to the label at the given pre-order `index` in the forest.
fn find_label(index: usize, root: &mut Node) -> Option<&mut String> {
    let mut i = index;
    let mut node = root;
    while i > 0 {
        if let  Node::Node { child, sibling, .. } = node {
            if i <= child.size() {
                i -= 1;
                node = &mut **child;
            } else {
                i -= 1 + child.size();
                node = &mut **sibling;
            }
        } else {
            return None;
        }
    }
    match node {
        Node::Node { label, .. } => Some(label),
        Node::Empty => None,
    }
}

// Return a zipper focused on the node at the pre-order `index` in the forest.
// If the index is invalid, the zipper will be focused on an empty node.
fn focus_node(index: usize, root: Node) -> ForestZipper {
    let mut i = index;
    let mut focus = root;
    let mut prev = ReturnNode::Empty;
    while i > 0 {
        match focus {
            Node::Node { label, child, sibling, .. } => {
                if i <= child.size() {
                    i -= 1;
                    focus = *child;
                    prev = ReturnNode::new_child(label, prev, *sibling);
                } else {
                    i -= 1 + child.size();
                    focus = *sibling;
                    prev = ReturnNode::new_sibling(label, prev, *child);
                }
            }
            Node::Empty => break,
        }
    }
    ForestZipper { focus, prev }
}

// Concatenate two trees, making their roots siblings.
fn concat(left_root: Node, right_root: Node) -> Node {
    if let Node::Empty = right_root {
        return left_root;
    }
    let mut focus = left_root;
    let mut prev = ReturnNode::Empty;
    while let Node::Node { label, child, sibling, .. } = focus {
        focus = *sibling;
        prev = ReturnNode::new_sibling(label, prev, *child);
    }
    ForestZipper { focus: right_root, prev }
        .restore()
}

impl Node {
    fn new(label: String, child: Self, sibling: Self) -> Self {
        let size = 1 + child.size() + sibling.size();
        Self::Node {
            label,
            child: Box::new(child),
            sibling: Box::new(sibling),
            size,
        }
    }

    /// Return the number of nodes in the forest.
    pub fn size(&self) -> usize {
        match self {
            Self::Empty => 0,
            Self::Node { size, .. } => *size,
        }
    }

    /// Insert a Node with the `label` at the start of the forest.
    pub fn prepend(self, label: String) -> Self {
        Self::new(label, Self::Empty, self)
    }

    /// Swap the subtree at `index` with its next sibling.
    pub fn move_forward(self, index: usize) -> (Self, usize) {
        focus_node(index, self)
            .move_forward()
            .restore_with_index()
    }

    /// Swap the subtree at `index` with its previous sibling.
    pub fn move_backward(self, index: usize) -> (Self, usize) {
        focus_node(index, self)
            .move_backward()
            .restore_with_index()
    }

    /// Delete the node of pre-order `index` from the forest.
    pub fn delete(self, index: usize) -> Self {
        let ForestZipper { focus, prev } = focus_node(index, self);
        let new_focus = match focus {
            Self::Node { child, sibling, .. } => concat(*child, *sibling),
            Self::Empty => Self::Empty,
        };
        ForestZipper { focus: new_focus, prev, }
            .restore()
    }

    /// Return a reference to the label at the given pre-order `index`.
    pub fn label_at(&mut self, index: usize) -> &str {
        find_label(index, self)
            .expect("Invalid index")
    }

    /// Set the label at the given `index` to the `new_label`.
    pub fn set_label(&mut self, index: usize, new_label: String) {
        let label = find_label(index, self)
            .expect("Invalid index");
        *label = new_label;
    }

    // Create a corresponding NodeRef from a Node if non-empty.
    fn to_node(&self, node_type: NodeType) -> Option<NodeRef> {
        match self {
            Self::Empty => None,
            Self::Node { label, child, sibling, .. } => {
                let is_last = match **sibling {
                    Self::Empty => true,
                    Self::Node { .. } => false,
                };
                let pos = NodePosition { node_type, is_last };
                Some(NodeRef { label, child, sibling, pos })
            }
        }
    }

    /// Return an iterator over node labels in pre-order.
    pub fn iter(&self) -> PreOrderIter {
        let mut stack = Vec::new();
        if let Some(node) = self.to_node(NodeType::Root) {
            stack.push(node);
        }
        PreOrderIter { stack }
    }
}

impl ReturnNode {
    fn new_child(label: String, prev: Self, sibling: Node) -> Self {
        Self::Child {
            label,
            prev: Box::new(prev),
            sibling,
        }
    }

    fn new_sibling(label: String, prev: Self, child: Node) -> Self {
        Self::Sibling {
            label,
            prev: Box::new(prev),
            child,
        }
    }
}

impl ForestZipper {
    // Restore the zipper's corresponding forest.
    fn restore(self) -> Node {
        let Self { mut focus, mut prev } = self;
        loop {
            prev = match prev {
                ReturnNode::Child { label, prev, sibling } => {
                    focus = Node::new(label, focus, sibling);
                    *prev
                }
                ReturnNode::Sibling { label, prev, child } => {
                    focus = Node::new(label, child, focus);
                    *prev
                }
                ReturnNode::Empty => return focus,
            }
        }
    }

    // Restore the forest and return the focused node's pre-order index.
    fn restore_with_index(self) -> (Node, usize) {
        let Self { mut focus, mut prev } = self;
        let mut i = 0;
        loop {
            prev = match prev {
                ReturnNode::Child { label, prev, sibling } => {
                    i += 1;
                    focus = Node::new(label, focus, sibling);
                    *prev
                }
                ReturnNode::Sibling { label, prev, child } => {
                    i += 1 + child.size();
                    focus = Node::new(label, child, focus);
                    *prev
                }
                ReturnNode::Empty => return (focus, i),
            }
        }
    }

    // Swap the subtree in focus with its next sibling.
    fn move_forward(self) -> Self {
        let Self { focus, prev } = self;
        let focus = match focus {
            Node::Node { label, child, sibling, .. } => match *sibling {
                Node::Node {
                    label: label2,
                    child: child2,
                    sibling: sibling2,
                    ..
                } => {
                    let focus = Node::new(label, *child, *sibling2);
                    let prev = ReturnNode::new_sibling(label2, prev, *child2);
                    return Self { focus, prev };
                }
                Node::Empty => Node::new(label, *child, *sibling),
            }
            Node::Empty => focus,
        };
        Self { focus, prev }
    }

    // Swap the subtree in focus with its previous sibling.
    fn move_backward(self) -> Self {
        let Self { focus, prev } = self;
        if let ReturnNode::Sibling { label, prev, child } = prev {
            match focus {
                Node::Node {
                    label: label2,
                    child: child2,
                    sibling: sibling2,
                    ..
                } => {
                    let sibling = Node::new(label, child, *sibling2);
                    let focus = Node::new(label2, *child2, sibling);
                    Self { focus, prev: *prev }
                }
                Node::Empty => Self {
                    focus,
                    prev: ReturnNode::new_sibling(label, *prev, child),
                },
            }
        } else {
            Self { focus, prev }
        }
    }
}

/// Iterator type returning node labels/positions in pre-order.
pub struct PreOrderIter<'a> {
    stack: Vec<NodeRef<'a>>,
}

impl<'a> Iterator for PreOrderIter<'a> {
    type Item = (&'a str, NodePosition);

    fn next(&mut self) -> Option<Self::Item> {
        let NodeRef { label, child, sibling, pos } = self.stack.pop()?;
        let sibling_type = match pos.node_type {
            NodeType::Root => NodeType::Root,
            _ => NodeType::Sibling,
        };
        if let Some(node) = sibling.to_node(sibling_type) {
            self.stack.push(node);
        }
        if let Some(node) = child.to_node(NodeType::Child) {
            self.stack.push(node);
        }
        Some((label, pos))
    }
}

