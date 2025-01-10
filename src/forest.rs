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

struct Tree {
    label: String,
    child: Node,
}

// Represents the direction taken through a node in a path from the root.
enum Direction {
    Child { label: String, sibling: Node },
    Sibling { label: String, child: Node },
}

// Zipper represention of a forest focused on a node.
struct ForestZipper {
    focus: Node,
    path: Vec<Direction>,
}

// Represents an attempt to separate the first two trees in a forest.
enum TwoTrees {
    Success { tree_1: Tree, tree_2: Tree, rest: Node},
    Fail(Node),
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

/// Represents the number of roots in a forest (zero, one, or multiple).
///
/// If multiple, include references to the first two labels.
pub enum ForestStatus<'a> {
    Empty,
    SingleRoot,
    MultiRoot(&'a str, &'a str),
}

// Return a reference to the label at the given pre-order `index` in the forest.
fn find_label(root: &mut Node, index: usize) -> Option<&mut String> {
    let mut node = root;
    let mut i = index;
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
fn find_node(root: Node, index: usize) -> ForestZipper {
    let mut node = root;
    let mut i = index;
    let mut path = Vec::new();
    while i > 0 {
        if let Node::Node { label, child, sibling, .. } = node {
            if i <= child.size() {
                i -= 1;
                path.push(Direction::Child { label, sibling: *sibling });
                node = *child;
            } else {
                i -= 1 + child.size();
                path.push(Direction::Sibling { label, child: *child });
                node = *sibling;
            }
        } else {
            break;
        }
    }
    ForestZipper { focus: node, path }
}

// Reconstruct a forest from a ForestZipper.
fn reconstruct_forest(forest_zipper: ForestZipper) -> Node {
    let ForestZipper { focus, mut path } = forest_zipper;
    let mut current_node = focus;
    while let Some(direction) = path.pop() {
        current_node = match direction {
            Direction::Child { label, sibling } => {
                Node::new(label, current_node, sibling)
            }
            Direction::Sibling { label, child } => {
                Node::new(label, child, current_node)
            }
        };
    }
    current_node
}

// Concatenate two trees, making their roots siblings.
fn concat(left_root: Node, right_root: Node) -> Node {
    if let Node::Empty = right_root {
        return left_root;
    }
    let mut path = Vec::new();
    let mut current_node = left_root;
    while let Node::Node { label, child, sibling, .. } = current_node {
        path.push(Direction::Sibling{ label, child: *child });
        current_node = *sibling;
    }
    let forest = ForestZipper { focus: right_root, path };
    reconstruct_forest(forest)
}

// Attempt to separate the first two trees from a forest.
fn pop_two_trees(root: Node) -> TwoTrees {
    match root {
        Node::Node {
            label: label_1,
            child: child_1,
            sibling: sibling_1,
            ..
        } => match *sibling_1 {
            Node::Node {
                label: label_2,
                child: child_2,
                sibling: sibling_2,
                ..
            } => {
                let tree_1 = Tree { label: label_1, child: *child_1 };
                let tree_2 = Tree { label: label_2, child: *child_2 };
                TwoTrees::Success { tree_1, tree_2, rest: *sibling_2 }
            }
            Node::Empty => {
                let old_forest = Node::new(label_1, *child_1, Node::Empty);
                TwoTrees::Fail(old_forest)
            }
        }
        Node::Empty => TwoTrees::Fail(Node::Empty),
    }
}

impl Node {
    // Construct a new Node given its`label`, `child`, and `sibling`.
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

    /// Delete the node of pre-order `index` from the forest.
    pub fn delete(self, index: usize) -> Self {
        let ForestZipper { focus, path } = find_node(self, index);
        let new_focus = match focus {
            Self::Node { child, sibling, .. } => concat(*child, *sibling),
            Self::Empty => Self::Empty,
        };
        let forest = ForestZipper {
            focus: new_focus,
            path,
        };
        reconstruct_forest(forest)
    }

    /// Merge the first two trees, appending the result as the final tree.
    pub fn merge_pair(self, promote_first: bool) -> Self {
        match pop_two_trees(self) {
            TwoTrees::Success { tree_1, tree_2, rest } => {
                let (parent, child) = match promote_first {
                    true => (tree_1, tree_2),
                    false => (tree_2, tree_1),
                };
                let Tree { label: parent_label, child: old_child } = parent;
                let Tree { label: child_label, child: grandchild } = child;
                let new_child = Node::new(child_label, grandchild, old_child);
                let merged = Node::new(parent_label, new_child, Node::Empty);
                concat(rest, merged)
            }
            TwoTrees::Fail(root) => root,
        }
    }

    /// Return a reference to the label at the given pre-order `index`.
    pub fn label_at(&mut self, index: usize) -> &str {
        find_label(self, index)
            .expect("Invalid index")
    }

    /// Set the label at the given `index` to the `new_label`.
    pub fn set_label(&mut self, index: usize, new_label: String) {
        let label = find_label(self, index)
            .expect("Invalid index");
        *label = new_label;
    }

    /// Return the status of the forest roots.
    pub fn status(&self) -> ForestStatus {
        match self {
            Self::Empty => ForestStatus::Empty,
            Self::Node { label, sibling, .. } => match &**sibling {
                Self::Empty => ForestStatus::SingleRoot,
                Self::Node { label: label2, .. } => {
                    ForestStatus::MultiRoot(label, label2)
                }
            }
        }
    }

    // Create a corresponding NodeRef from a Node if non-empty.
    fn to_node(&self, node_type: NodeType) -> Option<NodeRef> {
        match self {
            Node::Empty => None,
            Node::Node { label, child, sibling, .. } => {
                let is_last = match **sibling {
                    Node::Empty => true,
                    Node::Node { .. } => false,
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

