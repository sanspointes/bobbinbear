pub mod input;

use bevy::prelude::*;

use self::frontend::FrontendMsg;

pub mod frontend;

#[derive(Event)]
pub enum Msg {
    Frontend(FrontendMsg),
}
