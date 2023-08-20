use serenity::{async_trait, client::EventHandler, model::gateway::Ready, prelude::Context};
use tracing::log::info;

pub struct EvHandler;

#[async_trait]
impl EventHandler for EvHandler {
    async fn ready(&self, _: Context, ready: Ready) {
        info!(
            "{username}({id})",
            username = ready.user.name,
            id = ready.user.id
        );
    }
}
