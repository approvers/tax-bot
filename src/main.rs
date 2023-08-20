use std::env;

use anyhow::{Context, Ok};
use client::discord::discord_client;
use dotenvy::dotenv;

mod client;
mod event;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().context(".env file not found")?;
    tracing_subscriber::fmt().compact().init();

    let token = get_env("DISCORD_API_TOKEN")?;

    discord_client(token.as_str()).await?;

    Ok(())
}

pub fn get_env(key: &str) -> anyhow::Result<String> {
    env::var(key).with_context(|| format!("{} is not found", key))
}
