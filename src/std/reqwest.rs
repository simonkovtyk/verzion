use std::time::Duration;

use crate::{http::get_retry_policy, webhooks::config::WebhookItemConfig};
use reqwest::{ClientBuilder};
use reqwest_middleware::{ClientBuilder as MiddlewareClientBuilder, ClientWithMiddleware};
use reqwest_retry::RetryTransientMiddleware;

pub trait FromWebhookItemConfig {
  fn from_webhook_item_config (value: &WebhookItemConfig) -> ClientWithMiddleware;
}

impl FromWebhookItemConfig for ClientWithMiddleware {
  fn from_webhook_item_config (value: &WebhookItemConfig) -> ClientWithMiddleware {
    let mut client_builder = ClientBuilder::new();

    if let Some(value) = value.http_timeout {
      client_builder = client_builder.timeout(Duration::from_millis(value));
    }

    return MiddlewareClientBuilder::new(
      client_builder.build().expect("Could not get client")
    )
      .with(
        RetryTransientMiddleware::new_with_policy(
          get_retry_policy(
            value.http_retries
          )
        )
      ).build();
  }
}
