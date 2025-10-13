use ratatui::{
    style::Style,
    text::{Line, Span, Text}
};

use crate::{
    node::{
        Node,
        NodePosition,
        NodeType,
        PreOrderIter,
    },
    view::{
        Scroll,
        style,
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
    label_iter: PreOrderIter<'a>,
}

impl<'a> ForestIter<'a> {
    fn new(root: &'a Node) -> Self {
        ForestIter {
            prefix: Vec::new(),
            label_iter: root.iter(),
        }
    }
}

impl<'a> Iterator for ForestIter<'a> {
    type Item = (String, &'a str);

    fn next(&mut self) -> Option<Self::Item> {
        let (label, pos) = self.label_iter.next()?;
        let NodePosition { node_type, is_last } = pos;
        let mut tree_row = String::from(" ");
        match node_type {
            NodeType::Root => {
                self.prefix.clear();
                return Some((tree_row, label));
            }
            NodeType::Sibling => {
                while let Some(IndentBlock::Spacer) = self.prefix.pop() {}
            }
            NodeType::Child => (),
        }
        for block in &self.prefix {
            tree_row.push_str(match block {
                IndentBlock::Spacer => "   ",
                IndentBlock::VertBar => "│  ",
            });
        }
        if is_last {
            tree_row.push_str("└──");
            self.prefix.push(IndentBlock::Spacer);
        } else {
            tree_row.push_str("├──");
            self.prefix.push(IndentBlock::VertBar);
        }
        Some((tree_row, label))
    }
}

// Return the forest widget at `index` with the given styles.
fn forest(
    root: &Node,
    index: usize,
    text_style: Style,
    tree_style: Style,
) -> Scroll {
    let lines = ForestIter::new(root)
        .enumerate()
        .map(|(i, (tree_row, label))| {
            let spans = if i == index {
                vec![
                    Span::styled(tree_row, tree_style),
                    Span::styled(format!("{label} "), text_style),
                ]
            } else {
                vec![
                    Span::styled(tree_row, style::TREE),
                    Span::raw(label),
                ]
            };
            Line::from(spans)
        });
    Scroll {
        text: Text::from_iter(lines),
        list_size: root.size(),
        index,
    }
}

/// Return the forest widget in Normal mode.
pub fn forest_normal(root: &Node, index: usize) -> Scroll {
    forest(root, index, style::DEFAULT_BOLD, style::TREE)
}

/// Return the forest widget while editing.
pub fn forest_edit(root: &Node, index: usize) -> Scroll {
    forest(root, index, style::DEFAULT_HL, style::TREE_HL)
}

/// Return the forest widget while confirming a deletion.
pub fn forest_delete(root: &Node, index: usize) -> Scroll {
    forest(root, index, style::DELETE, style::TREE_DELETE)
}

