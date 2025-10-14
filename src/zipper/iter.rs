use crate::zipper::{
    Node,
    RevNode,
    FocusNode,
};

// Indicates a node's structural position within the forest.
#[derive(Clone, Copy)]
pub enum NodePosition {
    Root,
    FirstChild,
    SubsequentChild,
}

/// Information required to render each node in the TUI.
pub struct NodeInfo<'a> {
    pub label: &'a str,
    pub position: NodePosition,
    pub is_last_sibling: bool,
    pub is_focused: bool,
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
    mut prev: Option<&RevNode>,
    is_root: bool,
) -> impl Iterator<Item = NodeInfo> {
    let mut stack = Vec::new();
    while let Some(rev_node) = prev {
        stack.push(rev_node);
        prev = rev_node.prev.as_deref();
    }
    let prev_iter = std::iter::from_fn(move || stack.pop());
    prev_iter.flat_map(move |rev_node| {
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
        std::iter::once(info)
            .chain(child_iter)
    })
}

// Pre-order iterator over the focused node and its siblings' subtrees.
fn siblings_iter(focus: &FocusNode) -> impl Iterator<Item = NodeInfo> {
    let is_root = focus.parent.is_none();
    let (position, next_pos) = if is_root {
        (NodePosition::Root, NodePosition::Root)
    } else if focus.prev.is_none() {
        (NodePosition::FirstChild, NodePosition::SubsequentChild)
    } else {
        (NodePosition::SubsequentChild, NodePosition::SubsequentChild)
    };
    let focus_info = NodeInfo {
        label: &focus.label,
        position,
        is_last_sibling: focus.next.is_none(),
        is_focused: true,
    };
    let prev_iter = rev_node_iter(focus.prev.as_deref(), is_root);
    let focus_iter = std::iter::once(focus_info);
    let child_iter = node_iter(focus.child.as_deref(), NodePosition::FirstChild);
    let next_iter = node_iter(focus.next.as_deref(), next_pos);
    prev_iter
        .chain(focus_iter)
        .chain(child_iter)
        .chain(next_iter)
}

/// Pre-order iterator over all nodes in the forest.
pub fn focus_iter(focus: &FocusNode) -> impl Iterator<Item = NodeInfo> {
    let mut iter: Box<dyn Iterator<Item = NodeInfo>> =
        Box::new(siblings_iter(focus));
    let ancestors = std::iter::successors(
        focus.parent.as_deref(),
        |path_node| path_node.parent.as_deref()
    );
    for path_node in ancestors {
        let is_root = path_node.parent.is_none();
        let (position, next_pos) = if is_root {
            (NodePosition::Root, NodePosition::Root)
        } else if path_node.prev.is_none() {
            (NodePosition::FirstChild, NodePosition::SubsequentChild)
        } else {
            (NodePosition::SubsequentChild, NodePosition::SubsequentChild)
        };
        let path_node_info = NodeInfo {
            label: &path_node.label,
            position,
            is_last_sibling: path_node.next.is_none(),
            is_focused: false,
        };
        let prev_iter = rev_node_iter(path_node.prev.as_deref(), is_root);
        let path_node_iter = std::iter::once(path_node_info);
        let next_iter = node_iter(path_node.next.as_deref(), next_pos);
        iter = Box::new(
            prev_iter
                .chain(path_node_iter)
                .chain(iter)
                .chain(next_iter)
        );
    }
    iter
}

