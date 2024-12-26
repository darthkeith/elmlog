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
};

// Append a digit to `index` if valid, otherwise return a fallback value.
fn append_index(index: usize, c: char, heap_size: usize) -> usize {
    if !c.is_ascii_digit() {
        return index;
    }
    let idx_str = format!("{index}{c}");
    if let Ok(new_index) = idx_str.parse::<usize>() {
        if new_index < heap_size {
            return new_index;
        }
    }
    let c_val = (c as usize) - ('0' as usize);
    if c_val < heap_size {
        return c_val;
    }
    index
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
            let path = load_state.get_path();
            return Command::InitSession(path);
        }
        LoadMsg::New => Mode::Normal,
        LoadMsg::Delete => match load_state.delete() {
            Some(load_state) => Mode::Load(load_state),
            None => Mode::Normal,
        }
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
                HeapStatus::MultiRoot(item1, item2) => Mode::Compare(
                    CompareState {
                        item1: item1.to_string(),
                        item2: item2.to_string(),
                        first: true,
                    }
                ),
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
            false => return Command::SaveNew(state, filename_state),
        }
    };
    let mode = filename_state.into_mode();
    Command::None(Model { state, mode })
}

// Update the Model based on a Select mode message.
fn update_select(msg: SelectMsg, index: usize, state: SessionState) -> Command {
    let mode = match msg {
        SelectMsg::Append(c) => {
            let i = append_index(index, c, state.heap.size());
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
    let mode = match msg {
        SelectedMsg::Edit => {
            let label = state.heap.label_at(index).to_string();
            let input_state = InputState::new_edit(label, index);
            Mode::Input(input_state)
        }
        SelectedMsg::Delete => {
            state = state.delete(index);
            Mode::Normal
        }
    };
    Command::None(Model { state, mode })
}

// Update the Model based on a Compare mode message.
fn update_compare(
    msg: CompareMsg,
    cmp_state: CompareState,
    mut state: SessionState,
) -> Command {
    let CompareState { item1, item2, first } = cmp_state;
    let mode = match msg {
        CompareMsg::Toggle => {
            Mode::Compare(CompareState { item1, item2, first: !first })
        }
        CompareMsg::Confirm => {
            state = state.merge_pair(first);
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
                    None => {
                        let input_state = InputState::new_save(post_save);
                        Mode::Input(input_state)
                    }
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
        Message::Compare(msg, choice) => update_compare(msg, choice, state),
        Message::Save(msg, save_state) => update_save(msg, save_state, state),
        Message::Continue(mode) => Command::None(Model { state, mode }),
    }
}

