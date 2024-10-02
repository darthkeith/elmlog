/// An ordered forest of multi-way trees containing strings.
///
/// Each node can have any number of children and siblings (the roots are
/// siblings of each other). The data structure is implemented as a left-child
/// right-sibling binary tree.
pub struct Heap {
    root: Option<Box<Node>>,
}

// A node in a left-child right-sibling binary tree containing a string.
//
// In the binary tree, `size` is the number of nodes in the node's subtree.
// In the represented heap, `size` is the number of nodes in the node's
// subtree plus the number of nodes in all subsequent sibling subtrees.
struct Node {
    label: String,
    child: Heap,
    sibling: Heap,
    size: usize,
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

// Construct a new node given its `label`, left `child`, and right `sibling`.
fn new_node(label: String, child: Heap, sibling: Heap) -> Box<Node> {
    let size = 1 + child.size() + sibling.size();
    let node = Node {
        label,
        child,
        sibling,
        size,
    };
    Box::new(node)
}

// Move the root node out of the `heap` if present.
fn move_root(heap: Heap) -> Option<Node> {
    heap.root.map(|boxed| *boxed)
}

// Return a path to the subheap at the pre-order `index` in the `heap`.
fn find_subheap(heap: Heap, index: usize) -> PathToSubheap {
    let mut i = index;
    let mut path = Vec::new();
    let mut current_heap = heap;
    loop {
        if i == 0 {
            return PathToSubheap { path, subheap: current_heap };
        }
        let Node { label, child, sibling, size } = move_root(current_heap)
            .expect("Invalid index.");
        if i <= child.size() {
            i -= 1;
            path.push(Direction::Child { label, sibling });
            current_heap = child;
        } else {
            i -= 1 + child.size();
            path.push(Direction::Sibling { label, child });
            current_heap = sibling;
        }
    }
}

// Reconstruct a heap given a path to a subheap.
fn reconstruct_heap(path_to_subheap: PathToSubheap) -> Heap {
    let PathToSubheap { mut path, subheap } = path_to_subheap;
    let mut current_heap = subheap;
    while let Some(direction) = path.pop() {
        let node = match direction {
            Direction::Child { label, sibling } => {
                new_node(label, current_heap, sibling)
            }
            Direction::Sibling { label, child } => {
                new_node(label, child, current_heap)
            }
        };
        current_heap = Heap { root: Some(node) };
    }
    current_heap
}

impl Heap {
    /// Contstruct an empty heap.
    pub fn empty() -> Self {
        Heap { root: None }
    }

    /// Return the number of nodes in the heap.
    pub fn size(&self) -> usize {
        match self.root {
            Some(ref node) => node.size,
            None => 0,
        }
    }

    /// Insert a node with the given `label` before the first tree in the heap.
    pub fn prepend(self, label: String) -> Self {
        let root = Some(new_node(label, Self::empty(), self));
        Heap { root }
    }

    /// Return the status of the heap (if there is one root, include its label).
    pub fn status(&self) -> HeapStatus {
        match &self.root {
            None => HeapStatus::Empty,
            Some(node) => match &node.sibling.root {
                None => HeapStatus::SingleRoot(&node.label),
                Some(_) => HeapStatus::MultiRoot,
            }
        }
    }

    /// Return an iterator over the heap's labels in pre-order.
    pub fn iter(&self) -> PreOrderIter {
        let mut stack = Vec::new();
        if let Some(root) = &self.root {
            stack.push(root.as_ref());
        }
        PreOrderIter { stack }
    }
}

/// Iterator type for iterating over a heap's labels in pre-order.
pub struct PreOrderIter<'a> {
    stack: Vec<&'a Node>,
}

impl<'a> Iterator for PreOrderIter<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(node) = self.stack.pop() {
            if let Some(sibling) = &node.sibling.root {
                self.stack.push(sibling);
            }
            if let Some(child) = &node.child.root {
                self.stack.push(child);
            }
            return Some(&node.label);
        }
        None
    }
}

