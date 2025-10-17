use crate::{
    io::LoadState,
    message::{
        Command,
        EditMsg,
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
        Mode,
        Model,
        PostSaveAction,
        SaveState,
        SessionState,
    },
    zipper::{FocusNode, FocusNodeExt},
};

// Update the Model based on a Confirm mode message.
fn update_confirm(
    confirm: bool,
    confirm_state: ConfirmState,
    mut state: SessionState
) -> Command {
    let mode = match confirm {
        true => match confirm_state {
            ConfirmState::NewSession => Mode::Normal,
            ConfirmState::DeleteItem => {
                state = state.delete();
                if state.focus.is_none() { Mode::Normal } else { Mode::Edit }
            }
            ConfirmState::DeleteFile(load_state) => {
                return Command::DeleteFile(load_state);
            }
        }
        false => match confirm_state {
            ConfirmState::NewSession => Mode::Confirm(ConfirmState::NewSession),
            ConfirmState::DeleteItem => Mode::Edit,
            ConfirmState::DeleteFile(load_state) => Mode::Load(load_state),
        }
    };
    Command::None(Model { state, mode })
}

// Update the Model based on a Load mode message.
fn update_load(
    msg: LoadMsg,
    load_state: LoadState,
    state: SessionState
) -> Command {
    let mode = match msg {
        LoadMsg::Decrement => Mode::Load(load_state.decrement()),
        LoadMsg::Increment => Mode::Load(load_state.increment()),
        LoadMsg::Open => {
            let file_entry = load_state.move_file_entry();
            return Command::InitSession(file_entry);
        }
        LoadMsg::New => Mode::Normal,
        LoadMsg::Rename => {
            Mode::FilenameInput(FilenameState::new_rename(load_state))
        }
        LoadMsg::Delete => Mode::Confirm(ConfirmState::DeleteFile(load_state)),
        LoadMsg::Quit => return Command::Quit,
    };
    Command::None(Model { state, mode })
}

// Update the Model based on a Normal mode message.
fn update_normal(msg: NormalMsg, mut state: SessionState) -> Command {
    let mode = match msg {
        NormalMsg::Ascend => {
            state.focus = state.focus.focus_parent();
            Mode::Normal
        }
        NormalMsg::Descend => {
            state.focus = state.focus.focus_child();
            Mode::Normal
        }
        NormalMsg::Previous => {
            state.focus = state.focus.focus_prev();
            Mode::Normal
        }
        NormalMsg::Next => {
            state.focus = state.focus.focus_next();
            Mode::Normal
        }
        NormalMsg::Insert => if state.focus.is_none() {
            state.focus = Some(FocusNode::new());
            Mode::LabelInput(LabelState::new_insert())
        } else {
            Mode::Normal
        }
        NormalMsg::Edit => {
            if state.focus.is_some() { Mode::Edit } else { Mode::Normal }
        }
        NormalMsg::Load => match state.is_changed() {
            true => Mode::Save(SaveState::new_load()),
            false => return Command::Load,
        }
        NormalMsg::Quit => match state.is_changed() {
            true => Mode::Save(SaveState::new_quit()),
            false => return Command::Quit,
        }
    };
    Command::None(Model { state, mode })
}

// Update the Model based on a Label Input mode message.
fn update_label_input(
    msg: LabelMsg,
    label_state: LabelState,
    state: SessionState,
) -> Model {
    let label_state = match msg {
        LabelMsg::Edit(edit) => match edit {
            InputEdit::Append(c) => label_state.append(c),
            InputEdit::PopChar => label_state.pop(),
        }
        LabelMsg::Submit => match label_state.is_empty() {
            true => label_state,
            false => {
                let label = label_state.input.trim().to_string();
                return Model {
                    state: state.set_label(label),
                    mode: Mode::Edit,
                };
            }
        }
        LabelMsg::Cancel => return match label_state.action {
            LabelAction::Insert => {
                let state = state.delete();
                let mode = if state.focus.is_none() {
                    Mode::Normal
                } else {
                    Mode::Edit
                };
                Model { state, mode }
            }
            LabelAction::Rename => {
                Model { state, mode: Mode::Edit }
            }
        }
    };
    let mode = Mode::LabelInput(label_state);
    Model { state, mode }
}

