use teloxide::prelude::*;
use crate::dialogue::{states::ReceiveUrlState, Dialogue};

#[derive(Clone, Generic)]
pub struct StartState;

#[teloxide(subtransition)]
async fn start(
    state: StartState,
    cx: TransitionIn<AutoSend<Bot>>,
    ans: String,
) -> TransitionOut<Dialogue> {
    cx.answer("Please send the b23.tv short url to convert\n`https://b23.tv/exampleShortUrl`").await?;
    next(ReceiveUrlState::up(state, ans))
}