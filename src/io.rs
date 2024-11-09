use std::{
    fs::{self, File, OpenOptions},
    io::Read,
    path::PathBuf,
};

use fs2::FileExt;

use crate::{
    heap::Heap,
    model::SessionState,
};

const APP_DIR: &str = "sieve-selector";
const DATA_FILE: &str = "data.bin";

/// A file locked for exclusive data access.
///
/// The File is stored in the `_file` field only to keep the lock active.
pub struct OpenDataFile {
    path: PathBuf,
    _file: File,
    changed: bool,
}

impl OpenDataFile {
    pub fn set_changed(&mut self) {
        self.changed = true;
    }

    pub fn is_changed(&self) -> bool {
        self.changed
    }
}

// Return the application data file path, creating any missing directories.
fn data_file_path() -> PathBuf {
    let data_dir = dirs::data_dir()
        .expect("Failed to identify data directory");
    let path = data_dir.join(APP_DIR);
    fs::create_dir_all(&path)
        .expect("Failed to create data directory");
    path.join(DATA_FILE)
}

// Lock the `file` for exclusive data access.
fn lock(file: &File) {
    file.try_lock_exclusive()
        .expect("File is currently locked");
}

// Load a Heap from a serialized data `file`.
fn load_heap(mut file: &File) -> Heap {
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)
        .expect("Failed to read file");
    bincode::deserialize(&buffer)
        .expect("Failed to deserialize data")
}

/// Initialize the session state.
pub fn init_state() -> SessionState {
    let path = data_file_path();
    let file_found = path.exists();
    if !file_found {
        File::create(&path)
            .expect("Failed to create file");
    }
    let file = OpenOptions::new()
        .read(true)
        .open(&path)
        .expect("Failed to open file");
    lock(&file);
    let heap = match file_found {
        true => load_heap(&file),
        false => Heap::Empty,
    };
    let open_file = OpenDataFile { path, _file: file, changed: false };
    SessionState { heap, open_file }
}

// Return the Heap and data file path from the session state.
// The File is dropped to unlock it.
fn unlock_state(state: SessionState) -> (Heap, PathBuf) {
    let SessionState { heap, open_file } = state;
    let OpenDataFile { path, _file: _, .. } = open_file;
    (heap, path)
}

// Set whether the file's permissions are read only or not.
fn set_read_only(file_path: &PathBuf, read_only: bool) {
    let file = File::open(file_path)
        .expect("Failed to open file");
    let metadata = file.metadata()
        .expect("Failed to extract metadata");
    let mut permissions = metadata.permissions();
    permissions.set_readonly(read_only);
    fs::set_permissions(file_path, permissions)
        .expect("Failed to set file permissions");
}

/// Save the current session `state`.
pub fn save(state: SessionState) {
    let (heap, path) = unlock_state(state);
    set_read_only(&path, false);
    let file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(&path)
        .expect("Failed to write to file");
    lock(&file);
    bincode::serialize_into(&file, &heap)
        .expect("Failed to serialize data");
    set_read_only(&path, true);
}

