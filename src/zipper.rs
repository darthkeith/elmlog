#![allow(dead_code)]

/// A node in a multi-way forest stored using child-sibling representation.
struct Node {
    next: Option<Box<Node>>,
    child: Option<Box<Node>>,
    label: String,
}

/// A node in the path from the focused node up to the root of its tree.
struct PathNode {
    parent: Option<Box<PathNode>>,
    prev: Option<Box<Node>>,
    next: Option<Box<Node>>,
    label: String,
}

/// The focused node in a zipper for a multi-way forest.
struct FocusNode {
    parent: Option<Box<PathNode>>,
    prev: Option<Box<Node>>,
    next: Option<Box<Node>>,
    child: Option<Box<Node>>,
    label: String,
}

impl FocusNode {
    /// Swap the focused node's subtree with its next sibling's (if present).
    pub fn move_forward(self) -> Self {
        match self.next {
            Some(sibling) => {
                let prev = Node {
                    next: self.prev,
                    ..*sibling
                };
                Self {
                    prev: Some(Box::new(prev)),
                    next: sibling.next,
                    ..self
                }
            }
            None => self,
        }
    }

    /// Swap the focused node's subtree with its previous sibling's (if present).
    pub fn move_backward(self) -> Self {
        match self.prev {
            Some(sibling) => {
                let next = Node {
                    next: self.next,
                    ..*sibling
                };
                Self {
                    prev: sibling.next,
                    next: Some(Box::new(next)),
                    ..self
                }
            }
            None => self,
        }
    }
}

