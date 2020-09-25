use super::button_literals::*;
use super::string_formatter::*;
use super::data_callback_literals::*;
use tbot::types::keyboard::inline::{Button, ButtonKind};


pub fn join(lang: &Lang) -> Button<'static> {
    Button::new(Join::get(lang), ButtonKind::CallbackData(JOIN_BTN_DATA))
}

pub fn leave(lang: &Lang) -> Button<'static> {
    Button::new(Leave::get(lang), ButtonKind::CallbackData(LEAVE_BTN_DATA))
}

pub fn start(lang: &Lang) -> Button<'static> {
    Button::new(Start::get(lang), ButtonKind::CallbackData(START_BTN_DATA))
}