// Update the Model based on a Filename Input mode message.
fn update_filename_input(
    msg: FilenameMsg,
    filename_state: FilenameState,
    state: SessionState,
) -> Command {
    let filename_state = match msg {
        FilenameMsg::Edit(edit) => {
            let filename_state = match edit {
                InputEdit::Append(c) => filename_state.append(c),
                InputEdit::PopChar => filename_state.pop(),
            };
            match filename_state.is_empty() {
                true => filename_state.set_status(FilenameStatus::Empty),
                false => return Command::CheckFileExists(state, filename_state),
            }
        }
        FilenameMsg::Submit => match filename_state.is_empty() {
            true => filename_state.set_status(FilenameStatus::Empty),
            false => {
                let filename = filename_state.trimmed().to_string();
                match filename_state.action {
                    FilenameAction::Rename(load_state) => {
                        return Command::Rename(state, filename, load_state);
                    }
                    FilenameAction::SaveNew(post_save) => {
                        return Command::SaveNew(state, filename, post_save);
                    }
                }
            }
        }
        FilenameMsg::Cancel => {
            let mode = match filename_state.action {
                FilenameAction::Rename(load_state) => Mode::Load(load_state),
                FilenameAction::SaveNew(_) => Mode::Normal,
            };
            return Command::None(Model { state, mode });
        }
    };
    let mode = Mode::FilenameInput(filename_state);
    Command::None(Model { state, mode })
}

// Update the Model based on an Edit mode message.
fn update_edit(msg: EditMsg, mut state: SessionState) -> Model {
    let mode = match msg {
        EditMsg::Ascend => {
            state.focus = state.focus.focus_parent();
            Mode::Edit
        }
        EditMsg::Descend => {
            state.focus = state.focus.focus_child();
            Mode::Edit
        }
        EditMsg::Previous => {
            state.focus = state.focus.focus_prev();
            Mode::Edit
        }
        EditMsg::Next => {
            state.focus = state.focus.focus_next();
            Mode::Edit
        }
        EditMsg::Rename => match state.focus.clone_label() {
            Some(label) => Mode::LabelInput(LabelState::new_rename(label)),
            None => Mode::Edit,
        }
        EditMsg::Move => Mode::Move,
        EditMsg::Nest => {
            state = state.nest();
            Mode::Edit
        }
        EditMsg::Flatten => {
            state = state.flatten();
            Mode::Edit
        }
        EditMsg::Insert => Mode::Insert,
        EditMsg::Delete => Mode::Confirm(ConfirmState::DeleteItem),
        EditMsg::Back => Mode::Normal,
    };
    Model { state, mode }
}

// Update the Model based on a Move mode message.
fn update_move(msg: MoveMsg, state: SessionState) -> Model {
    let (state, mode) = match msg {
        MoveMsg::Promote => (state.promote(), Mode::Move),
        MoveMsg::Demote => (state.demote(), Mode::Move),
        MoveMsg::Backward => (state.swap_prev(), Mode::Move),
        MoveMsg::Forward => (state.swap_next(), Mode::Move),
        MoveMsg::Done => (state, Mode::Edit),
    };
    Model { state, mode }
}

// Update the Model based on an Insert mode message.
fn update_insert(msg: InsertMsg, mut state: SessionState) -> Model {
    state.focus = match msg {
        InsertMsg::Parent => state.focus.insert_parent(),
        InsertMsg::Child => state.focus.insert_child(),
        InsertMsg::Before => state.focus.insert_prev(),
        InsertMsg::After => state.focus.insert_next(),
        InsertMsg::Back => return Model { state, mode: Mode::Edit },
    };
    let mode = Mode::LabelInput(LabelState::new_insert());
    Model { state, mode }
}

// Update the Model based on a Save mode message.
fn update_save(
    msg: SaveMsg,
    save_state: SaveState,
    state: SessionState,
) -> Command {
    let mode = match msg {
        SaveMsg::Toggle => Mode::Save(save_state.toggle()),
        SaveMsg::Confirm => {
            let SaveState { save, post_save } = save_state;
            match save {
                true => match &state.maybe_file {
                    Some(_) => return Command::Save(state, post_save),
                    None => {
                        Mode::FilenameInput(FilenameState::new_save(post_save))
                    }
                }
                false => match post_save {
                    PostSaveAction::Load => return Command::Load,
                    PostSaveAction::Quit => return Command::Quit,
                }
            }
        }
        SaveMsg::Cancel => Mode::Normal,
    };
    Command::None(Model { state, mode })
}

/// Update the Model based on `message` and return an IO Command.
pub fn update(message: Message, state: SessionState) -> Command {
    let model = match message {
        Message::Confirm(confirm, confirm_state) => {
            return update_confirm(confirm, confirm_state, state);
        }
        Message::Load(msg, load_state) => {
            return update_load(msg, load_state, state);
        }
        Message::Normal(msg) => return update_normal(msg, state),
        Message::LabelInput(msg, label_state) => {
            update_label_input(msg, label_state, state)
        }
        Message::FilenameInput(msg, filename_state) => {
            return update_filename_input(msg, filename_state, state);
        }
        Message::Edit(msg) => update_edit(msg, state),
        Message::Move(msg) => update_move(msg, state),
        Message::Insert(msg) => update_insert(msg, state),
        Message::Save(msg, save_state) => {
            return update_save(msg, save_state, state);
        }
        Message::Continue(mode) => Model { state, mode },
    };
    Command::None(model)
}

