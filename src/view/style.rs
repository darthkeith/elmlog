use ratatui::style::{
    Color,
    Modifier,
    Style,
};

const BG: Color = Color::Rgb(64, 58, 55);
const BG2: Color = Color::Rgb(89, 81, 71);
const FG: Color = Color::Rgb(250, 238, 202);

pub const DEFAULT: Style = Style::new().fg(FG).bg(BG);
pub const ACCENT: Style = Style::new().fg(FG).bg(BG2);
pub const HIGHLIGHT: Style = Style::new().fg(BG).bg(FG);
pub const CMD_KEY: Style = HIGHLIGHT.add_modifier(Modifier::BOLD);
pub const CMD_NAME: Style = DEFAULT.add_modifier(Modifier::ITALIC);
pub const CURSOR: Style = ACCENT.add_modifier(Modifier::SLOW_BLINK);

