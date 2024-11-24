use std::{
    fs::{self, File, OpenOptions},
    io::Read,
    path::{Path, PathBuf},
};

use fs2::FileExt;

use crate::{
    heap::Heap,
    model::SessionState,
};

const APP_DIR: &str = "sieve-selector";

/// The `name` and `path` of a file.
struct FileEntry {
    name: String,
    path: PathBuf,
}

/// List of `files` in the app directory and `index` of the current selection.
pub struct LoadState {
    files: Vec<FileEntry>,
    index: usize,
}

impl LoadState {
    /// Return the path at the current `index`.
    pub fn get_path(&self) -> PathBuf {
        self.files[self.index].path.clone()
    }

    /// Decrement the `index`.
    pub fn decrement(self) -> Self {
        if self.index == 0 {
            self
        } else {
            LoadState {
                index: self.index - 1,
                ..self
            }
        }
    }

    /// Increment the `index`.
    pub fn increment(self) -> Self {
        if self.index + 1 == self.files.len() {
            self
        } else {
            LoadState {
                index: self.index + 1,
                ..self
            }
        }
    }

    /// Iterator of file names with a boolean for whether the file is selected.
    pub fn get_file_names(&self) -> impl Iterator<Item = (&str, bool)> {
        self.files
            .iter()
            .map(|f| f.name.as_str())
            .enumerate()
            .map(|(i, name)| (name, i == self.index))
    }

    /// Return the total number of files.
    pub fn size(&self) -> usize {
        self.files.len()
    }
}

/// A file locked for exclusive data access.
///
/// The File is only stored to keep the lock active.
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

// Return the application directory path, creating any missing directories.
fn app_dir_path() -> PathBuf {
    let data_dir = dirs::data_dir()
        .expect("Failed to identify data directory");
    let path = data_dir.join(APP_DIR);
    fs::create_dir_all(&path)
        .expect("Failed to create data directory");
    path
}

/// Return the LoadState if there is a least one data file.
pub fn get_load_state() -> Option<LoadState> {
    let files: Vec<FileEntry> = fs::read_dir(app_dir_path())
        .expect("Unable to read app directory")
        .filter_map(Result::ok)
        .map(|entry| {
            let name = entry
                .file_name()
                .to_string_lossy()
                .into_owned();
            let path = entry.path();
            FileEntry { name, path }
        })
        .collect();
    match files.len() {
        0 => None,
        _ => Some(LoadState { files, index: 0 }),
    }
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

/// Initialize a session's state using the `path` to a data file.
pub fn init_session_state(path: PathBuf) -> SessionState {
    let file = OpenOptions::new()
        .read(true)
        .open(&path)
        .expect("Failed to open file");
    lock(&file);
    let heap = load_heap(&file);
    let open_file = OpenDataFile { path, _file: file, changed: false };
    SessionState { heap, maybe_file: Some(open_file) }
}

// Return the Heap and data file path (if present) from the session state.
// The File is dropped to unlock it.
fn unlock_state(state: SessionState) -> (Heap, Option<PathBuf>) {
    let SessionState { heap, maybe_file } = state;
    let maybe_path = maybe_file
        .map(|open_file| open_file.path);
    (heap, maybe_path)
}

// Set whether the file's permissions are read only.
fn set_read_only(path: &Path, read_only: bool) {
    let mut permissions = File::open(path)
        .expect("Failed to open file")
        .metadata()
        .expect("Failed to extract metadata")
        .permissions();
    permissions.set_readonly(read_only);
    fs::set_permissions(path, permissions)
        .expect("Failed to set file permissions");
}

// Write the `heap` to an existing file at the given `path`.
fn write_to_file(heap: Heap, path: &Path) {
    set_read_only(path, false);
    let file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(path)
        .expect("Failed to write to file");
    lock(&file);
    bincode::serialize_into(&file, &heap)
        .expect("Failed to serialize data");
    set_read_only(path, true);
}

/// Save the current session `state`.
pub fn save(state: SessionState) {
    let (heap, maybe_path) = unlock_state(state);
    if let Some(path) = maybe_path {
        write_to_file(heap, &path);
    }
}

/// Save the `heap` with the given `file_name`.
///
/// The file name is assumed to be unique and valid.
pub fn save_new(heap: Heap, file_name: String) {
    let path = app_dir_path().join(file_name);
    File::create_new(&path)
        .expect("Failed to create file");
    write_to_file(heap, &path);
}

