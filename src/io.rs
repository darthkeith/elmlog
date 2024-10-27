use std::{
    fs::{self, File, OpenOptions},
    io::{Read, Result},
    path::PathBuf,
};

use fs2::FileExt;

use crate::{
    heap::Heap,
};

const APP_DIR: &str = "sieve-selector";
const DATA_FILE: &str = "data.bin";

/// Return application data file path, creating any missing directories.
pub fn data_file_path() -> Result<PathBuf> {
    let data_dir = dirs::data_dir()
        .expect("Failed to locate data directory");
    let path = data_dir.join(APP_DIR);
    fs::create_dir_all(&path)?;
    Ok(path.join(DATA_FILE))
}

fn lock(file: &File) {
    file.try_lock_exclusive()
        .expect("Application data file is currently locked");
}

/// Return the initialized heap with its associated data file.
pub fn init(file_path: &PathBuf) -> Result<(Heap, File)> {
    let file_not_found = !file_path.exists();
    if file_not_found {
        File::create(file_path)?;
    }
    let mut file = OpenOptions::new()
        .read(true)
        .open(file_path)?;
    lock(&file);
    if file_not_found {
        return Ok((Heap::Empty, file));
    }
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    let heap = bincode::deserialize(&buffer)
        .expect("Failed to deserialize data");
    Ok((heap, file))
}

fn set_writable(file_path: &PathBuf, writable: bool) -> Result<()> {
    let file = File::open(file_path)?;
    let metadata = file.metadata()?;
    let mut permissions = metadata.permissions();
    permissions.set_readonly(!writable);
    fs::set_permissions(file_path, permissions)
}

/// Save the `heap` to the file with given path.
pub fn save(heap: Heap, file_path: &PathBuf) -> Result<()> {
    set_writable(file_path, true)?;
    let file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(file_path)?;
    lock(&file);
    bincode::serialize_into(&file, &heap)
        .expect("Failed to serialize data");
    set_writable(file_path, false)
}

