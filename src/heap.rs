/// A node in a left-child right-sibling binary tree containing a string.
///
/// This represents an ordered forest of multi-way trees, where each node can
/// have any number of children and siblings (the roots are siblings of each
/// other).  `size` is the number of nodes in a node's (binary) subtree.
/// When referring to a forest, "the root" is the root of its left-most tree,
/// which is also the root node of its binary tree representation.
pub enum Heap {
    Empty,
    Node {
        label: String,
        child: Box<Heap>,
        sibling: Box<Heap>,
        size: usize,
    }
}

// A heap with a single tree.
struct Tree {
    label: String,
    child: Heap,
}

// Represents the direction taken through a node in a path through a heap.
enum Direction {
    Child { label: String, sibling: Heap },
    Sibling { label: String, child: Heap },
}

// Represents a path to a subheap within a heap.
struct PathToSubheap {
    path: Vec<Direction>,
    subheap: Heap,
}

// Represents an attempt to separate the first two trees from a heap.
enum TwoTrees {
    Success { tree_1: Tree, tree_2: Tree, rest: Heap},
    Fail(Heap),
}

/// Describes whether a node is a root, first child, or non-root right sibling.
pub enum NodeType {
    Root,
    Child,
    Sibling,
}

/// Describes the position of a node in a heap (used for display).
pub struct NodePosition {
    pub node_type: NodeType,
    pub is_last: bool,
}

// Represents a node with additional positional information for display.
struct Node<'a> {
    label: &'a str,
    child: &'a Heap,
    sibling: &'a Heap,
    pos: NodePosition,
}

/// Represents the number of roots in a heap (zero, one, or multiple).
///
/// When there is one root, a reference to its label is included.
pub enum HeapStatus<'a> {
    Empty,
    SingleRoot(&'a str),
    MultiRoot,
}

// Return a path to the subheap at the pre-order `index` in the `heap`.
//
// If the index is invalid, a path to an empty sub-heap is returned.
fn find_subheap(heap: Heap, index: usize) -> PathToSubheap {
    let mut i = index;
    let mut path = Vec::new();
    let mut current_heap = heap;
    loop {
        if i == 0 {
            break;
        }
        if let Heap::Node { label, child, sibling, .. } = current_heap {
            if i <= child.size() {
                i -= 1;
                path.push(Direction::Child { label, sibling: *sibling });
                current_heap = *child;
            } else {
                i -= 1 + child.size();
                path.push(Direction::Sibling { label, child: *child });
                current_heap = *sibling;
            }
        } else {
            break;
        }
    }
    return PathToSubheap { path, subheap: current_heap };
}

// Reconstruct a heap given a path to a subheap.
fn reconstruct_heap(path_to_subheap: PathToSubheap) -> Heap {
    let PathToSubheap { mut path, subheap } = path_to_subheap;
    let mut current_heap = subheap;
    while let Some(direction) = path.pop() {
        current_heap = match direction {
            Direction::Child { label, sibling } => {
                Heap::new(label, current_heap, sibling)
            }
            Direction::Sibling { label, child } => {
                Heap::new(label, child, current_heap)
            }
        };
    }
    current_heap
}

// Concatenate two heaps, making their roots siblings.
fn concat(left_heap: Heap, right_heap: Heap) -> Heap {
    if let Heap::Empty = right_heap {
        return left_heap;
    }
    let mut path = Vec::new();
    let mut current_heap = left_heap;
    while let Heap::Node { label, child, sibling, .. } = current_heap {
        path.push(Direction::Sibling{ label, child: *child });
        current_heap = *sibling;
    }
    let new_heap = PathToSubheap { path, subheap: right_heap };
    reconstruct_heap(new_heap)
}

// Attempt to separate the first two trees from a heap.
fn pop_two_trees(heap: Heap) -> TwoTrees {
    match heap {
        Heap::Node {
            label: label_1,
            child: child_1,
            sibling: sibling_1,
            ..
        } => match *sibling_1 {
            Heap::Node {
                label: label_2,
                child: child_2,
                sibling: sibling_2,
                ..
            } => {
                let tree_1 = Tree { label: label_1, child: *child_1 };
                let tree_2 = Tree { label: label_2, child: *child_2 };
                TwoTrees::Success { tree_1, tree_2, rest: *sibling_2 }
            }
            Heap::Empty => {
                let old_heap = Heap::new(label_1, *child_1, Heap::Empty);
                TwoTrees::Fail(old_heap)
            }
        }
        Heap::Empty => TwoTrees::Fail(Heap::Empty),
    }
}

