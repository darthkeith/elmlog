use ratatui::{
    text::{Line, Span, Text},
    widgets::Paragraph,
};

use crate::{
    forest::{
        Node,
        NodePosition,
        NodeType,
        PreOrderIter,
    },
    util,
    view::{
        style,
        main_paragraph,
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
        let mut tree_row = String::new();
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

/// Return the forest widget without indices.
pub fn forest_normal(root: &Node) -> Paragraph {
    let lines = ForestIter::new(root)
        .map(|(tree_row, label)| {
            Line::from(vec![
                Span::styled(tree_row, style::TREE),
                Span::raw(label),
            ])
        });
    main_paragraph(Text::from_iter(lines))
}

/// Return the forest widget with indices.
pub fn forest_indexed(root: &Node, current_idx: usize) -> Paragraph {
    let index_len = util::max_index_length(root.size());
    let lines = ForestIter::new(root)
        .enumerate()
        .map(|(i, (tree_row, label))| {
            let fmt_index = format!(" {i:>width$}   ", width = index_len);
            let highlight = i == current_idx;
            let spans = if highlight {
                vec![
                    Span::styled(fmt_index, style::DEFAULT_HL),
                    Span::styled(tree_row, style::TREE_HL),
                    Span::styled(format!("{label} "), style::DEFAULT_HL),
                ]
            } else {
                vec![
                    Span::raw(fmt_index),
                    Span::styled(tree_row, style::TREE),
                    Span::raw(label),
                ]
            };
            Line::from(spans)
        });
    main_paragraph(Text::from_iter(lines))
}

