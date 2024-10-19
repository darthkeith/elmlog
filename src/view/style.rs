use ratatui::style::{
    Color,
    Modifier,
    Style,
};

const BG: Color = Color::Rgb(64, 58, 55);
const BG_DARK: Color = Color::Rgb(44, 40, 38);
const BG2: Color = Color::Rgb(89, 81, 71);
const FG: Color = Color::Rgb(250, 238, 202);
const FG_DARK: Color = Color::Rgb(248, 232, 180);

pub const DEFAULT: Style = Style::new().fg(FG).bg(BG);
pub const ACCENT: Style = Style::new().fg(FG).bg(BG2);
pub const HIGHLIGHT: Style = Style::new().fg(BG).bg(FG);
pub const CURSOR: Style = DEFAULT.add_modifier(Modifier::SLOW_BLINK);
pub const CMD_KEY: Style = Style::new().fg(BG_DARK).bg(FG_DARK)
    .add_modifier(Modifier::BOLD);
pub const CMD_NAME: Style = DEFAULT.add_modifier(Modifier::ITALIC);

