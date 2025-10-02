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


// Join left (reverse-ordered) and right siblings into one forest.
fn join_siblings(
    mut left: Option<Box<Node>>,
    mut right: Option<Box<Node>>,
) -> Option<Box<Node>> {
    while let Some(left_sib) = left {
        left = left_sib.next;
        let node = Node {
            next: right,
            ..*left_sib
        };
        right = Some(Box::new(node));
    }
    right
}

// Reverse the order of the node’s sibling chain.
fn reverse_siblings(mut node: Option<Box<Node>>) -> Option<Box<Node>> {
    let mut reversed = None;
    while let Some(curr) = node {
        node = curr.next;
        let rev_node = Node {
            next: reversed,
            ..*curr
        };
        reversed = Some(Box::new(rev_node));
    }
    reversed
}

impl FocusNode {
    /// Swap the focused node's subtree with its next sibling's (if present).
    pub fn move_forward(self) -> Self {
        match self.next {
            Some(next_sib) => {
                let prev = Node {
                    next: self.prev,
                    ..*next_sib
                };
                Self {
                    prev: Some(Box::new(prev)),
                    next: next_sib.next,
                    ..self
                }
            }
            None => self,
        }
    }

    /// Swap the focused node's subtree with its previous sibling's (if present).
    pub fn move_backward(self) -> Self {
        match self.prev {
            Some(prev_sib) => {
                let next = Node {
                    next: self.next,
                    ..*prev_sib
                };
                Self {
                    prev: prev_sib.next,
                    next: Some(Box::new(next)),
                    ..self
                }
            }
            None => self,
        }
    }

    /// Move the focused node's subtree to be its parent's next sibling.
    pub fn promote(self) -> Self {
        match self.parent {
            Some(parent) => {
                let prev = Node {
                    next: parent.prev,
                    child: join_siblings(self.prev, self.next),
                    label: parent.label,
                };
                Self {
                    parent: parent.parent,
                    prev: Some(Box::new(prev)),
                    next: parent.next,
                    ..self
                }
            }
            None => self,
        }
    }
}

