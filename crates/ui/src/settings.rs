#![allow(dead_code)]

use ratatui::style::Color;

pub const BG: Color = Color::Rgb(24, 20, 18);
pub const SURFACE: Color = Color::Rgb(40, 33, 30);
pub const TEXT: Color = Color::Rgb(238, 220, 190);
pub const BORDER: Color = Color::Rgb(120, 85, 55);
pub const TITLE: Color = Color::Rgb(255, 140, 80);
pub const SELECT: Color = Color::Rgb(70, 45, 35);
pub const ACTIVE: Color = Color::Rgb(255, 190, 120);
pub const SUCCESS: Color = Color::Rgb(180, 220, 140);
pub const ERROR: Color = Color::Rgb(255, 120, 120);

pub const CONTACTS_HORIZONTAL_PERCENTAGE: u16 = 15;
pub const CHAT_AREA_HORIZONTAL_PERCENTAGE: u16 = 100 - CONTACTS_HORIZONTAL_PERCENTAGE;
pub const INPUT_BOX_VERTICAL_PERCENTAGE: u16 = 12;
pub const CHAT_AREA_VERTICAL_PERCENTAGE: u16 = 100 - INPUT_BOX_VERTICAL_PERCENTAGE;
