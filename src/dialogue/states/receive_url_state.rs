use crate::dialogue::Dialogue;
use teloxide::prelude::*;
use crate::network::get_aid;


#[derive(Clone, Generic)]
pub struct ReceiveUrlState {
    pub command: String,
}

#[teloxide(subtransition)]
async fn receive_url_state(
    _state: ReceiveUrlState,
    cx: TransitionIn<AutoSend<Bot>>,
    ans: String,
) -> TransitionOut<Dialogue> {
    log::info!("received url requesr: {}", ans);

    if ans.contains("b23.tv") {
        let result = get_aid(&ans).unwrap();

        cx.answer(format!("{}", result)).await?;
    } else {
        cx.answer("Url missing match").await?;
    }

    exit()
}
