use super::buttons::*;
use super::string_formatter::Lang;
use tbot::types::keyboard::inline::Button;

pub fn game_lobby_keyboard(can_start: bool) -> Vec<Vec<Button<'static>>> {
    if !can_start {
        vec![vec![join(&Lang::Pt), leave(&Lang::Pt)]]
    } else {
        vec![vec![join(&Lang::Pt), leave(&Lang::Pt)],
             vec![start(&Lang::Pt)]]
    }
}
