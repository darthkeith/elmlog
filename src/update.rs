use crate::{
    io::LoadState,
    message::{
        Command,
        EditMsg,
        InputEdit,
        InputMsg,
        InsertMsg,
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
        InputState,
        InsertPosition,
        LabelAction,
        LabelState,
        Mode,
        Model,
        PostSaveAction,
        SaveState,
        SessionState,
    },
    util,
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
            ConfirmState::DeleteItem(_, index) => {
                state = state.delete(index);
                Mode::Normal
            }
            ConfirmState::DeleteFile(load_state) => {
                return Command::DeleteFile(load_state);
            }
        }
        false => match confirm_state {
            ConfirmState::NewSession => Mode::Confirm(ConfirmState::NewSession),
            ConfirmState::DeleteItem(..) => Mode::Normal,
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
        LoadMsg::Append(c) => Mode::Load(load_state.append_index(c)),
        LoadMsg::Decrement => Mode::Load(load_state.decrement()),
        LoadMsg::Increment => Mode::Load(load_state.increment()),
        LoadMsg::Open => {
            let file_entry = load_state.move_file_entry();
            return Command::InitSession(file_entry);
        }
        LoadMsg::New => Mode::Normal,
        LoadMsg::Rename => Mode::Input(InputState::new_rename_file(load_state)),
        LoadMsg::Delete => Mode::Confirm(ConfirmState::DeleteFile(load_state)),
        LoadMsg::Quit => return Command::Quit,
    };
    Command::None(Model { state, mode })
}

