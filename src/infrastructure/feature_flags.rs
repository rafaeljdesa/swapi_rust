use std::time::Duration;

use launchdarkly_server_sdk::{Client, ConfigBuilder, ContextBuilder};
use mockall::automock;

#[automock]
pub trait FeatureFlag: Send + Sync {
    fn is_forcing_api_call(&self) -> bool; 
}

pub struct FeatureFlagLaunchDarkly {
    client: Client,
}

impl FeatureFlagLaunchDarkly {
    pub async fn new() -> Self {
        let sdk_key =
            std::env::var("LAUNCHDARKLY_SDK_KEY").expect("LAUNCHDARKLY_SDK_KEY env should be set");

        let config = ConfigBuilder::new(&sdk_key)
            .build()
            .expect("Config failed to build");

        let client = Client::build(config).expect("Client failed to build");

        // Starts the client using the currently active runtime.
        client.start_with_default_executor();

        // Wait for the client to be ready.
        client
            .wait_for_initialization(Duration::from_secs(5))
            .await
            .expect(
                "*** SDK failed to initialize within the timeout period. 
                Please check your internet connection and SDK credential for any typo.",
            );

        println!("*** SDK successfully initialized.");

        FeatureFlagLaunchDarkly { client }
    }
}

impl FeatureFlag for FeatureFlagLaunchDarkly {
    fn is_forcing_api_call(&self) -> bool {
        let context = ContextBuilder::new("example-user-key")
            .kind("user")
            .name("Sandy")
            .build()
            .expect("Context failed to build");

        self.client
            .bool_variation(&context, "forcing-api-call", false)
    }
}
