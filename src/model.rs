use crate::heap::Heap;

/// Represents the operational modes of the application.
pub enum Mode {
    Normal,
    Input(String),
    Select(usize),
    Compare,
}

/// Type for storing all application state.
pub struct Model {
    pub heap: Heap,
    pub mode: Mode,
}

impl Model {
    /// Construct a model storing the initial application state.
    pub fn new() -> Self {
        Model {
            heap: Heap::Empty,
            mode: Mode::Normal,
        }
    }
}

