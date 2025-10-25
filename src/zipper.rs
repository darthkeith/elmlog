pub mod iter;

use serde::{Serialize, Deserialize};

// A node in a multi-way forest stored using child-sibling representation.
#[derive(Serialize, Deserialize)]
struct Node {
    child: Option<Box<Node>>,
    next: Option<Box<Node>>,
    label: String,
}

// A node with a reversed sibling chain for leftward traversal.
#[derive(Serialize, Deserialize)]
struct RevNode {
    child: Option<Box<Node>>,
    prev: Option<Box<RevNode>>,
    label: String,
}

// A node in the path from the focused node up to the root of its tree.
#[derive(Serialize, Deserialize)]
struct PathNode {
    parent: Option<Box<PathNode>>,
    prev: Option<Box<RevNode>>,
    next: Option<Box<Node>>,
    label: String,
}

/// The focused node in a zipper for a multi-way forest.
#[derive(Serialize, Deserialize)]
pub struct FocusNode {
    parent: Option<Box<PathNode>>,
    child: Option<Box<Node>>,
    prev: Option<Box<RevNode>>,
    next: Option<Box<Node>>,
    label: String,
}


// Join two sibling chains into one forest.
fn join_siblings(
    mut left: Option<Box<RevNode>>,
    mut right: Option<Box<Node>>,
) -> Option<Box<Node>> {
    while let Some(curr) = left {
        left = curr.prev;
        let node = Node {
            child: curr.child,
            next: right,
            label: curr.label,
        };
        right = Some(Box::new(node));
    }
    right
}

// Reverse the direction of the node’s sibling chain.
fn reverse_siblings(mut node: Option<Box<Node>>) -> Option<Box<RevNode>> {
    let mut reversed = None;
    while let Some(curr) = node {
        node = curr.next;
        let rev_node = RevNode {
            child: curr.child,
            prev: reversed,
            label: curr.label,
        };
        reversed = Some(Box::new(rev_node));
    }
    reversed
}

impl FocusNode {
    /// Construct a forest containing a single node with empty label.
    pub fn new() -> Self {
        Self {
            parent: None,
            child: None,
            prev: None,
            next: None,
            label: String::new(),
        }
    }

    /// Focus on the parent of the current focused node (if present).
    pub fn focus_parent(self) -> Self {
        match self.parent{
            Some(parent) => {
                let node = Node {
                    child: self.child,
                    next: self.next,
                    label: self.label,
                };
                Self {
                    parent: parent.parent,
                    child: join_siblings(self.prev, Some(Box::new(node))),
                    prev: parent.prev,
                    next: parent.next,
                    label: parent.label,
                }
            }
            None => self,
        }
    }

    /// Focus on the first child of the current focused node (if present).
    pub fn focus_child(self) -> Self {
        match self.child{
            Some(child) => {
                let parent = PathNode {
                    parent: self.parent,
                    prev: self.prev,
                    next: self.next,
                    label: self.label,
                };
                Self {
                    parent: Some(Box::new(parent)),
                    child: child.child,
                    prev: None,
                    next: child.next,
                    label: child.label,
                }
            }
            None => self,
        }
    }

    /// Focus on the previous sibling of the current focused node (if present).
    pub fn focus_prev(self) -> Self {
        match self.prev {
            Some(prev) => {
                let next = Node {
                    child: self.child,
                    next: self.next,
                    label: self.label,
                };
                Self {
                    parent: self.parent,
                    child: prev.child,
                    prev: prev.prev,
                    next: Some(Box::new(next)),
                    label: prev.label,
                }
            }
            None => self,
        }
    }

    /// Focus on the next sibling of the current focused node (if present).
    pub fn focus_next(self) -> Self {
        match self.next {
            Some(next) => {
                let prev = RevNode {
                    child: self.child,
                    prev: self.prev,
                    label: self.label,
                };
                Self {
                    parent: self.parent,
                    child: next.child,
                    prev: Some(Box::new(prev)),
                    next: next.next,
                    label: next.label,
                }
            }
            None => self,
        }
    }

