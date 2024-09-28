use crate::heap::{Heap, empty};

/// Represents the operational modes of the application.
pub enum Mode {
    Normal,
    Input(String),
}

/// Type for storing all application state.
pub struct Model {
    pub heap: Heap,
    pub mode: Mode,
    pub quit: bool,
}

impl Model {
    /// Construct a model storing the initial application state.
    pub fn new() -> Self {
        Model {
            heap: empty(),
            mode: Mode::Normal,
            quit: false,
        }
    }
}

