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
const FG_LIGHT: Color = Color::Rgb(253, 246, 228);
const GRAY: Color = Color::Rgb(109, 109, 115);
const RED: Color = Color::Rgb(171, 26, 10);
const WHITE: Color = Color::Rgb(255, 255, 255);
const AMBER: Color = Color::Rgb(160, 110, 30);
const GREEN: Color = Color::Rgb(130, 150, 70);

pub const DEFAULT: Style = Style::new().fg(FG).bg(BG);
pub const DEFAULT_HL: Style = Style::new().fg(BG_DARK).bg(FG);
pub const DEFAULT_BOLD: Style = Style::new().fg(FG_LIGHT).bg(BG)
    .add_modifier(Modifier::BOLD);
pub const DELETE: Style = Style::new().fg(WHITE).bg(RED)
    .add_modifier(Modifier::BOLD);
pub const TREE_DELETE: Style = Style::new().fg(GRAY).bg(RED);
pub const MOVE_TEXT: Style = Style::new().fg(WHITE).bg(AMBER)
    .add_modifier(Modifier::BOLD);
pub const MOVE_TREE: Style = Style::new().fg(GRAY).bg(AMBER);
pub const INSERT_TEXT: Style = Style::new().fg(WHITE).bg(GREEN)
    .add_modifier(Modifier::BOLD);
pub const INSERT_TREE: Style = Style::new().fg(GRAY).bg(GREEN);
pub const ACCENT: Style = Style::new().fg(FG).bg(BG2);
pub const TREE: Style = Style::new().fg(GRAY).bg(BG);
pub const TREE_HL: Style = Style::new().fg(GRAY).bg(FG);
pub const CURSOR: Style = DEFAULT.add_modifier(Modifier::SLOW_BLINK);
pub const CMD_KEY: Style = Style::new().fg(BG_DARK).bg(FG_DARK)
    .add_modifier(Modifier::BOLD);
pub const CMD_NAME: Style = ACCENT.add_modifier(Modifier::ITALIC);

