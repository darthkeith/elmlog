use ratatui::{
    style::Style,
    text::{Line, Span, Text},
    widgets::Paragraph,
};

use crate::{
    heap::{
        Heap,
        HeapStatus,
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

// Indicates what style to apply to a label.
enum LabelType {
    SingleRoot,
    Root,
    Child,
}

// Iterator type returning the strings used to display the forest.
struct ForestIter<'a> {
    prefix: Vec<IndentBlock>,
    label_iter: PreOrderIter<'a>,
    single_root: bool,
}

impl<'a> ForestIter<'a> {
    fn new(heap: &'a Heap) -> Self {
        ForestIter {
            prefix: Vec::new(),
            label_iter: heap.iter(),
            single_root: matches!(heap.status(), HeapStatus::SingleRoot),
        }
    }
}

impl<'a> Iterator for ForestIter<'a> {
    type Item = (String, &'a str, LabelType);

    fn next(&mut self) -> Option<Self::Item> {
        let (label, pos) = self.label_iter.next()?;
        let NodePosition { node_type, is_last } = pos;
        let mut tree_row = String::new();
        if let NodeType::Root = node_type {
            self.prefix.clear();
            let label_type = match self.single_root {
                true => LabelType::SingleRoot,
                false => LabelType::Root,
            };
            return Some((tree_row, label, label_type));
        }
        if let NodeType::Sibling = node_type {
            while let Some(IndentBlock::Spacer) = self.prefix.pop() {}
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
        Some((tree_row, label, LabelType::Child))
    }
}

// Return the style to apply to a label of given type with optional highlight.
fn get_label_style(label_type: LabelType, highlight: bool) -> Style {
    if highlight {
        match label_type {
            LabelType::SingleRoot => style::SINGLE_ROOT_HL,
            LabelType::Root => style::ROOT_HL,
            LabelType::Child => style::DEFAULT_HL,
        }
    } else {
        match label_type {
            LabelType::SingleRoot => style::SINGLE_ROOT,
            LabelType::Root => style::ROOT,
            LabelType::Child => style::DEFAULT,
        }
    }
}

/// Return the forest widget in normal mode.
pub fn forest_normal(heap: &Heap) -> Paragraph {
    let lines = ForestIter::new(heap)
        .map(|(tree_row, label, label_type)| {
            let label_style = get_label_style(label_type, false);
            Line::from(vec![
                Span::styled(tree_row, style::TREE),
                Span::styled(format!("{label} "), label_style),
            ])
        });
    main_paragraph(Text::from_iter(lines))
}

/// Return the forest widget in select mode.
pub fn forest_select(heap: &Heap, current_idx: usize) -> Paragraph {
    let index_len = util::max_index_length(heap.size());
    let lines = ForestIter::new(heap)
        .enumerate()
        .map(|(i, (tree_row, label, label_type))| {
            let fmt_index = format!(" {i:>width$}   ", width = index_len);
            let highlight = i == current_idx;
            let label_style = get_label_style(label_type, highlight);
            let tree_style = match highlight {
                true => style::TREE_HL,
                false => style::TREE,
            };
            Line::from(vec![
                Span::styled(fmt_index, label_style),
                Span::styled(tree_row, tree_style),
                Span::styled(format!("{label} "), label_style),
            ])
        });
    main_paragraph(Text::from_iter(lines))
}

/// Return the forest widget in selected mode.
pub fn forest_selected(heap: &Heap, current_idx: usize) -> Paragraph {
    let lines = ForestIter::new(heap)
        .enumerate()
        .map(|(i, (tree_row, label, label_type))| {
            let highlight = i == current_idx;
            let fmt_label = match highlight {
                true => format!(" {label} "),
                false => format!("{label} "),
            };
            let label_style = get_label_style(label_type, highlight);
            Line::from(vec![
                Span::styled(tree_row, style::TREE),
                Span::styled(fmt_label, label_style),
            ])
        });
    main_paragraph(Text::from_iter(lines))
}

