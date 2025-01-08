/// Append a digit to `index` if valid, otherwise return a fallback value.
pub fn append_index(index: usize, c: char, list_size: usize) -> usize {
    if !c.is_ascii_digit() {
        return index;
    }
    let idx_str = format!("{index}{c}");
    if let Ok(new_index) = idx_str.parse::<usize>() {
        if new_index < list_size {
            return new_index;
        }
    }
    let c_val = (c as usize) - ('0' as usize);
    if c_val < list_size {
        return c_val;
    }
    index
}

/// Return the maximum character length of a 0-indexed list's indices.
pub fn max_index_length(list_size: usize) -> usize {
    match list_size {
        0 => 0,
        n => (n - 1).to_string().len(),
    }
}

