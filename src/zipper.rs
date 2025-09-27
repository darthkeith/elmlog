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

