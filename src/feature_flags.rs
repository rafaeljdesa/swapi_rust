use launchdarkly_server_sdk::{Client, ConfigBuilder, ContextBuilder};

pub struct FeatureFlagManager {
    client: Client
}

impl FeatureFlagManager {
    pub async fn new() -> Self {
        let sdk_key = std::env::var("LAUNCHDARKLY_SDK_KEY")
            .expect("LAUNCHDARKLY_SDK_KEY env should be set");

        let config = ConfigBuilder::new(&sdk_key)
            .build()
            .expect("Config failed to build");

        let client = Client::build(config).expect("Client failed to build");
        
        // Starts the client using the currently active runtime.
        client.start_with_default_executor();

        if !client.initialized_async().await {
            panic!("*** SDK failed to initialize. Please check your internet connection and SDK credential for any typo.");
        }

        println!("*** SDK successfully initialized.");

        FeatureFlagManager { client }
    }

    pub fn is_forcing_api_call(&self) -> bool {
        let context = ContextBuilder::new("example-user-key")
            .kind("user")
            .name("Sandy")
            .build()
            .expect("Context failed to build");

        self.client.bool_variation(&context, "forcing-api-call", false)
    }
}