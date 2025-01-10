use crate::{
    heap::HeapStatus,
    io::LoadState,
    message::{
        Command,
        CompareMsg,
        InputEdit,
        InputMsg,
        LoadMsg,
        Message,
        NormalMsg,
        SaveMsg,
        SelectedMsg,
        SelectMsg,
    },
    model::{
        CompareState,
        ConfirmState,
        FilenameAction,
        FilenameState,
        FilenameStatus,
        InputState,
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
        LoadMsg::Rename => Mode::Input(InputState::new_rename(load_state)),
        LoadMsg::Delete => Mode::Confirm(ConfirmState::DeleteFile(load_state)),
        LoadMsg::Quit => return Command::Quit,
    };
    Command::None(Model { state, mode })
}

// Update the Model based on a Normal mode message.
fn update_normal(msg: NormalMsg, state: SessionState) -> Command {
    let mode = match msg {
        NormalMsg::Input => Mode::Input(InputState::new_add()),
        NormalMsg::Select => match state.heap.size() > 0 {
            true => Mode::Select(0),
            false => Mode::Normal,
        }
        NormalMsg::Compare => {
            match state.heap.status() {
                HeapStatus::MultiRoot(item1, item2) => {
                    Mode::Compare(CompareState::new(item1, item2))
                }
                _ => Mode::Normal,
            }
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
                let state = match action {
                    LabelAction::Add => state.add(label),
                    LabelAction::Edit(index) => state.edit(index, label),
                };
                let model = Model { state, mode: Mode::Normal };
                return Command::None(model);
            }
        }
        InputMsg::Cancel => {
            let model = Model { state, mode: Mode::Normal };
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

// Update the Model based on a Select mode message.
fn update_select(msg: SelectMsg, index: usize, state: SessionState) -> Command {
    let mode = match msg {
        SelectMsg::Append(c) => {
            let i = util::append_index(index, c, state.heap.size());
            Mode::Select(i)
        }
        SelectMsg::Decrement => {
            match index > 0 {
                true => Mode::Select(index - 1),
                false => Mode::Select(index),
            }
        }
        SelectMsg::Increment => {
            match index + 1 < state.heap.size() {
                true => Mode::Select(index + 1),
                false => Mode::Select(index),
            }
        }
        SelectMsg::Confirm => Mode::Selected(index),
    };
    Command::None(Model { state, mode })
}

// Update the Model based on a Selected mode message.
fn update_selected(
    msg: SelectedMsg,
    index: usize,
    mut state: SessionState,
) -> Command {
    let label = state.heap.label_at(index).to_string();
    let mode = match msg {
        SelectedMsg::Edit => {
            Mode::Input(InputState::new_edit(label, index))
        }
        SelectedMsg::Delete => {
            Mode::Confirm(ConfirmState::DeleteItem(label, index))
        }
    };
    Command::None(Model { state, mode })
}

// Update the Model based on a Compare mode message.
fn update_compare(
    msg: CompareMsg,
    compare_state: CompareState,
    mut state: SessionState,
) -> Command {
    let mode = match msg {
        CompareMsg::Toggle => {
            Mode::Compare(compare_state.toggle())
        }
        CompareMsg::Confirm => {
            state = state.merge_pair(compare_state.first);
            Mode::Normal
        }
    };
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
                    None => Mode::Input(InputState::new_save(post_save))
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
        Message::Select(msg, index) => update_select(msg, index, state),
        Message::Selected(msg, index) => update_selected(msg, index, state),
        Message::Compare(msg, compare_state) => {
            update_compare(msg, compare_state, state)
        }
        Message::Save(msg, save_state) => update_save(msg, save_state, state),
        Message::Continue(mode) => Command::None(Model { state, mode }),
    }
}

