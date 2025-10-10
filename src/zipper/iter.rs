#![allow(dead_code)]
use crate::zipper::{
    Node,
    RevNode,
};

// Indicates a node's structural position within the forest.
#[derive(Clone, Copy)]
enum NodePosition {
    Root,
    FirstChild,
    SubsequentChild,
}

/// Information required to render each node in the TUI.
pub struct NodeInfo<'a> {
    label: &'a str,
    position: NodePosition,
    is_last_sibling: bool,
    is_focused: bool,
}

// A stack frame used during pre-order traversal of a Node.
struct Frame<'a> {
    node: &'a Node,
    position: NodePosition,
}

// Pre-order iterator yielding node information.
struct NodePreOrderIter<'a> {
    stack: Vec<Frame<'a>>,
}

// Iterator over a reversed sibling chain, left to right.
struct RevNodeIter<'a> {
    stack: Vec<&'a RevNode>,
}

impl<'a> Iterator for NodePreOrderIter<'a> {
    type Item = NodeInfo<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let Frame { node, position } = self.stack.pop()?;
        let is_last_sibling = if let Some(next) = &node.next {
            let next_pos = match position {
                NodePosition::Root => NodePosition::Root,
                _ => NodePosition::SubsequentChild,
            };
            let next_frame = Frame {
                node: next,
                position: next_pos,
            };
            self.stack.push(next_frame);
            false
        } else {
            true
        };
        if let Some(child) = &node.child {
            let child_frame = Frame {
                node: child,
                position: NodePosition::FirstChild,
            };
            self.stack.push(child_frame);
        }
        let node_info = NodeInfo {
            label: &node.label,
            position,
            is_last_sibling,
            is_focused: false,
        };
        Some(node_info)
    }
}

impl<'a> Iterator for RevNodeIter<'a> {
    type Item = &'a RevNode;

    fn next(&mut self) -> Option<Self::Item> {
        self.stack.pop()
    }
}

impl<'a> RevNodeIter<'a> {
    // Construct an iterator over a reversed sibling chain, left to right.
    fn new(mut prev: Option<&'a RevNode>) -> Self {
        let mut stack = Vec::new();
        while let Some(rev_node) = prev {
            stack.push(rev_node);
            prev = rev_node.prev.as_deref();
        }
        Self { stack }
    }
}

// Pre-order iterator over the subtree, if any, and its subsequent siblings.
fn node_iter(
    maybe_node: Option<&Node>,
    position: NodePosition,
) -> impl Iterator<Item = NodeInfo> {
    maybe_node.into_iter().flat_map(move |node| {
        NodePreOrderIter {
            stack: vec![Frame { node, position }]
        }
    })
}

// Pre-order iterator over the subtree, if any, and its previous siblings.
fn rev_node_iter(
    prev: Option<&RevNode>,
    is_root: bool,
) -> impl Iterator<Item = NodeInfo> {
    RevNodeIter::new(prev).flat_map(move |rev_node| {
        let position = if is_root {
            NodePosition::Root
        } else if rev_node.prev.is_none() {
            NodePosition::FirstChild
        } else {
            NodePosition::SubsequentChild
        };
        let info = NodeInfo {
            label: &rev_node.label,
            position,
            is_last_sibling: false,
            is_focused: false,
        };
        let child_iter = node_iter(
            rev_node.child.as_deref(),
            NodePosition::FirstChild,
        );
        std::iter::once(info).chain(child_iter)
    })
}

