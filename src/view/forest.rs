use std::collections::VecDeque;

use ratatui::{
    prelude::{Buffer, Rect, Widget},
    text::{Line, Span, Text},
    widgets::Block,
};

use crate::{
    view::{
        style,
        top_mid_bottom,
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

// Data needed to render a single tree line in the TUI.
struct TreeLine<'a> {
    tree_prefix: String,
    label: &'a str,
    is_focused: bool,
}

// Iterator type returning the strings used to display the forest.
struct ForestIter<'a> {
    prefix_stack: Vec<IndentBlock>,
    node_iter: Box<dyn Iterator<Item = NodeInfo<'a>> + 'a>,
}

impl<'a> ForestIter<'a> {
    fn new(focus: Option<&'a FocusNode>) -> Self {
        let node_iter = focus
            .into_iter()
            .flat_map(focus_iter);
        ForestIter {
            prefix_stack: Vec::new(),
            node_iter: Box::new(node_iter),
        }
    }
}

impl<'a> Iterator for ForestIter<'a> {
    type Item = TreeLine<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let NodeInfo {
            label,
            position,
            is_last_sibling,
            is_focused,
        } = self.node_iter.next()?;
        let mut tree_prefix = String::from("  ");  // Left padding
        match position {
            NodePosition::Root => {
                self.prefix_stack.clear();
                return Some(TreeLine { tree_prefix, label, is_focused });
            }
            NodePosition::FirstChild => (),
            NodePosition::SubsequentChild => {
                while let Some(IndentBlock::Spacer) = self.prefix_stack.pop() {}
            }
        }
        for block in &self.prefix_stack {
            tree_prefix.push_str(match block {
                IndentBlock::Spacer => "   ",
                IndentBlock::VertBar => "│  ",
            });
        }
        if is_last_sibling {
            tree_prefix.push_str("└──");
            self.prefix_stack.push(IndentBlock::Spacer);
        } else {
            tree_prefix.push_str("├──");
            self.prefix_stack.push(IndentBlock::VertBar);
        }
        Some(TreeLine { tree_prefix, label, is_focused })
    }
}

// TreeLine data for the visible window, plus flags for more above/below.
struct ScrollWindow<'a> {
    lines: Vec<TreeLine<'a>>,
    has_more_above: bool,
    has_more_below: bool,
}

impl ScrollWindow<'_> {
    fn empty() -> Self {
        ScrollWindow {
            lines: Vec::new(),
            has_more_above: false,
            has_more_below: false,
        }
    }
}

// Build a window of visible lines centering the focused line.
fn scroll_window(mut iter: ForestIter, window_height: usize) -> ScrollWindow {
    if window_height == 0 {
        return ScrollWindow::empty();
    }
    let mut line_queue: VecDeque<TreeLine> =
        VecDeque::with_capacity(window_height);
    let mut has_more_above = false;
    while let Some(tree_line) = iter.next() {
        if line_queue.len() == window_height {
            line_queue.pop_front();
            has_more_above = true;
        }
        if tree_line.is_focused {
            line_queue.push_back(tree_line);
            break;
        }
        line_queue.push_back(tree_line);
    }
    if line_queue.is_empty() {
        return ScrollWindow::empty();
    }
    let mut focus_idx = line_queue.len() - 1;
    let center_idx = window_height / 2;
    let mut has_more_below = false;
    while let Some(tree_line) = iter.next() {
        if line_queue.len() < window_height {
            line_queue.push_back(tree_line);
        } else if focus_idx <= center_idx {
            has_more_below = true;
            break;
        } else {
            line_queue.pop_front();
            line_queue.push_back(tree_line);
            has_more_above = true;
            focus_idx -= 1;
        }
    }
    ScrollWindow {
        lines: line_queue.into_iter().collect(),
        has_more_above,
        has_more_below,
    }
}

