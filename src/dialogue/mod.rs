mod states;

use crate::dialogue::states::{
    ReceiveUrlState, StartState,
};

use derive_more::From;
use teloxide::macros::Transition;

#[derive(Transition, Clone, From)]
pub enum Dialogue {
    Start(StartState),
    ReceiveUrl(ReceiveUrlState),
}

impl Default for Dialogue {
    fn default() -> Self {
        Self::Start(StartState)
    }
}