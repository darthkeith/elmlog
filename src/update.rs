use crate::{
    io::{Command, LoadState},
    message::{
        ConfirmMsg,
        FilenameMsg,
        InputEdit,
        InsertMsg,
        LabelMsg,
        LoadMsg,
        Message,
        MoveMsg,
        NormalMsg,
        SaveMsg,
    },
    model::{
        ConfirmState,
        FilenameAction,
        FilenameState,
        FilenameStatus,
        LabelAction,
        LabelState,
        Model,
        PostSaveAction,
        SaveState,
        SessionState,
    },
    zipper::FocusNode,
};

// Update the Model based on a Load mode message.
fn update_load(msg: LoadMsg, load_state: LoadState) -> Command {
    let model = match msg {
        LoadMsg::Decrement => Model::Load(load_state.decrement()),
        LoadMsg::Increment => Model::Load(load_state.increment()),
        LoadMsg::Open => {
            let file_entry = load_state.move_file_entry();
            return Command::InitSession(file_entry);
        }
        LoadMsg::New => Model::Normal(SessionState::new()),
        LoadMsg::Rename =>
            Model::FilenameInput(FilenameState::new_rename(load_state)),
        LoadMsg::Delete =>
            Model::Confirm(ConfirmState::DeleteFile(load_state)),
        LoadMsg::Quit => return Command::Quit,
    };
    Command::None(model)
}

// Update the Model based on a Normal mode message.
fn update_normal(msg: NormalMsg, state: SessionState) -> Command {
    let model = match msg {
        NormalMsg::Ascend => Model::Normal(state.focus_parent()),
        NormalMsg::Descend => Model::Normal(state.focus_child()),
        NormalMsg::Previous => Model::Normal(state.focus_prev()),
        NormalMsg::Next => Model::Normal(state.focus_next()),
        NormalMsg::Rename => match state.clone_label() {
            Some(label) =>
                Model::LabelInput(LabelState::new_rename(label, state)),
            None => Model::Normal(state),
        }
        NormalMsg::Insert => if state.focus.is_some() {
            Model::Insert(state)
        } else {
            let state = SessionState {focus: Some(FocusNode::new()), ..state };
            Model::LabelInput(LabelState::new_insert(state))
        }
        NormalMsg::Move => Model::Move(state),
        NormalMsg::Nest => Model::Normal(state.nest()),
        NormalMsg::Flatten => Model::Normal(state.flatten()),
        NormalMsg::Delete => if state.focus.is_some() {
            Model::Confirm(ConfirmState::DeleteItem(state))
        } else {
            Model::Normal(state)
        }
        NormalMsg::Load => if state.changed {
            Model::Save(SaveState::new_load(state))
        } else {
            return Command::Load
        }
        NormalMsg::Quit => if state.changed {
            Model::Save(SaveState::new_quit(state))
        } else {
            return Command::Quit
        }
    };
    Command::None(model)
}

// Update the Model based on an Insert mode message.
fn update_insert(msg: InsertMsg, state: SessionState) -> Model {
    let state = match msg {
        InsertMsg::Parent => state.insert_parent(),
        InsertMsg::Child => state.insert_child(),
        InsertMsg::Before => state.insert_prev(),
        InsertMsg::After => state.insert_next(),
        InsertMsg::Back => return Model::Normal(state),
    };
    Model::LabelInput(LabelState::new_insert(state))
}

// Update the Model based on a Move mode message.
fn update_move(msg: MoveMsg, state: SessionState) -> Model {
    let state = match msg {
        MoveMsg::Promote => state.promote(),
        MoveMsg::Demote => state.demote(),
        MoveMsg::Backward => state.swap_prev(),
        MoveMsg::Forward => state.swap_next(),
        MoveMsg::Done => return Model::Normal(state),
    };
    Model::Move(state)
}

// Update the Model based on a Save mode message.
fn update_save(msg: SaveMsg, save_state: SaveState) -> Command {
    let model = match msg {
        SaveMsg::Toggle => Model::Save(save_state.toggle()),
        SaveMsg::Confirm => {
            let SaveState { save, post_save, session } = save_state;
            if save {
                if session.maybe_file.is_some() {
                    return Command::Save(session, post_save);
                } else {
                    let state = FilenameState::new_save(session, post_save);
                    Model::FilenameInput(state)
                }
            } else {
                match post_save {
                    PostSaveAction::Load => return Command::Load,
                    PostSaveAction::Quit => return Command::Quit,
                }
            }
        }
        SaveMsg::Cancel => Model::Normal(save_state.session),
    };
    Command::None(model)
}

