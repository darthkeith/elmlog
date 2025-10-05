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
    while let Some(curr) = left {
        left = curr.next;
        let node = Node {
            next: right,
            ..*curr
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
    /// Focus on the previous sibling of the current focused node.
    pub fn focus_prev(self) -> Self {
        match self.prev {
            Some(prev) => {
                let next = Node {
                    next: self.next,
                    child: self.child,
                    label: self.label,
                };
                Self {
                    parent: self.parent,
                    prev: prev.next,
                    next: Some(Box::new(next)),
                    child: prev.child,
                    label: prev.label,
                }
            }
            None => self,
        }
    }

    /// Swap the focused node's subtree with its next sibling's (if present).
    pub fn swap_next(self) -> Self {
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
    pub fn swap_prev(self) -> Self {
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

    /// Move the focused node's subtree to be its previous sibling's last child.
    pub fn demote(self) -> Self {
        match self.prev {
            Some(prev_sib) => {
                let parent = PathNode {
                    parent: self.parent,
                    prev: prev_sib.next,
                    next: self.next,
                    label: prev_sib.label,
                };
                Self {
                    parent: Some(Box::new(parent)),
                    prev: reverse_siblings(prev_sib.child),
                    next: None,
                    ..self
                }
            }
            None => self,
        }
    }

    /// Adjoin the siblings of the focused node to its children, preserving order.
    pub fn nest(self) -> Self {
        let child_plus_next = join_siblings(
            reverse_siblings(self.child),
            self.next
        );
        Self {
            prev: None,
            next: None,
            child: join_siblings(self.prev, child_plus_next),
            ..self
        }
    }

    /// Insert the focused node's children before its subsequent siblings.
    pub fn flatten(self) -> Self {
        let child_plus_next = join_siblings(
            reverse_siblings(self.child),
            self.next
        );
        Self {
            next: child_plus_next,
            child: None,
            ..self
        }
    }

    /// Insert a new node as the parent of the focused node.
    pub fn insert_parent(self, label: String) -> Self {
        let node = Node {
            next: self.next,
            child: self.child,
            label: self.label,
        };
        Self {
            parent: self.parent,
            prev: None,
            next: None,
            child: join_siblings(self.prev, Some(Box::new(node))),
            label
        }
    }

    /// Insert a new child node above the focused node's children.
    pub fn insert_child(self, label: String) -> Self {
        let parent = PathNode {
            parent: self.parent,
            prev: self.prev,
            next: self.next,
            label: self.label,
        };
        Self {
            parent: Some(Box::new(parent)),
            prev: None,
            next: None,
            child: self.child,
            label
        }
    }

    /// Insert a new node as the prior sibling of the focused node.
    pub fn insert_before(self, label: String) -> Self {
        let node = Node {
            next: self.next,
            child: self.child,
            label: self.label,
        };
        Self {
            parent: self.parent,
            prev: self.prev,
            next: Some(Box::new(node)),
            child: None,
            label
        }
    }

    /// Insert a new node as the next sibling of the focused node.
    pub fn insert_after(self, label: String) -> Self {
        let node = Node {
            next: self.prev,
            child: self.child,
            label: self.label,
        };
        Self {
            parent: self.parent,
            prev: Some(Box::new(node)),
            next: self.next,
            child: None,
            label
        }
    }

    /// Delete the focused node.
    pub fn delete(self) -> Option<Self> {
        let focus = self.flatten();
        let new_focus = if let Some(next_sib) = focus.next {
            Self {
                parent: focus.parent,
                prev: focus.prev,
                next: next_sib.next,
                child: next_sib.child,
                label: next_sib.label,
            }
        } else if let Some(prev_sib) = focus.prev {
            Self {
                parent: focus.parent,
                prev: prev_sib.next,
                next: None,
                child: prev_sib.child,
                label: prev_sib.label,
            }
        } else if let Some(parent) = focus.parent {
            Self {
                parent: parent.parent,
                prev: parent.prev,
                next: parent.next,
                child: None,
                label: parent.label,
            }
        } else {
            return None;
        };
        Some(new_focus)
    }

    /// Set the label of the focused node.
    pub fn set_label(self, label: String) -> Self {
        Self { label, ..self }
    }
}

