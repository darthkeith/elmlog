pub mod fs;

use std::io::Result;

use crate::{
    model::{
        ConfirmState,
        FileEntry,
        FilenameAction,
        FilenameState,
        FilenameStatus,
        LoadState,
        Model,
        PostSaveAction,
        SessionState,
    },
};

/// A message indicating an IO action to perform.
pub enum Command {
    None(Model),
    Load,
    InitSession(FileEntry),
    CheckFileExists(FilenameState),
    RenameFile(String, LoadState),
    SaveNew(String, SessionState, PostSaveAction),
    Save(SessionState, PostSaveAction),
    DeleteFile(LoadState),
    Quit,
}

/// Rename the selected file in the LoadState to `filename`.
pub fn rename_selected_file(
    load_state: &mut LoadState,
    filename: &str
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

/// Delete the currently selected file and remove it from the list.
///
/// Return None if there are no files left.
pub fn delete_selected_file(mut load_state: LoadState) -> Option<LoadState> {
    let entry = load_state.files.remove(load_state.index);
    std::fs::remove_file(entry.path)
        .expect("Failed to delete file");
    if load_state.files.is_empty() {
        return None;
    }
    if load_state.index == load_state.files.len() {
        load_state.index -= 1;
    }
    Some(load_state)
}

/// Execute `command` and return the updated Model.
pub fn execute_command(command: Command) -> Option<Model> {
    let model = match command {
        Command::None(model) => model,
        Command::Load => match fs::get_load_state() {
            Some(load_state) => Model::Load(load_state),
            None => Model::Confirm(ConfirmState::NewSession),
        }
        Command::InitSession(file_entry) => fs::init_model(file_entry),
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
            } else if rename_selected_file(&mut load_state, &filename).is_err() {
                FilenameStatus::Invalid
            } else {
                return Some(Model::Load(load_state))
            };
            let filename_state = FilenameState {
                input: filename,
                status,
                action: FilenameAction::Rename(load_state),
            };
            Model::FilenameInput(filename_state)
        }
        Command::SaveNew(filename, session, post_save) => {
            let status = if fs::filename_exists(&filename) {
                FilenameStatus::Exists
            } else if fs::save_new(&session.focus, &filename).is_err() {
                FilenameStatus::Invalid
            } else {
                return match post_save {
                    PostSaveAction::Load => execute_command(Command::Load),
                    PostSaveAction::Quit => None,
                }
            };
            let filename_state = FilenameState {
                input: filename,
                status,
                action: FilenameAction::SaveNew { session, post_save },
            };
            Model::FilenameInput(filename_state)
        }
        Command::Save(state, action) => {
            fs::save(state);
            return match action {
                PostSaveAction::Load => execute_command(Command::Load),
                PostSaveAction::Quit => None,
            }
        }
        Command::DeleteFile(load_state) => match delete_selected_file(load_state) {
            Some(load_state) => Model::Load(load_state),
            None => Model::Confirm(ConfirmState::NewSession),
        }
        Command::Quit => return None,
    };
    Some(model)
}
