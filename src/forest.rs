use serde::{Serialize, Deserialize};

/// A node in a left-child right-sibling binary tree, containing a string.
///
/// The `size` field stores the size of the node's binary subtree.
/// The binary tree represents a forest of multi-way trees, where each node can
/// have any number of children and siblings (the roots are siblings).
#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Debug)]
pub enum Node {
    Empty,
    Node {
        label: String,
        child: Box<Node>,
        sibling: Box<Node>,
        size: usize,
    },
}

// Represents a node in the path from the focused node to the root.
#[derive(PartialEq, Eq, Debug)]
enum ReturnNode {
    Parent { label: String, prev: Box<ReturnNode>, sibling: Node },
    Sibling { label: String, prev: Box<ReturnNode>, child: Node },
    Empty,
}

// Zipper represention of a forest focused on a node.
#[derive(PartialEq, Eq, Debug)]
struct ForestZipper {
    focus: Node,
    prev: ReturnNode,
}

// The root of a single tree.
enum Tree {
    Root { label: String, child: Node },
    Empty,
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

    // Return a zipper focused on the node of pre-order `index` in the forest.
    // If the index is invalid, the zipper will be focused on an empty node
    // and behavior is undefined.
    fn focus_node(self: Node, index: usize) -> ForestZipper {
        let mut i = index;
        let mut focus = self;
        let mut prev = ReturnNode::Empty;
        while i > 0 {
            match focus {
                Node::Node { label, child, sibling, .. } => {
                    if i <= child.size() {
                        i -= 1;
                        focus = *child;
                        prev = ReturnNode::new_parent(label, prev, *sibling);
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

    /// Return the label at pre-order `index` (panic if invalid).
    pub fn find_label(&self, index: usize) -> String {
        let mut i = index;
        let mut node = self;
        while i > 0 {
            match node {
                Self::Node { child, sibling, .. } => {
                    if i <= child.size() {
                        i -= 1;
                        node = child;
                    } else {
                        i -= 1 + child.size();
                        node = sibling;
                    }
                }
                Self::Empty => break,
            }
        }
        match node {
            Self::Node { label, .. } => label.clone(),
            Self::Empty => panic!("Invalid index"),
        }
    }

    /// Assign the `label` to the node at `index`.
    pub fn set_label(self, index: usize, label: String) -> Self {
        let ForestZipper { focus, prev } = self.focus_node(index);
        let focus = match focus {
            Self::Node { child, sibling, size, .. } => {
                Self::Node { label, child, sibling, size }
            }
            Self::Empty => Self::Empty,
        };
        ForestZipper { focus, prev }
            .restore()
    }

    /// Insert a Node with the `label` at the start of the forest.
    pub fn prepend(self, label: String) -> Self {
        Self::new(label, Self::Empty, self)
    }

    /// Swap the subtree at `index` with its next sibling.
    pub fn move_forward(self, index: usize) -> (Self, usize) {
        self.focus_node(index)
            .move_forward()
            .restore_with_index()
    }

    /// Swap the subtree at `index` with its previous sibling.
    pub fn move_backward(self, index: usize) -> (Self, usize) {
        self.focus_node(index)
            .move_backward()
            .restore_with_index()
    }

    /// Move subtree at `index` to be its parent's next sibling.
    ///
    /// If it has no parent, move it to be the first tree in the forest.
    pub fn promote(self, index: usize) -> (Self, usize) {
        self.focus_node(index)
            .promote()
            .restore_with_index()
    }

    /// Move subtree at `index` to be its previous sibling's last child.
    pub fn demote(self, index: usize) -> (Self, usize) {
        self.focus_node(index)
            .demote()
            .restore_with_index()
    }

    /// Move the siblings of the node at `index` to be its children.
    pub fn raise(self, index: usize) -> (Self, usize) {
        self.focus_node(index)
            .raise()
            .restore_with_index()
    }

    /// Move the children of the node at `index` to be its subsequent siblings.
    pub fn flatten(self, index: usize) -> (Self, usize) {
        self.focus_node(index)
            .flatten()
            .restore_with_index()
    }

    // Insert a node with `label` as the parent of the node at `index`.
    fn insert_parent(self, index: usize, label: String) -> (Self, usize) {
        self.focus_node(index)
            .insert_parent(label)
            .restore_with_index()
    }
    
    // Concatenate the roots of a forest as siblings of the roots of `self`.
    fn concat(self, root: Node) -> Node {
        if let Node::Empty = root {
            return self;
        }
        let mut focus = self;
        let mut prev = ReturnNode::Empty;
        while let Node::Node { label, child, sibling, .. } = focus {
            focus = *sibling;
            prev = ReturnNode::new_sibling(label, prev, *child);
        }
        ForestZipper { focus: root, prev }
            .restore()
    }

    /// Delete the node of pre-order `index` from the forest.
    pub fn delete(self, index: usize) -> Self {
        let ForestZipper { focus, prev } = self.focus_node(index);
        let new_focus = match focus {
            Self::Node { child, sibling, .. } => child.concat(*sibling),
            Self::Empty => Self::Empty,
        };
        ForestZipper { focus: new_focus, prev, }
            .restore()
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
    fn new_parent(label: String, prev: Self, sibling: Node) -> Self {
        Self::Parent {
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
                ReturnNode::Parent { label, prev, sibling } => {
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
    // Behavior is undefined when the focused node is empty.
    fn restore_with_index(self) -> (Node, usize) {
        let Self { mut focus, mut prev } = self;
        let mut i = 0;
        loop {
            prev = match prev {
                ReturnNode::Parent { label, prev, sibling } => {
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

    // Swap the focused node's subtree with its next sibling (if present).
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

    // Swap the focused node's subtree with its previous sibling (if present).
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

    // Extract the subtree of the focused node from the forest.
    fn extract_tree(self) -> (Self, Tree) {
        match self.focus {
            Node::Node { label, child, sibling, .. } => {
                let zipper = Self { focus: *sibling, ..self };
                let tree = Tree::Root { label, child: *child };
                (zipper, tree)
            }
            Node::Empty => (self, Tree::Empty),
        }
    }

    // Move the focused node's subtree to be its parent's next sibling.
    // If it has no parent, move it to be the first tree in the forest.
    fn promote(self) -> Self {
        let (zipper, tree) = self.extract_tree();
        let (root_label, root_child) = match tree {
            Tree::Root { label, child } => (label, child),
            Tree::Empty => return zipper,
        };
        let Self { mut focus, mut prev } = zipper;
        loop {
            match prev {
                ReturnNode::Sibling { label, prev: prev2, child } => {
                    focus = Node::new(label, child, focus);
                    prev = *prev2;
                }
                ReturnNode::Parent { label, prev, sibling } => {
                    let prev = ReturnNode::new_sibling(label, *prev, focus);
                    let focus = Node::new(root_label, root_child, sibling);
                    return Self { focus, prev };
                }
                ReturnNode::Empty => {
                    let focus = Node::new(root_label, root_child, focus);
                    return Self { focus, prev };
                }
            };
        }
    }

    // Move the focused node's subtree to be its previous sibling's last child.
    fn demote(self) -> Self {
        let (zipper, tree) = self.extract_tree();
        let (root_label, root_child) = match tree {
            Tree::Root { label, child } => (label, child),
            Tree::Empty => return zipper,
        };
        let Self { focus, prev } = zipper;
        if let ReturnNode::Sibling { label, prev, child } = prev {
            let mut prev = ReturnNode::new_parent(label, *prev, focus);
            let mut focus = child;
            while let Node::Node { label, child, sibling, .. } = focus {
                prev = ReturnNode::new_sibling(label, prev, *child);
                focus = *sibling;
            }
            focus = Node::new(root_label, root_child, Node::Empty);
            Self { focus, prev }
        } else {
            let focus = Node::new(root_label, root_child, focus);
            Self { focus, prev }
        }
    }

    // Return a zipper focused on the first sibling of the focused node.
    fn focus_first_sibling(self) -> Self {
        let Self { mut focus, mut prev } = self;
        while let ReturnNode::Sibling { label, prev: prev2, child } = prev {
            focus = Node::new(label, child, focus);
            prev = *prev2;
        }
        Self { focus, prev }
    }

    // Move the siblings of the focused node to be its children.
    fn raise(self) -> Self {
        let Self { focus, prev } = self;
        if let Node::Node { label, child, sibling, .. } = focus {
            let focus = child.concat(*sibling);
            let Self { focus, prev } = Self { focus, prev }
                .focus_first_sibling();
            let focus = Node::new(label, focus, Node::Empty);
            Self { focus, prev }
        } else {
            Self { focus, prev }
        }
    }

    // Move the children of the focused node to be its subsequent siblings.
    fn flatten(self) -> Self {
        let Self { focus, prev } = self;
        let focus = match focus {
            Node::Node { label, child, sibling, .. } => {
                Node::new(label, Node::Empty, child.concat(*sibling))
            }
            Node::Empty => Node::Empty,
        };
        Self { focus, prev }
    }

    // Insert a node with `label` as the parent of the focused node.
    fn insert_parent(self, label: String) -> Self {
        self
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

#[cfg(test)]
mod tests {
    use once_cell::sync::Lazy;

    use super::*;

    // Create a forest from a list of `trees`.
    fn forest(mut trees: Vec<Tree>) -> Node {
        let mut root = Node::Empty;
        while let Some(tree) = trees.pop() {
            if let Tree::Root { label, child } = tree {
                root = Node::new(label, child, root);
            }
        }
        root
    }

    // Create a tree with root `label` and list of `children` subtrees.
    fn tree(label: &str, children: Vec<Tree>) -> Tree {
        Tree::Root {
            label: label.to_string(),
            child: forest(children),
        }
    }

    // Create a single-node tree with root `label`.
    fn leaf(label: &str) -> Tree {
        Tree::Root {
            label: label.to_string(),
            child: Node::Empty,
        }
    }

    static FOREST_A: Lazy<Node> = Lazy::new(|| {
        forest(vec![
            leaf("0"),
            tree("1", vec![
                leaf("2"),
                leaf("3"),
            ]),
            leaf("4"),
        ])
    });

    static FOREST_B: Lazy<Node> = Lazy::new(|| {
        forest(vec![
            tree("0", vec![
                tree("1", vec!{
                    leaf("2"),
                    leaf("3"),
                }),
                tree("4", vec!{
                    leaf("5"),
                }),
            ]),
            leaf("6"),
        ])
    });

    #[test]
    fn focus_empty_forest() {
        let result_0 = Node::Empty.focus_node(0);
        let result_1 = Node::Empty.focus_node(1);
        let empty_zipper = ForestZipper {
            focus: Node::Empty,
            prev: ReturnNode::Empty,
        };
        assert_eq!(result_0, empty_zipper);
        assert_eq!(result_1, empty_zipper);
        assert_eq!(empty_zipper.restore(), Node::Empty);
    }

    #[test]
    fn focus_and_restore_forest() {
        let zipper = FOREST_A.clone().focus_node(1);
        let focus = forest(vec![
            tree("1", vec![
                leaf("2"),
                leaf("3"),
            ]),
            leaf("4"),
        ]);
        assert_eq!(zipper.focus, focus);
        assert_eq!(zipper.restore(), *FOREST_A);

        let zipper = FOREST_A.clone().focus_node(2);
        let focus = forest(vec![
            leaf("2"),
            leaf("3"),
        ]);
        assert_eq!(zipper.focus, focus);
        assert_eq!(zipper.restore(), *FOREST_A);
    }

    #[test]
    fn restore_forest_invalid_focus() {
        let zipper = FOREST_A.clone().focus_node(9);
        assert_eq!(zipper.focus, Node::Empty);
        assert_eq!(zipper.restore(), *FOREST_A);
    }

    #[test]
    fn test_raise() {
        let (result, index) = FOREST_A.clone().raise(0);
        let (result2, index2) = result.clone().raise(index);
        let expected = forest(vec![
            tree("0", vec![
                tree("1", vec![
                    leaf("2"),
                    leaf("3"),
                ]),
                leaf("4"),
            ]),
        ]);
        assert_eq!(result, expected);
        assert_eq!(result2, expected);
        assert_eq!(index, 0);
        assert_eq!(index2, 0);

        let (result, index) = FOREST_A.clone().raise(3);
        let (result2, index2) = result.clone().raise(index);
        let expected = forest(vec![
            leaf("0"),
            tree("1", vec![
                tree("3", vec![
                    leaf("2"),
                ]),
            ]),
            leaf("4"),
        ]);
        assert_eq!(result, expected);
        assert_eq!(result2, expected);
        assert_eq!(index, 2);
        assert_eq!(index2, 2);
    }

    #[test]
    fn test_flatten() {
        let (result, index) = FOREST_B.clone().flatten(0);
        let (result2, index2) = result.clone().flatten(index);
        let expected = forest(vec![
            leaf("0"),
            tree("1", vec!{
                leaf("2"),
                leaf("3"),
            }),
            tree("4", vec!{
                leaf("5"),
            }),
            leaf("6"),
        ]);
        assert_eq!(result, expected);
        assert_eq!(result2, expected);
        assert_eq!(index, 0);
        assert_eq!(index2, 0);
        
        let (result, index) = FOREST_B.clone().flatten(1);
        let (result2, index2) = result.clone().flatten(index);
        let expected = forest(vec![
            tree("0", vec![
                leaf("1"),
                leaf("2"),
                leaf("3"),
                tree("4", vec!{
                    leaf("5"),
                }),
            ]),
            leaf("6"),
        ]);
        assert_eq!(result, expected);
        assert_eq!(result2, expected);
        assert_eq!(index, 1);
        assert_eq!(index2, 1);
    }

    #[test]
    fn test_insert_parent() {
        let (result, index) = FOREST_A.clone().insert_parent(0, "x".to_string());
        let expected = forest(vec![
            tree("x", vec![
                leaf("0"),
            ]),
            tree("1", vec![
                leaf("2"),
                leaf("3"),
            ]),
            leaf("4"),
        ]);
        assert_eq!(result, expected);
        assert_eq!(index, 0);

        let (result, index) = FOREST_A.clone().insert_parent(3, "x".to_string());
        let expected = forest(vec![
            leaf("0"),
            tree("1", vec![
                leaf("2"),
                tree("x", vec![
                    leaf("3"),
                ]),
            ]),
            leaf("4"),
        ]);
        assert_eq!(result, expected);
        assert_eq!(index, 3);
    }
}

