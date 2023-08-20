# tax-bot

限界税納税時に通知・ロールの付与を行う Discord Bot.

## Features

tax-bot は以下の機能を持つ.

- **納税者** ロールを付与.
- 納税時に `#無法地帯` に通知する.

## Environment Variables

| キー | 説明 | 必須か |
| --- | --- | --- |
| `DISCORD_API_TOKEN` | Discord API のトークン | true |
| `TAX_ROLE_ID` | 納税者のロール ID | true |
| `NOTICE_CHANNEL_ID` | 通知対象(`#無法地帯`)のチャンネル ID | true |
