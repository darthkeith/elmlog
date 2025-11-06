use ratatui::style::{
    Color,
    Modifier,
    Style,
};

const WARM_GRAY: Color = Color::Rgb(64, 58, 55);
const LIGHT_WARM_GRAY: Color = Color::Rgb(89, 81, 71);
const DARK_WARM_GRAY: Color = Color::Rgb(55, 50, 47);
const DARKER_WARM_GRAY: Color = Color::Rgb(44, 40, 38);
const COOL_GRAY: Color = Color::Rgb(109, 109, 115);
const IVORY: Color = Color::Rgb(250, 238, 202);
const DARK_IVORY: Color = Color::Rgb(248, 232, 180);
const RED: Color = Color::Rgb(171, 26, 10);
const AMBER: Color = Color::Rgb(160, 110, 30);
const GREEN: Color = Color::Rgb(130, 150, 70);

pub const TEXT_TREE: Style = Style::new().fg(COOL_GRAY);
pub const TEXT_DEFAULT: Style = Style::new().fg(IVORY);
pub const TEXT_SELECTED: Style = Style::new().fg(Color::White)
    .add_modifier(Modifier::BOLD);
pub const BG_DEFAULT: Style = Style::new().bg(WARM_GRAY);
pub const BG_INSERT: Style = Style::new().bg(GREEN);
pub const BG_MOVE: Style = Style::new().bg(AMBER);
pub const BG_INPUT: Style = Style::new().bg(DARK_WARM_GRAY);
pub const BG_DELETE: Style = Style::new().bg(RED);

pub const DEFAULT: Style = Style::new().fg(IVORY).bg(WARM_GRAY);
pub const DEFAULT_HL: Style = Style::new().fg(DARKER_WARM_GRAY).bg(IVORY);
pub const DELETE: Style = Style::new().fg(Color::White).bg(RED)
    .add_modifier(Modifier::BOLD);
pub const ACCENT: Style = Style::new().fg(IVORY).bg(LIGHT_WARM_GRAY);
pub const CURSOR: Style = DEFAULT.add_modifier(Modifier::SLOW_BLINK);
pub const CMD_KEY: Style = Style::new().fg(DARKER_WARM_GRAY).bg(DARK_IVORY)
    .add_modifier(Modifier::BOLD);
pub const CMD_NAME: Style = ACCENT.add_modifier(Modifier::ITALIC);