// Update the Model based on a Normal mode message.
fn update_normal(msg: NormalMsg, state: SessionState) -> Command {
    let mode = match msg {
        NormalMsg::Insert => match state.root.size() {
            0 => Mode::Input(InputState::new_insert_empty()),
            _ => Mode::Normal,
        }
        NormalMsg::Edit => match state.root.size() {
            0 => Mode::Normal,
            _ => Mode::Edit(0),
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

// Update the Model based on an Input mode label editing message.
fn update_label(
    msg: InputMsg,
    label_state: LabelState,
    state: SessionState,
) -> Command {
    let label_state = match msg {
        InputMsg::Edit(edit) => match edit {
            InputEdit::Append(c) => label_state.append(c),
            InputEdit::PopChar => label_state.pop(),
        }
        InputMsg::Submit => match label_state.is_empty() {
            true => label_state,
            false => {
                let LabelState { input, action } = label_state;
                let label = input.trim().to_string();
                let model = match action {
                    LabelAction::Rename(index) => Model {
                        state: state.edit(index, label),
                        mode: Mode::Normal,
                    },
                    LabelAction::Insert(index, pos) => {
                        let (state, index) = state.insert(index, pos, label);
                        Model {
                            state,
                            mode: Mode::Edit(index),
                        }
                    }
                };
                return Command::None(model);
            }
        }
        InputMsg::Cancel => {
            let model = Model {
                state,
                mode: Mode::Normal,
            };
            return Command::None(model);
        }
    };
    let mode = label_state.into_mode();
    Command::None(Model { state, mode })
}

// Update the Model based on an Input mode filename editing message.
fn update_filename(
    msg: InputMsg,
    filename_state: FilenameState,
    state: SessionState,
) -> Command {
    let filename_state = match msg {
        InputMsg::Edit(edit) => {
            let filename_state = match edit {
                InputEdit::Append(c) => filename_state.append(c),
                InputEdit::PopChar => filename_state.pop(),
            };
            match filename_state.is_empty() {
                true => filename_state.status(FilenameStatus::Empty),
                false => return Command::CheckFileExists(state, filename_state),
            }
        }
        InputMsg::Submit => match filename_state.is_empty() {
            true => filename_state.status(FilenameStatus::Empty),
            false => {
                let filename = filename_state.input;
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
        InputMsg::Cancel => {
            let mode = match filename_state.action {
                FilenameAction::Rename(load_state) => Mode::Load(load_state),
                FilenameAction::SaveNew(_) => Mode::Normal,
            };
            return Command::None(Model { state, mode });
        }
    };
    let mode = filename_state.into_mode();
    Command::None(Model { state, mode })
}

// Update the Model based on an Edit mode message.
fn update_edit(
    msg: EditMsg,
    index: usize,
    mut state: SessionState,
) -> Command {
    let mode = match msg {
        EditMsg::Append(c) => {
            let i = util::append_index(index, c, state.root.size());
            Mode::Edit(i)
        }
        EditMsg::Decrement => match index > 0 {
            true => Mode::Edit(index - 1),
            false => Mode::Edit(index),
        }
        EditMsg::Increment => match index + 1 < state.root.size() {
            true => Mode::Edit(index + 1),
            false => Mode::Edit(index),
        }
        EditMsg::Rename => {
            let label = state.root.find_label(index);
            Mode::Input(InputState::new_rename_label(label, index))
        }
        EditMsg::Move => Mode::Move(index),
        EditMsg::Nest => {
            let (new_state, new_index) = state.nest(index);
            state = new_state;
            Mode::Edit(new_index)
        }
        EditMsg::Flatten => {
            let (new_state, new_index) = state.flatten(index);
            state = new_state;
            Mode::Edit(new_index)
        }
        EditMsg::Insert => Mode::Insert(index),
        EditMsg::Delete => {
            let label = state.root.find_label(index);
            Mode::Confirm(ConfirmState::DeleteItem(label, index))
        }
    };
    Command::None(Model { state, mode })
}

// Update the Model based on a Move mode message.
fn update_move(
    msg: MoveMsg,
    index: usize,
    state: SessionState,
) -> Command {
    let model = match msg {
        MoveMsg::Forward => {
            let (state, index) = state.move_forward(index);
            let mode = Mode::Move(index);
            Model { state, mode }
        }
        MoveMsg::Backward => {
            let (state, index) = state.move_backward(index);
            let mode = Mode::Move(index);
            Model { state, mode }
        }
        MoveMsg::Promote => {
            let (state, index) = state.promote(index);
            let mode = Mode::Move(index);
            Model { state, mode }
        }
        MoveMsg::Demote => {
            let (state, index) = state.demote(index);
            let mode = Mode::Move(index);
            Model { state, mode }
        }
        MoveMsg::Done => Model {
            state,
            mode: Mode::Edit(index),
        }
    };
    Command::None(model)
}

// Update the Model based on an Insert mode message.
fn update_insert(
    msg: InsertMsg,
    index: usize,
    state: SessionState,
) -> Command {
    let position = match msg {
        InsertMsg::Parent => InsertPosition::Parent,
        InsertMsg::Child => InsertPosition::Child,
        InsertMsg::Before => InsertPosition::Before,
        InsertMsg::After => InsertPosition::After,
    };
    let mode = Mode::Input(InputState::new_insert(index, position));
    Command::None(Model { state, mode })
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
                    None => Mode::Input(InputState::new_save(post_save)),
                }
                false => match post_save {
                    PostSaveAction::Load => return Command::Load,
                    PostSaveAction::Quit => return Command::Quit,
                }
            }
        }
    };
    Command::None(Model { state, mode })
}

/// Update the Model based on `message` and return an IO Command.
pub fn update(message: Message, state: SessionState) -> Command {
    match message {
        Message::Confirm(confirm, confirm_state) => {
            update_confirm(confirm, confirm_state, state)
        }
        Message::Load(msg, load_state) => update_load(msg, load_state, state),
        Message::Normal(msg) => update_normal(msg, state),
        Message::Input(msg, input_state) => match input_state {
            InputState::Label(label_state) => {
                update_label(msg, label_state, state)
            }
            InputState::Filename(filename_state) => {
                update_filename(msg, filename_state, state)
            }
        }
        Message::Edit(msg, index) => update_edit(msg, index, state),
        Message::Move(msg, index) => update_move(msg, index, state),
        Message::Insert(msg, index) => update_insert(msg, index, state),
        Message::Save(msg, save_state) => update_save(msg, save_state, state),
        Message::Continue(mode) => Command::None(Model { state, mode }),
    }
}

