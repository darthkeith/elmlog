use std::{
    fs::{self, File, OpenOptions},
    io::{Read, Result},
    path::{Path, PathBuf},
};

use fs2::FileExt;

const APP_DIR: &str = "elmlog";

// Return the application directory path, creating any missing directories.
fn app_dir_path() -> PathBuf {
    let data_dir = dirs::data_dir()
        .expect("Failed to identify data directory");
    let path = data_dir.join(APP_DIR);
    fs::create_dir_all(&path)
        .expect("Failed to create data directory");
    path
}

/// Rename a file and return its new path.
pub fn rename_file(old_path: &Path, filename: &str) -> Result<PathBuf> {
    let new_path = app_dir_path().join(filename);
    fs::rename(old_path, &new_path)?;
    Ok(new_path)
}

/// Return the filenames and paths of the files in the app directory.
pub fn scan_app_dir() -> Result<Vec<(String, PathBuf)>> {
    let files = fs::read_dir(app_dir_path())?
        .filter_map(Result::ok)
        .map(|entry| {
            let name = entry
                .file_name()
                .to_string_lossy()
                .into_owned();
            let path = entry.path();
            (name, path)
        })
        .collect();
    Ok(files)
}

// Lock the `file` for exclusive data access.
fn lock(file: &File) {
    file.try_lock_exclusive()
        .expect("File is currently locked");
}

/// Return a buffer containing all of the file's bytes.
pub fn read_all_bytes(mut file: &File) -> Vec<u8> {
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)
        .expect("Failed to read file");
    buffer
}

/// Open a file in read mode and lock it.
pub fn open_read_locked(path: &Path) -> File {
    let file = OpenOptions::new()
        .read(true)
        .open(path)
        .expect("Failed to open file");
    lock(&file);
    file
}

/// Open a file in write mode and lock it.
pub fn open_write_locked(path: &Path) -> File {
    let file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(path)
        .expect("Failed to write to file");
    lock(&file);
    file
}

/// Check whether `filename` exists in the app directory.
pub fn filename_exists(filename: &str) -> bool {
    let path = app_dir_path().join(filename);
    path.exists()
}

/// Set whether the file's permissions are read only.
pub fn set_read_only(path: &Path, read_only: bool) {
    let mut permissions = File::open(path)
        .expect("Failed to open file")
        .metadata()
        .expect("Failed to extract metadata")
        .permissions();
    permissions.set_readonly(read_only);
    fs::set_permissions(path, permissions)
        .expect("Failed to set file permissions");
}

/// Create a new file in the app directory and return its path.
pub fn create_new_file(filename: &str) -> Result<PathBuf> {
    let path = app_dir_path().join(filename);
    File::create_new(&path)?;
    Ok(path)
}