impl Heap {
    // Construct a heap given the `label`, `child`, and `sibling` of its root.
    fn new(label: String, child: Self, sibling: Self) -> Self {
        let size = 1 + child.size() + sibling.size();
        Self::Node {
            label,
            child: Box::new(child),
            sibling: Box::new(sibling),
            size,
        }
    }

    /// Return the number of nodes in the heap.
    pub fn size(&self) -> usize {
        match self {
            Self::Empty => 0,
            Self::Node { size, .. } => *size,
        }
    }

    /// Return the number of roots in the heap.
    pub fn root_count(&self) -> usize {
        let mut heap = self;
        let mut count = 0;
        while let Heap::Node { sibling, .. } = heap {
            count += 1;
            heap = &*sibling;
        }
        count
    }

    /// Insert a node with the given `label` before the first tree in the heap.
    pub fn prepend(self, label: String) -> Self {
        Self::new(label, Self::Empty, self)
    }

    /// Delete the node with the pre-order `index` from the heap.
    pub fn delete(self, index: usize) -> Self {
        let PathToSubheap { path, subheap } = find_subheap(self, index);
        let new_subheap = match subheap {
            Self::Node { child, sibling, .. } => concat(*child, *sibling),
            Self::Empty => Self::Empty,
        };
        let new_heap = PathToSubheap {
            path,
            subheap: new_subheap,
        };
        reconstruct_heap(new_heap)
    }

    /// Merge the first two trees, appending the result as the final tree.
    pub fn merge_pair(self, promote_first: bool) -> Self {
        match pop_two_trees(self) {
            TwoTrees::Success { tree_1, tree_2, rest } => {
                let (parent, child) = match promote_first {
                    true => (tree_1, tree_2),
                    false => (tree_2, tree_1),
                };
                let Tree { label: parent_label, child: old_child } = parent;
                let Tree { label: child_label, child: grandchild } = child;
                let new_child = Heap::new(child_label, grandchild, old_child);
                let merged = Heap::new(parent_label, new_child, Heap::Empty);
                concat(rest, merged)
            }
            TwoTrees::Fail(heap) => heap
        }
    }

    /// Return the status of the heap (if there is one root, include its label).
    pub fn status(&self) -> HeapStatus {
        match self {
            Self::Empty => HeapStatus::Empty,
            Self::Node { label, sibling, .. } => match **sibling {
                Self::Empty => HeapStatus::SingleRoot(label),
                Self::Node { .. } => HeapStatus::MultiRoot,
            }
        }
    }

    // Convert a Heap into a Node if non-empty.
    fn to_node<'a>(&'a self, node_type: NodeType) -> Option<Node<'a>> {
        match self {
            Heap::Empty => None,
            Heap::Node { label, child, sibling, .. } => {
                let is_last = match **sibling {
                    Heap::Empty => true,
                    Heap::Node { .. } => false,
                };
                let pos = NodePosition { node_type, is_last };
                Some(Node { label, child, sibling, pos })
            }
        }
    }

    /// Return an iterator over node labels in pre-order.
    pub fn iter(&self) -> PreOrderIter {
        let mut stack = Vec::new();
        if let Some(node) = self.to_node(NodeType::Root) {
            stack.push(node);
        }
        PreOrderIter { stack }
    }
}

/// Iterator type for iterating over node labels in pre-order.
pub struct PreOrderIter<'a> {
    stack: Vec<Node<'a>>,
}

impl<'a> Iterator for PreOrderIter<'a> {
    type Item = (&'a str, NodePosition);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(node) = self.stack.pop() {
            let Node { label, child, sibling, pos } = node;
            let sibling_type = match pos.node_type {
                NodeType::Root => NodeType::Root,
                _ => NodeType::Sibling,
            };
            if let Some(node) = sibling.to_node(sibling_type) {
                self.stack.push(node);
            }
            if let Some(node) = child.to_node(NodeType::Child) {
                self.stack.push(node);
            }
            return Some((label, pos));
        }
        None
    }
}

