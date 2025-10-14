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
    /// Construct a forest containing a single node with `label`.
    pub fn new(label: String) -> Self {
        Self {
            parent: None,
            child: None,
            prev: None,
            next: None,
            label,
        }
    }

    // Focus on the parent of the current focused node (if present).
    fn focus_parent(self) -> Self {
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

    // Focus on the first child of the current focused node (if present).
    fn focus_child(self) -> Self {
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

    // Focus on the previous sibling of the current focused node (if present).
    fn focus_prev(self) -> Self {
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

    // Focus on the next sibling of the current focused node (if present).
    fn focus_next(self) -> Self {
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

    // Move the focused node's subtree to be its parent's next sibling.
    fn promote(self) -> Self {
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

    // Move the focused node's subtree to be its previous sibling's last child.
    fn demote(self) -> Self {
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

    // Swap the focused node's subtree with its previous sibling (if present).
    fn swap_prev(self) -> Self {
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

    // Swap the focused node's subtree with its next sibling (if present).
    fn swap_next(self) -> Self {
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

    // Adjoin the siblings of the focused node to its children, preserving order.
    fn nest(self) -> Self {
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

    // Insert the focused node's children before its subsequent siblings.
    fn flatten(self) -> Self {
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

    // Insert a new node as the parent of the focused node.
    fn insert_parent(self, label: String) -> Self {
        let node = Node {
            child: self.child,
            next: self.next,
            label: self.label,
        };
        Self {
            parent: self.parent,
            child: join_siblings(self.prev, Some(Box::new(node))),
            prev: None,
            next: None,
            label
        }
    }

    // Insert a new child node above the focused node's children.
    fn insert_child(self, label: String) -> Self {
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
            label
        }
    }

    // Insert a new node as the previous sibling of the focused node.
    fn insert_prev(self, label: String) -> Self {
        let next = Node {
            child: self.child,
            next: self.next,
            label: self.label,
        };
        Self {
            parent: self.parent,
            child: None,
            prev: self.prev,
            next: Some(Box::new(next)),
            label
        }
    }

    // Insert a new node as the next sibling of the focused node.
    fn insert_next(self, label: String) -> Self {
        let prev = RevNode {
            child: self.child,
            prev: self.prev,
            label: self.label,
        };
        Self {
            parent: self.parent,
            child: None,
            prev: Some(Box::new(prev)),
            next: self.next,
            label
        }
    }

    // Delete the focused node.
    fn delete(self) -> Option<Self> {
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

    fn set_label(self, label: String) -> Self {
        Self { label, ..self }
    }

    fn clone_label(&self) -> String {
        self.label.clone()
    }
}

pub trait FocusNodeExt {
    fn focus_parent(self) -> Option<FocusNode>;
    fn focus_child(self) -> Option<FocusNode>;
    fn focus_prev(self) -> Option<FocusNode>;
    fn focus_next(self) -> Option<FocusNode>;
    fn promote(self) -> Option<FocusNode>;
    fn demote(self) -> Option<FocusNode>;
    fn swap_prev(self) -> Option<FocusNode>;
    fn swap_next(self) -> Option<FocusNode>;
    fn nest(self) -> Option<FocusNode>;
    fn flatten(self) -> Option<FocusNode>;
    fn insert_parent(self, label: String) -> Option<FocusNode>;
    fn insert_child(self, label: String) -> Option<FocusNode>;
    fn insert_prev(self, label: String) -> Option<FocusNode>;
    fn insert_next(self, label: String) -> Option<FocusNode>;
    fn delete(self) -> Option<FocusNode>;
    fn set_label(self, label: String) -> Option<FocusNode>;
    fn clone_label(&self) -> Option<String>;
}

impl FocusNodeExt for Option<FocusNode> {
    fn focus_parent(self) -> Option<FocusNode> {
        self.map(|focus| focus.focus_parent())
    }
    fn focus_child(self) -> Option<FocusNode> {
        self.map(|focus| focus.focus_child())
    }
    fn focus_prev(self) -> Option<FocusNode> {
        self.map(|focus| focus.focus_prev())
    }
    fn focus_next(self) -> Option<FocusNode> {
        self.map(|focus| focus.focus_next())
    }
    fn promote(self) -> Option<FocusNode> {
        self.map(|focus| focus.promote())
    }
    fn demote(self) -> Option<FocusNode> {
        self.map(|focus| focus.demote())
    }
    fn swap_prev(self) -> Option<FocusNode> {
        self.map(|focus| focus.swap_prev())
    }
    fn swap_next(self) -> Option<FocusNode> {
        self.map(|focus| focus.swap_next())
    }
    fn nest(self) -> Option<FocusNode> {
        self.map(|focus| focus.nest())
    }
    fn flatten(self) -> Option<FocusNode> {
        self.map(|focus| focus.flatten())
    }
    fn insert_parent(self, label: String) -> Option<FocusNode> {
        self.map(|focus| focus.insert_parent(label))
    }
    fn insert_child(self, label: String) -> Option<FocusNode> {
        self.map(|focus| focus.insert_child(label))
    }
    fn insert_prev(self, label: String) -> Option<FocusNode> {
        self.map(|focus| focus.insert_prev(label))
    }
    fn insert_next(self, label: String) -> Option<FocusNode> {
        self.map(|focus| focus.insert_next(label))
    }
    fn delete(self) -> Option<FocusNode> {
        self.and_then(|focus| focus.delete())
    }
    fn set_label(self, label: String) -> Option<FocusNode> {
        self.map(|focus| focus.set_label(label))
    }
    fn clone_label(&self) -> Option<String> {
        self.as_ref().map(|focus| focus.clone_label())
    }
}