// Update the Model based on a Label Input mode message.
fn update_label_input(msg: LabelMsg, label_state: LabelState) -> Model {
    match msg {
        LabelMsg::Edit(edit) => {
            let label_state = match edit {
                InputEdit::Append(c) => label_state.append(c),
                InputEdit::PopChar => label_state.pop(),
            };
            Model::LabelInput(label_state)
        }
        LabelMsg::Submit => if label_state.input.is_empty() {
            Model::LabelInput(label_state)
        } else {
            Model::Normal(label_state.set_label())
        }
        LabelMsg::Cancel => {
            let state = match label_state.action {
                LabelAction::Insert => label_state.session.delete(false),
                LabelAction::Rename => label_state.session,
            };
            Model::Normal(state)
        }
    }
}

// Update the Model based on a Filename Input mode message.
fn update_filename_input(
    msg: FilenameMsg,
    filename_state: FilenameState,
) -> Command {
    let filename_state = match msg {
        FilenameMsg::Edit(edit) => {
            let filename_state = match edit {
                InputEdit::Append(c) => filename_state.append(c),
                InputEdit::PopChar => filename_state.pop(),
            };
            if filename_state.input.is_empty() {
                filename_state.set_status(FilenameStatus::Empty)
            } else {
                return Command::CheckFileExists(filename_state)
            }
        }
        FilenameMsg::Submit => if filename_state.input.is_empty() {
            filename_state.set_status(FilenameStatus::Empty)
        } else {
            let filename = filename_state.trimmed().to_string();
            return match filename_state.action {
                FilenameAction::Rename(load_state) =>
                    Command::RenameFile(filename, load_state),
                FilenameAction::SaveNew { session, post_save } =>
                    Command::SaveNew(filename, session, post_save),
            }
        }
        FilenameMsg::Cancel => {
            let model = match filename_state.action {
                FilenameAction::Rename(load_state) =>
                    Model::Load(load_state),
                FilenameAction::SaveNew { session, .. } =>
                    Model::Normal(session),
            };
            return Command::None(model);
        }
    };
    Command::None(Model::FilenameInput(filename_state))
}

// Update the Model based on a Confirm mode message.
fn update_confirm(msg: ConfirmMsg, confirm_state: ConfirmState) -> Command {
    let model = match msg {
        ConfirmMsg::Confirm => match confirm_state {
            ConfirmState::NewSession => Model::Normal(SessionState::new()),
            ConfirmState::DeleteItem(state) =>
                Model::Normal(state.delete(true)),
            ConfirmState::DeleteFile(load_state) =>
                return Command::DeleteFile(load_state),
        }
        ConfirmMsg::Cancel => match confirm_state {
            ConfirmState::NewSession =>
                Model::Confirm(ConfirmState::NewSession),
            ConfirmState::DeleteItem(state) => Model::Normal(state),
            ConfirmState::DeleteFile(load_state) => Model::Load(load_state),
        }
    };
    Command::None(model)
}

/// Update the Model based on the `message` and return an IO Command.
pub fn update(message: Message) -> Command {
    let model = match message {
        Message::Load(load_msg, load_state) =>
            return update_load(load_msg, load_state),
        Message::Normal(normal_msg, session_state) =>
            return update_normal(normal_msg, session_state),
        Message::Insert(insert_msg, session_state) =>
            update_insert(insert_msg, session_state),
        Message::Move(move_msg, session_state) =>
            update_move(move_msg, session_state),
        Message::Save(save_msg, save_state) =>
            return update_save(save_msg, save_state),
        Message::LabelInput(label_msg, label_state) =>
            update_label_input(label_msg, label_state),
        Message::FilenameInput(filename_msg, filename_state) =>
            return update_filename_input(filename_msg, filename_state),
        Message::Confirm(confirm_msg, confirm_state) => {
            return update_confirm(confirm_msg, confirm_state);
        }
        Message::Continue(model) => model,
    };
    Command::None(model)
}

