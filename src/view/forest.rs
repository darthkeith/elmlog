use std::{borrow::Cow, collections::VecDeque};

use ratatui::text::{Line, Span, Text};

use crate::{
    view::{
        INDENT,
        scroll::{ScrollArea, ScrollContent},
        style,
    },
    zipper::{
        FocusNode,
        iter::{NodeInfo, NodePosition, focus_iter},
    },
};

// Represents a text block used for tree drawing.
enum IndentBlock {
    Spacer,
    VertBar,
}

// Data used to render a single forest line in the TUI.
struct LineContent<'a> {
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
        let node_iter = focus.into_iter().flat_map(focus_iter);
        ForestIter {
            prefix_stack: Vec::new(),
            node_iter: Box::new(node_iter),
        }
    }
}

impl<'a> Iterator for ForestIter<'a> {
    type Item = LineContent<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let NodeInfo {
            label,
            position,
            is_last_sibling,
            is_focused,
        } = self.node_iter.next()?;
        let mut tree_prefix = String::new();
        match position {
            NodePosition::Root => {
                self.prefix_stack.clear();
                return Some(LineContent {
                    tree_prefix,
                    label,
                    is_focused,
                });
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
        Some(LineContent {
            tree_prefix,
            label,
            is_focused,
        })
    }
}

// Content of the visible window, plus flags for if there is more to scroll.
struct ForestWindow<'a> {
    items: Vec<LineContent<'a>>,
    more_above: bool,
    more_below: bool,
}

impl ForestWindow<'_> {
    fn empty() -> Self {
        Self {
            items: Vec::new(),
            more_above: false,
            more_below: false,
        }
    }
}

// Build a window of visible lines centering the focused line.
fn build_forest_window(
    mut iter: ForestIter,
    window_height: usize,
) -> ForestWindow {
    if window_height == 0 {
        return ForestWindow::empty();
    }
    let mut line_queue: VecDeque<LineContent> =
        VecDeque::with_capacity(window_height);
    let mut more_above = false;
    for tree_line in iter.by_ref() {
        if line_queue.len() == window_height {
            line_queue.pop_front();
            more_above = true;
        }
        if tree_line.is_focused {
            line_queue.push_back(tree_line);
            break;
        }
        line_queue.push_back(tree_line);
    }
    if line_queue.is_empty() {
        return ForestWindow::empty();
    }
    let mut focus_idx = line_queue.len() - 1;
    let center_idx = window_height / 2;
    let mut more_below = false;
    for tree_line in iter {
        if line_queue.len() < window_height {
            line_queue.push_back(tree_line);
        } else if focus_idx <= center_idx {
            more_below = true;
            break;
        } else {
            line_queue.pop_front();
            line_queue.push_back(tree_line);
            more_above = true;
            focus_idx -= 1;
        }
    }
    ForestWindow {
        items: line_queue.into_iter().collect(),
        more_above,
        more_below,
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

// Construct a styled UI Line from its content.
fn format_line<'a>(item: LineContent<'a>, style: &FocusStyle<'a>) -> Line<'a> {
    let LineContent {
        tree_prefix,
        label,
        is_focused,
    } = item;
    let prefix_span = Span::styled(tree_prefix, style::TEXT_TREE);
    let (text, text_style, bg_style) = if is_focused {
        let (text, bg_style) = match style {
            FocusStyle::Normal => (Cow::Borrowed(label), style::BG_DEFAULT),
            FocusStyle::Insert => (Cow::Borrowed(label), style::BG_INSERT),
            FocusStyle::Move => (Cow::Borrowed(label), style::BG_MOVE),
            FocusStyle::Input(input) => {
                (Cow::Owned(format!("{input}█")), style::BG_INPUT)
            }
            FocusStyle::Delete => (Cow::Borrowed(label), style::BG_DELETE),
        };
        (text, style::TEXT_SELECTED, bg_style)
    } else {
        (Cow::Borrowed(label), style::TEXT_DEFAULT, style::BG_DEFAULT)
    };
    let label_span = Span::styled(text, text_style);
    let spans = vec![Span::raw(INDENT), prefix_span, label_span];
    Line::from(spans).style(bg_style)
}

// Construct a ScrollArea to display the forest, styling the focused line.
fn new_scroll_area<'a>(
    focus: Option<&'a FocusNode>,
    style: FocusStyle<'a>,
) -> ScrollArea<'a, impl FnOnce(usize) -> ScrollContent<'a> + 'a> {
    let build = move |height| {
        let ForestWindow {
            items,
            more_above,
            more_below,
        } = build_forest_window(ForestIter::new(focus), height);
        let lines = items.into_iter().map(|item| format_line(item, &style));
        ScrollContent {
            text: Text::from_iter(lines),
            more_above,
            more_below,
        }
    };
    ScrollArea { build }
}

/// Return a ScrollArea widget for Normal mode.
pub fn normal<'a>(
    focus: Option<&'a FocusNode>,
) -> ScrollArea<'a, impl FnOnce(usize) -> ScrollContent<'a> + 'a> {
    new_scroll_area(focus, FocusStyle::Normal)
}

/// Return a ScrollArea widget for selecting an insert position.
pub fn insert<'a>(
    focus: Option<&'a FocusNode>,
) -> ScrollArea<'a, impl FnOnce(usize) -> ScrollContent<'a> + 'a> {
    new_scroll_area(focus, FocusStyle::Insert)
}

/// Return a ScrollArea widget for Move mode.
pub fn move_mode<'a>(
    focus: Option<&'a FocusNode>,
) -> ScrollArea<'a, impl FnOnce(usize) -> ScrollContent<'a> + 'a> {
    new_scroll_area(focus, FocusStyle::Move)
}

/// Return a ScrollArea widget with user `input` on the focused line.
pub fn input<'a>(
    focus: Option<&'a FocusNode>,
    input: &'a str,
) -> ScrollArea<'a, impl FnOnce(usize) -> ScrollContent<'a> + 'a> {
    new_scroll_area(focus, FocusStyle::Input(input))
}

/// Return a ScrollArea widget for confirming a deletion.
pub fn delete<'a>(
    focus: Option<&'a FocusNode>,
) -> ScrollArea<'a, impl FnOnce(usize) -> ScrollContent<'a> + 'a> {
    new_scroll_area(focus, FocusStyle::Delete)
}