// Indicates which style to apply to the focused line.
enum FocusStyle<'a> {
    Normal,
    Insert,
    Move,
    Input(&'a str),
    Delete,
}

// Convert TreeLines into styled Text based on focus style.
fn lines_to_text<'a>(
    lines: Vec<TreeLine<'a>>,
    style: FocusStyle<'a>,
) -> Text<'a> {
    let lines = lines.into_iter().map(|TreeLine { tree_prefix, label, is_focused }| {
        let mut spans = vec![Span::styled(tree_prefix, style::TEXT_TREE)];
        if is_focused {
            match style {
                FocusStyle::Normal => {
                    spans.push(Span::styled(label, style::TEXT_SELECTED));
                    Line::from(spans).style(style::BG_DEFAULT)
                }
                FocusStyle::Insert => {
                    spans.push(Span::styled(label, style::TEXT_SELECTED));
                    Line::from(spans).style(style::BG_INSERT)
                }
                FocusStyle::Move => {
                    spans.push(Span::styled(label, style::TEXT_SELECTED));
                    Line::from(spans).style(style::BG_MOVE)
                }
                FocusStyle::Input(input) => {
                    let text = format!("{input}█");
                    spans.push(Span::styled(text, style::TEXT_SELECTED));
                    Line::from(spans).style(style::BG_INPUT)
                }
                FocusStyle::Delete => {
                    spans.push(Span::styled(label, style::TEXT_SELECTED));
                    Line::from(spans).style(style::BG_DELETE)
                }
            }
        } else {
            spans.push(Span::styled(label, style::TEXT_DEFAULT));
            Line::from(spans).style(style::BG_DEFAULT)
        }
    });
    Text::from_iter(lines)
}

// Widget for displaying a forest view centering the focused line.
pub struct ForestScroll<'a> {
    iter: ForestIter<'a>,
    style: FocusStyle<'a>,
}

impl<'a> ForestScroll<'a> {
    fn new(focus: Option<&'a FocusNode>, style: FocusStyle<'a>) -> Self {
        Self {
            iter: ForestIter::new(focus),
            style,
        }
    }
}

impl<'a> Widget for ForestScroll<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [top_line, mid_area, bottom_line] = top_mid_bottom(area);
        let ScrollWindow {
            lines,
            has_more_above,
            has_more_below,
        } = scroll_window(self.iter, mid_area.height as usize);
        Block::new().style(style::BG_DEFAULT)
            .render(mid_area, buf);
        lines_to_text(lines, self.style)
            .render(mid_area, buf);
        let scroll_hint = |has_more: bool| if has_more { " ..." } else { "" };
        Text::from(scroll_hint(has_more_above))
            .style(style::DEFAULT)
            .render(top_line, buf);
        Text::from(scroll_hint(has_more_below))
            .style(style::DEFAULT)
            .render(bottom_line, buf);
    }
}

/// Return a ForestScroll widget for Normal mode.
pub fn normal(focus: Option<&FocusNode>) -> ForestScroll {
    ForestScroll::new(focus, FocusStyle::Normal)
}

/// Return a ForestScroll widget for selecting an insert position.
pub fn insert(focus: Option<&FocusNode>) -> ForestScroll {
    ForestScroll::new(focus, FocusStyle::Insert)
}

/// Return a ForestScroll widget for Move mode.
pub fn move_mode(focus: Option<&FocusNode>) -> ForestScroll {
    ForestScroll::new(focus, FocusStyle::Move)
}

/// Return a ForestScroll widget with user `input` on the focused line.
pub fn input<'a>(
    focus: Option<&'a FocusNode>,
    input: &'a str,
) -> ForestScroll<'a> {
    ForestScroll::new(focus, FocusStyle::Input(input))
}

/// Return a ForestScroll widget for confirming a deletion.
pub fn delete(focus: Option<&FocusNode>) -> ForestScroll {
    ForestScroll::new(focus, FocusStyle::Delete)
}

