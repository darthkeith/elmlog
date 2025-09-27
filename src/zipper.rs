#![allow(dead_code)]

struct Node {
    child: Option<Box<Node>>,
    sibling: Option<Box<Node>>,
    label: String,
}

enum PathNodeKind {
    Parent { sibling: Option<Box<Node>> },
    Sibling { child: Option<Box<Node>> },
}

struct PathNode {
    context: Option<Box<PathNode>>,
    kind: PathNodeKind,
    label: String,
}

struct FocusNode {
    context: Option<Box<PathNode>>,
    child: Option<Box<Node>>,
    sibling: Option<Box<Node>>,
    label: String,
}

impl FocusNode {
    // Swap the focused subtree with its next sibling's (if present).
    pub fn move_forward(self) -> Self {
        match self.sibling {
            Some(sib_node) => {
                let path_node = PathNode {
                    context: self.context,
                    kind: PathNodeKind::Sibling{ child: sib_node.child },
                    label: sib_node.label,
                };
                FocusNode {
                    context: Some(Box::new(path_node)),
                    child: self.child,
                    sibling: sib_node.sibling,
                    label: self.label,
                }
            }
            None => self,
        }
    }
}

