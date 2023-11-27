use reqwest::blocking::Client as HttpClient;
use url::Url;

pub struct Client {
    // HTTP client used for making requests.
    pub http: HttpClient,

    // Base URL for API requests.
    base_url: Url,

    // User-agent used when communicating with the API.
    user_agent: String,

    // The API key used for authentication & authorization.
    api_key: String,
}

impl Client {

    pub fn new() -> Self {
        Self {
            http: HttpClient::new(),
            base_url: Url::parse("https://mslm.io").expect("Failed to parse default base URL"),
            user_agent: String::from("mslm/rust/1.0.0"),
            api_key: String::new(),
        }
    }

    pub fn set_http_client(&mut self, http_client: HttpClient) {
        self.http = http_client;
    }

    pub fn set_base_url(&mut self, base_url_str: &str) -> Result<(), url::ParseError> {
        let base_url = Url::parse(base_url_str)?;
        self.base_url = base_url;
        Ok(())
    }

    pub fn set_user_agent(&mut self, user_agent: &str) {
        self.user_agent = user_agent.to_string();
    }

    pub fn set_api_key(&mut self, api_key: &str) {
        self.api_key = api_key.to_string();
    }

}

pub fn init_defaults() -> Client {
    Client::new()
}

