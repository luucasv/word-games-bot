#[macro_use]
mod helper_macros {
    macro_rules! to_slice {
        ($keyboard:ident) => {
            let $keyboard: Vec<_> = $keyboard.iter().map(Vec::as_slice).collect();
            let $keyboard: &[&[_]] = $keyboard.as_slice();
        };
    }
}

mod command;
mod data_callback;
mod helper;
mod new_game;
mod start;
mod util;

// make this trait public so main can use its associated functions
pub use command::Command;

// making commands public
pub use new_game::NewGameCommand;
pub use start::StartCommand;

pub use data_callback::DataCallback;
