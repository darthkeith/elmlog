pub mod iter;

use std::rc::Rc;

use serde::{Serialize, Deserialize};

// A node in a multi-way forest stored using child-sibling representation.
#[derive(Clone, Serialize, Deserialize)]
struct Node {
    child: Option<Rc<Node>>,
    next: Option<Rc<Node>>,
    label: Rc<str>,
}

// A node with a reversed sibling chain for leftward traversal.
#[derive(Clone, Serialize, Deserialize)]
struct RevNode {
    child: Option<Rc<Node>>,
    prev: Option<Rc<RevNode>>,
    label: Rc<str>,
}

// A node in the path from the focused node up to the root of its tree.
#[derive(Clone, Serialize, Deserialize)]
struct PathNode {
    parent: Option<Rc<PathNode>>,
    prev: Option<Rc<RevNode>>,
    next: Option<Rc<Node>>,
    label: Rc<str>,
}

/// The focused node in a zipper for a multi-way forest.
#[derive(Clone, Serialize, Deserialize)]
pub struct FocusNode {
    parent: Option<Rc<PathNode>>,
    child: Option<Rc<Node>>,
    prev: Option<Rc<RevNode>>,
    next: Option<Rc<Node>>,
    label: Rc<str>,
}


// Join two sibling chains into one forest.
fn join_siblings(
    mut left: Option<Rc<RevNode>>,
    mut right: Option<Rc<Node>>,
) -> Option<Rc<Node>> {
    while let Some(curr_rc) = left {
        let curr = Rc::unwrap_or_clone(curr_rc);
        left = curr.prev;
        let node = Node {
            child: curr.child,
            next: right,
            label: curr.label,
        };
        right = Some(Rc::new(node));
    }
    right
}

// Reverse the direction of the node’s sibling chain.
fn reverse_siblings(mut node: Option<Rc<Node>>) -> Option<Rc<RevNode>> {
    let mut reversed = None;
    while let Some(curr_rc) = node {
        let curr = Rc::unwrap_or_clone(curr_rc);
        node = curr.next;
        let rev_node = RevNode {
            child: curr.child,
            prev: reversed,
            label: curr.label,
        };
        reversed = Some(Rc::new(rev_node));
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
            label: Rc::from(""),
        }
    }

    /// Focus on the parent of the current focused node (if present).
    pub fn focus_parent(self) -> Self {
        match self.parent{
            Some(parent_rc) => {
                let parent = Rc::unwrap_or_clone(parent_rc);
                let node = Node {
                    child: self.child,
                    next: self.next,
                    label: self.label,
                };
                Self {
                    parent: parent.parent,
                    child: join_siblings(self.prev, Some(Rc::new(node))),
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
            Some(child_rc) => {
                let child = Rc::unwrap_or_clone(child_rc);
                let parent = PathNode {
                    parent: self.parent,
                    prev: self.prev,
                    next: self.next,
                    label: self.label,
                };
                Self {
                    parent: Some(Rc::new(parent)),
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
            Some(prev_rc) => {
                let prev = Rc::unwrap_or_clone(prev_rc);
                let next = Node {
                    child: self.child,
                    next: self.next,
                    label: self.label,
                };
                Self {
                    parent: self.parent,
                    child: prev.child,
                    prev: prev.prev,
                    next: Some(Rc::new(next)),
                    label: prev.label,
                }
            }
            None => self,
        }
    }

    /// Focus on the next sibling of the current focused node (if present).
    pub fn focus_next(self) -> Self {
        match self.next {
            Some(next_rc) => {
                let next = Rc::unwrap_or_clone(next_rc);
                let prev = RevNode {
                    child: self.child,
                    prev: self.prev,
                    label: self.label,
                };
                Self {
                    parent: self.parent,
                    child: next.child,
                    prev: Some(Rc::new(prev)),
                    next: next.next,
                    label: next.label,
                }
            }
            None => self,
        }
    }

    /// Move the focused node's subtree to be its parent's previous sibling.
    pub fn promote(self) -> Self {
        match self.parent {
            Some(parent_rc) => {
                let parent = Rc::unwrap_or_clone(parent_rc);
                let next = Node {
                    child: join_siblings(self.prev, self.next),
                    next: parent.next,
                    label: parent.label,
                };
                Self {
                    parent: parent.parent,
                    prev: parent.prev,
                    next: Some(Rc::new(next)),
                    ..self
                }
            }
            None => self,
        }
    }

    /// Move the focused node's subtree to be its next sibling's first child.
    pub fn demote(self) -> Self {
        match self.next {
            Some(next_rc) => {
                let next = Rc::unwrap_or_clone(next_rc);
                let parent = PathNode {
                    parent: self.parent,
                    prev: self.prev,
                    next: next.next,
                    label: next.label,
                };
                Self {
                    parent: Some(Rc::new(parent)),
                    prev: None,
                    next: next.child,
                    ..self
                }
            }
            None => self,
        }
    }

    /// Swap the focused node's subtree with its previous sibling (if present).
    pub fn swap_prev(self) -> Self {
        match self.prev {
            Some(prev_rc) => {
                let prev = Rc::unwrap_or_clone(prev_rc);
                let next = Node {
                    child: prev.child,
                    next: self.next,
                    label: prev.label,
                };
                Self {
                    prev: prev.prev,
                    next: Some(Rc::new(next)),
                    ..self
                }
            }
            None => self,
        }
    }

    /// Swap the focused node's subtree with its next sibling (if present).
    pub fn swap_next(self) -> Self {
        match self.next {
            Some(next_rc) => {
                let next = Rc::unwrap_or_clone(next_rc);
                let prev = RevNode {
                    child: next.child,
                    prev: self.prev,
                    label: next.label,
                };
                Self {
                    prev: Some(Rc::new(prev)),
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
            child: Some(Rc::new(child)),
            label: Rc::from(""),
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
            parent: Some(Rc::new(parent)),
            child: self.child,
            prev: None,
            next: None,
            label: Rc::from(""),
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
            next: Some(Rc::new(next)),
            label: Rc::from(""),
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
            prev: Some(Rc::new(prev)),
            label: Rc::from(""),
            ..self
        }
    }

    /// Delete the focused node.
    pub fn delete(self) -> Option<Self> {
        let focus = self.flatten();
        let new_focus = if let Some(next_rc) = focus.next {
            let next = Rc::unwrap_or_clone(next_rc);
            Self {
                parent: focus.parent,
                child: next.child,
                prev: focus.prev,
                next: next.next,
                label: next.label,
            }
        } else if let Some(prev_rc) = focus.prev {
            let prev = Rc::unwrap_or_clone(prev_rc);
            Self {
                parent: focus.parent,
                child: prev.child,
                prev: prev.prev,
                next: None,
                label: prev.label,
            }
        } else if let Some(parent_rc) = focus.parent {
            let parent = Rc::unwrap_or_clone(parent_rc);
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
        Self {
            label: Rc::from(label),
            ..self
        }
    }

    pub fn clone_label(&self) -> String {
        self.label.to_string()
    }
}
