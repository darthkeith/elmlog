use ratatui::{
    style::Style,
    text::{Line, Span, Text}
};

use crate::{
    view::{
        Scroll,
        style,
    },
    zipper::{
        FocusNode,
        iter::{
            NodeInfo,
            NodePosition,
            focus_iter,
        },
    },
};

// Represents a text block used for tree drawing.
enum IndentBlock {
    Spacer,
    VertBar,
}

// Iterator type returning the strings used to display the forest.
struct ForestIter<'a> {
    prefix: Vec<IndentBlock>,
    node_iter: Box<dyn Iterator<Item = NodeInfo<'a>> + 'a>,
}

impl<'a> ForestIter<'a> {
    fn new(focus: Option<&'a FocusNode>) -> Self {
        let node_iter = focus
            .into_iter()
            .flat_map(focus_iter);
        ForestIter {
            prefix: Vec::new(),
            node_iter: Box::new(node_iter),
        }
    }
}

impl<'a> Iterator for ForestIter<'a> {
    type Item = (String, &'a str, bool);

    fn next(&mut self) -> Option<Self::Item> {
        let NodeInfo {
            label,
            position,
            is_last_sibling,
            is_focused,
        } = self.node_iter.next()?;
        let mut tree_row = String::from(" ");
        match position {
            NodePosition::Root => {
                self.prefix.clear();
                return Some((tree_row, label, is_focused));
            }
            NodePosition::FirstChild => (),
            NodePosition::SubsequentChild => {
                while let Some(IndentBlock::Spacer) = self.prefix.pop() {}
            }
        }
        for block in &self.prefix {
            tree_row.push_str(match block {
                IndentBlock::Spacer => "   ",
                IndentBlock::VertBar => "│  ",
            });
        }
        if is_last_sibling {
            tree_row.push_str("└──");
            self.prefix.push(IndentBlock::Spacer);
        } else {
            tree_row.push_str("├──");
            self.prefix.push(IndentBlock::VertBar);
        }
        Some((tree_row, label, is_focused))
    }
}

// Return a forest widget with the given styles.
// Display `label_override` text at focused node if present.
fn forest<'a>(
    focus: Option<&'a FocusNode>,
    label_override: Option<&'a str>,
    selected_text_style: Style,
    selected_tree_style: Style,
) -> Scroll<'a> {
    let mut focus_index = 0;
    let lines: Vec<_> = ForestIter::new(focus)
        .enumerate()
        .map(|(i, (tree_row, label, is_focused))| {
            let spans = if is_focused {
                focus_index = i;
                let tree_span = Span::styled(tree_row, selected_tree_style);
                let mut line_spans = vec![tree_span];
                match label_override {
                    Some(input) => {
                        line_spans.push(Span::styled(input, selected_text_style));
                        line_spans.push(Span::styled("█", style::CURSOR));
                    },
                    None => {
                        let label_span = Span::styled(
                            format!("{label} "),
                            selected_text_style,
                        );
                        line_spans.push(label_span);
                    },
                }
                line_spans
            } else {
                vec![
                    Span::styled(tree_row, style::TREE),
                    Span::raw(label),
                ]
            };
            Line::from(spans)
        })
        .collect();
    Scroll {
        list_size: lines.len(),
        text: Text::from_iter(lines),
        index: focus_index,
    }
}

/// Return a forest widget for Normal mode.
pub fn forest_normal(focus: Option<&FocusNode>) -> Scroll {
    forest(focus, None, style::DEFAULT_BOLD, style::TREE)
}

/// Return a forest widget for editing.
pub fn forest_edit(focus: Option<&FocusNode>) -> Scroll {
    forest(focus, None, style::DEFAULT_HL, style::TREE_HL)
}

/// Return a forest widget for confirming a deletion.
pub fn forest_delete(focus: Option<&FocusNode>) -> Scroll {
    forest(focus, None, style::DELETE, style::TREE_DELETE)
}

/// Return a forest widget with user `input` at the focused node.
pub fn forest_input<'a>(
    focus: Option<&'a FocusNode>,
    input: &'a str,
) -> Scroll<'a> {
    forest(focus, Some(input), style::DEFAULT_BOLD, style::TREE)
}

