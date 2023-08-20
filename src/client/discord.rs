use anyhow::Context;
use serenity::{prelude::GatewayIntents, Client};

use crate::event::EvHandler;

pub async fn discord_client(token: &str) -> anyhow::Result<()> {
    let intents = GatewayIntents::GUILDS;

    let mut client = Client::builder(token, intents)
        .event_handler(EvHandler)
        .await
        .context("クライアントの作成に失敗しました.")?;

    client
        .start()
        .await
        .context("クライアントの起動に失敗しました.")
}
