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

    /// Return an iterator over the heap's labels in pre-order.
    pub fn iter(&self) -> PreOrderIter {
        PreOrderIter { stack: vec![self] }
    }
}

/// Iterator type for iterating over a heap's labels in pre-order.
pub struct PreOrderIter<'a> {
    stack: Vec<&'a Heap>,
}

impl<'a> Iterator for PreOrderIter<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(heap) = self.stack.pop() {
            match heap {
                Heap::Empty => (),
                Heap::Node { label, child, sibling, .. } => {
                    self.stack.push(sibling);
                    self.stack.push(child);
                    return Some(&label);
                }
            }
        }
        None
    }
}