    /// Move the focused node's subtree to be its parent's next sibling.
    pub fn promote(self) -> Self {
        match self.parent {
            Some(parent) => {
                let prev = RevNode {
                    child: join_siblings(self.prev, self.next),
                    prev: parent.prev,
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
            Some(prev) => {
                let parent = PathNode {
                    parent: self.parent,
                    prev: prev.prev,
                    next: self.next,
                    label: prev.label,
                };
                Self {
                    parent: Some(Box::new(parent)),
                    prev: reverse_siblings(prev.child),
                    next: None,
                    ..self
                }
            }
            None => self,
        }
    }

    /// Swap the focused node's subtree with its previous sibling (if present).
    pub fn swap_prev(self) -> Self {
        match self.prev {
            Some(prev) => {
                let next = Node {
                    child: prev.child,
                    next: self.next,
                    label: prev.label,
                };
                Self {
                    prev: prev.prev,
                    next: Some(Box::new(next)),
                    ..self
                }
            }
            None => self,
        }
    }

    /// Swap the focused node's subtree with its next sibling (if present).
    pub fn swap_next(self) -> Self {
        match self.next {
            Some(next) => {
                let prev = RevNode {
                    child: next.child,
                    prev: self.prev,
                    label: next.label,
                };
                Self {
                    prev: Some(Box::new(prev)),
                    next: next.next,
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
            child: join_siblings(self.prev, child_plus_next),
            prev: None,
            next: None,
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
            child: None,
            next: child_plus_next,
            ..self
        }
    }

    /// Insert a new node as the parent of the focused node.
    pub fn insert_parent(self) -> Self {
        let child = Node {
            child: self.child,
            next: None,
            label: self.label,
        };
        Self {
            child: Some(Box::new(child)),
            label: String::new(),
            ..self
        }
    }

    /// Insert a new child node above the focused node's children.
    pub fn insert_child(self) -> Self {
        let parent = PathNode {
            parent: self.parent,
            prev: self.prev,
            next: self.next,
            label: self.label,
        };
        Self {
            parent: Some(Box::new(parent)),
            child: self.child,
            prev: None,
            next: None,
            label: String::new(),
        }
    }

    /// Insert a new node as the previous sibling of the focused node.
    pub fn insert_prev(self) -> Self {
        let next = Node {
            child: self.child,
            next: self.next,
            label: self.label,
        };
        Self {
            child: None,
            next: Some(Box::new(next)),
            label: String::new(),
            ..self
        }
    }

    /// Insert a new node as the next sibling of the focused node.
    pub fn insert_next(self) -> Self {
        let prev = RevNode {
            child: self.child,
            prev: self.prev,
            label: self.label,
        };
        Self {
            child: None,
            prev: Some(Box::new(prev)),
            label: String::new(),
            ..self
        }
    }

    /// Delete the focused node.
    pub fn delete(self) -> Option<Self> {
        let focus = self.flatten();
        let new_focus = if let Some(next) = focus.next {
            Self {
                parent: focus.parent,
                child: next.child,
                prev: focus.prev,
                next: next.next,
                label: next.label,
            }
        } else if let Some(prev) = focus.prev {
            Self {
                parent: focus.parent,
                child: prev.child,
                prev: prev.prev,
                next: None,
                label: prev.label,
            }
        } else if let Some(parent) = focus.parent {
            Self {
                parent: parent.parent,
                child: None,
                prev: parent.prev,
                next: parent.next,
                label: parent.label,
            }
        } else {
            return None;
        };
        Some(new_focus)
    }

    pub fn set_label(self, label: String) -> Self {
        Self { label, ..self }
    }

    pub fn clone_label(&self) -> String {
        self.label.clone()
    }
}

