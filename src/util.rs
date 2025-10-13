/// Return the maximum character length of a 0-indexed list's indices.
pub fn max_index_length(list_size: usize) -> usize {
    match list_size {
        0 => 0,
        n => (n - 1).to_string().len(),
    }
}

