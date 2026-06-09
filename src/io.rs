pub mod fs;

use std::{io::Result, path::Path};

use crate::{
    model::{
        ConfirmState, FileEntry, FilenameAction, FilenameState, FilenameStatus,
        ForestState, LoadState, Model, OpenDataFile, SessionState,
    },
    zipper::FocusNode,
};

/// A message indicating an IO action to perform.
pub enum Command {
    None(Model),
    Load { quit: bool },
    InitSession(FileEntry),
    CheckFileExists(FilenameState),
    RenameFile(String, LoadState),
    SaveNew(Option<FocusNode>, String, SessionState),
    Save(SessionState),
    DeleteFile(LoadState),
    Quit,
}

// Rename the selected file in the LoadState to `filename`.
fn rename_selected_file(
    load_state: &mut LoadState,
    filename: &str,
) -> Result<()> {
    let i = load_state.index;
    let old_path = &load_state.files[i].path;
    let new_path = fs::rename_file(old_path, filename)?;
    load_state.files[i] = FileEntry {
        name: filename.to_string(),
        path: new_path,
    };
    Ok(())
}

// Delete the currently selected file and remove it from the list.
// Return None if there are no files left.
fn delete_selected_file(mut load_state: LoadState) -> Option<LoadState> {
    let entry = load_state.files.remove(load_state.index);
    std::fs::remove_file(entry.path).expect("Failed to delete file");
    if load_state.files.is_empty() {
        return None;
    }
    if load_state.index == load_state.files.len() {
        load_state.index -= 1;
    }
    Some(load_state)
}

// Return the LoadState if there is a least one data file.
fn get_load_state() -> Option<LoadState> {
    let files: Vec<_> = fs::scan_app_dir()
        .expect("Unable to read app directory")
        .into_iter()
        .map(|(name, path)| FileEntry { name, path })
        .collect();
    if files.is_empty() {
        return None;
    }
    Some(LoadState { files, index: 0 })
}

// Initialize a session from a saved file.
fn init_session(file_entry: FileEntry) -> SessionState {
    let FileEntry { name, path } = file_entry;
    let file = fs::open_read_locked(&path);
    let focus = bincode::deserialize(&fs::read_all_bytes(&file))
        .expect("Failed to deserialize data");
    let forest = ForestState {
        focus,
        changed: false,
    };
    let open_file = OpenDataFile {
        name,
        path,
        _file: file,
    };
    SessionState {
        forest,
        undo_stack: Vec::new(),
        redo_stack: Vec::new(),
        maybe_file: Some(open_file),
    }
}

// Write the forest to an existing file at `path`.
fn write_to_file(focus: &Option<FocusNode>, path: &Path) {
    fs::set_read_only(path, false);
    let file = fs::open_write_locked(path);
    bincode::serialize_into(&file, focus).expect("Failed to serialize data");
    fs::set_read_only(path, true);
}

// Save the current session.
fn save(state: SessionState) {
    let (focus, maybe_path) = state.unlock_state();
    if let Some(path) = maybe_path {
        write_to_file(&focus, &path);
    }
}

// Save the forest to `filename`.
fn save_new(focus: &Option<FocusNode>, filename: &str) -> Result<()> {
    let path = fs::create_new_file(filename)?;
    write_to_file(focus, &path);
    Ok(())
}

/// Execute `command` and return the updated Model.
pub fn execute_command(command: Command) -> Option<Model> {
    let model = match command {
        Command::None(model) => model,
        Command::Load { quit } => match get_load_state() {
            Some(load_state) => Model::Load(load_state),
            None => {
                if quit {
                    return None;
                } else {
                    Model::Confirm(ConfirmState::NewSession)
                }
            }
        },
        Command::InitSession(file_entry) => {
            Model::Normal(init_session(file_entry))
        }
        Command::CheckFileExists(filename_state) => {
            let status = if fs::filename_exists(filename_state.trimmed()) {
                FilenameStatus::Exists
            } else {
                FilenameStatus::Valid
            };
            Model::FilenameInput(filename_state.set_status(status))
        }
        Command::RenameFile(filename, mut load_state) => {
            let status = if fs::filename_exists(&filename) {
                FilenameStatus::Exists
            } else if rename_selected_file(&mut load_state, &filename).is_err()
            {
                FilenameStatus::Invalid
            } else {
                return Some(Model::Load(load_state));
            };
            let filename_state = FilenameState {
                input: filename,
                status,
                action: FilenameAction::Rename(load_state),
            };
            Model::FilenameInput(filename_state)
        }
        Command::SaveNew(initial_focus, filename, session) => {
            let status = if fs::filename_exists(&filename) {
                FilenameStatus::Exists
            } else if save_new(&initial_focus, &filename).is_err() {
                FilenameStatus::Invalid
            } else {
                return execute_command(Command::Load { quit: true });
            };
            let filename_state = FilenameState {
                input: filename,
                status,
                action: FilenameAction::SaveNew(session),
            };
            Model::FilenameInput(filename_state)
        }
        Command::Save(state) => {
            save(state);
            return execute_command(Command::Load { quit: true });
        }
        Command::DeleteFile(load_state) => {
            match delete_selected_file(load_state) {
                Some(load_state) => Model::Load(load_state),
                None => Model::Confirm(ConfirmState::NewSession),
            }
        }
        Command::Quit => return None,
    };
    Some(model)
}
