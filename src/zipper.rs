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

