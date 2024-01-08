use crate::{api, Data};
use std::time::Duration;

use color_eyre::eyre::Result;
use log::*;
use poise::serenity_prelude::{Context, Message};
use tokio::time::sleep;

const PK_DELAY_SEC: Duration = Duration::from_secs(1000);

pub async fn is_message_proxied(message: &Message) -> Result<bool> {
	debug!(
		"Waiting on PluralKit API for {} seconds",
		PK_DELAY_SEC.as_secs()
	);
	sleep(PK_DELAY_SEC).await;

	let proxied = api::pluralkit::get_sender(message.id).await.is_ok();

	Ok(proxied)
}

pub async fn handle(_ctx: &Context, msg: &Message, data: &Data) -> Result<()> {
	if msg.webhook_id.is_some() {
		debug!(
			"Message {} has a webhook ID. Checking if it was sent through PluralKit",
			msg.id
		);

		debug!(
			"Waiting on PluralKit API for {} seconds",
			PK_DELAY_SEC.as_secs()
		);
		sleep(PK_DELAY_SEC).await;

		if let Ok(sender) = api::pluralkit::get_sender(msg.id).await {
			data.storage.store_user_plurality(sender).await?;
		}
	}

	Ok(())
}
