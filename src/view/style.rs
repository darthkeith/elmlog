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
const GRAY: Color = Color::Rgb(109, 109, 115);
const ORANGE: Color = Color::Rgb(255, 195, 74);
const ORANGE_DARK: Color = Color::Rgb(82, 54, 0);
const GREEN: Color = Color::Rgb(214, 217, 61);
const GREEN_DARK: Color = Color::Rgb(88, 89, 0);

pub const DEFAULT: Style = Style::new().fg(FG).bg(BG);
pub const DEFAULT_HL: Style = Style::new().fg(BG_DARK).bg(FG);
pub const ACCENT: Style = Style::new().fg(FG).bg(BG2);
pub const NUMBER: Style = Style::new().fg(ORANGE).bg(BG2)
    .add_modifier(Modifier::BOLD);
pub const TREE: Style = Style::new().fg(GRAY).bg(BG);
pub const TREE_HL: Style = Style::new().fg(GRAY).bg(FG);
pub const SINGLE_ROOT: Style = Style::new().fg(GREEN).bg(BG)
    .add_modifier(Modifier::BOLD);
pub const SINGLE_ROOT_HL: Style = Style::new().fg(GREEN_DARK).bg(FG)
    .add_modifier(Modifier::BOLD);
pub const ROOT: Style = Style::new().fg(ORANGE).bg(BG)
    .add_modifier(Modifier::BOLD);
pub const ROOT_HL: Style = Style::new().fg(ORANGE_DARK).bg(FG)
    .add_modifier(Modifier::BOLD);
pub const CURSOR: Style = DEFAULT.add_modifier(Modifier::SLOW_BLINK);
pub const CMD_KEY: Style = Style::new().fg(BG_DARK).bg(FG_DARK)
    .add_modifier(Modifier::BOLD);
pub const CMD_NAME: Style = DEFAULT.add_modifier(Modifier::ITALIC);

