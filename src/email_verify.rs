use crate::Client;
use reqwest::blocking::Client as HttpClient;

pub struct EmailVerifyClient {
    // Common client system.
    pub client: Client,
}

impl EmailVerifyClient {
    pub fn new(api_key: &str) -> Self {
        let mut client = Client::new();
        client.set_api_key(api_key);
        Self { client }
    }

    pub fn init_defaults() -> Self {
        Self::new("")
    }

    pub fn set_http_client(&mut self, http_client: HttpClient) {
        self.client.set_http_client(http_client);
    }

    pub fn set_base_url(&mut self, base_url_str: &str) -> Result<(), url::ParseError> {
        self.client.set_base_url(base_url_str)
    }

    pub fn set_user_agent(&mut self, user_agent: &str) {
        self.client.set_user_agent(user_agent);
    }

    pub fn set_api_key(&mut self, api_key: &str) {
        self.client.set_api_key(api_key);
    }
}
