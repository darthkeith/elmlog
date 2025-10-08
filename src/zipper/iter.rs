#![allow(dead_code)]
use crate::zipper::Node;

// Indicates a node's structural position within the forest.
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

// Iterator yielding node information in pre-order.
struct PreOrderIter<'a> {
    stack: Vec<Frame<'a>>,
}

impl<'a> Iterator for PreOrderIter<'a> {
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

fn iter_node(node: &Node, position: NodePosition) -> PreOrderIter {
    PreOrderIter {
        stack: vec![Frame { node, position }]
    }
}

