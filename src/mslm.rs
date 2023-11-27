use reqwest::blocking::Client as HttpClient;
use reqwest::Error;
use crate::{BaseClient, EmailVerifyClient};

static mut DEFAULT_CLIENT: Option<Client> = None;
static mut DEFAULT_HTTP_CLIENT: HttpClient = HttpClient::new();
static mut DEFAULT_BASE_URL: &'static str = "https://mslm.io";
static mut DEFAULT_USER_AGENT: &'static str = "mslm/go/1.1.2";
static mut DEFAULT_API_KEY: &'static str = "";

struct Client {
    // Common client system.
    c: BaseClient,

    // The Email Verify API client.
    email_verify: EmailVerifyClient,
}

impl Client {
    fn init(api_key: &'static str) -> Self {
        let mut client = Client {
            c: BaseClient::new(),
            email_verify: EmailVerifyClient::new(api_key),
        };
        client.set_http_client(HttpClient::new());
        client.set_base_url(DEFAULT_BASE_URL).unwrap();
        client.set_user_agent(DEFAULT_USER_AGENT);
        client.set_api_key(api_key);
        client
    }

    fn init_defaults() -> Self {
        Client::init(DEFAULT_API_KEY)
    }

    fn set_http_client(&mut self, http_client: HttpClient) {
        self.c.set_http_client(http_client);
        self.email_verify.set_http_client(http_client.clone());
    }

    fn set_base_url(&mut self, base_url_str: &'static str) -> Result<(), Error> {
        self.c.set_base_url(base_url_str)?;
        self.email_verify.set_base_url(base_url_str)?;
        Ok(())
    }

    fn set_user_agent(&mut self, user_agent: &'static str) {
        self.c.set_user_agent(user_agent);
        self.email_verify.set_user_agent(user_agent);
    }

    fn set_api_key(&mut self, api_key: &'static str) {
        self.c.set_api_key(api_key);
        self.email_verify.set_api_key(api_key);
    }
}

fn set_http_client(http_client: HttpClient) {
    unsafe {
        if let Some(ref mut default_client) = DEFAULT_CLIENT {
            default_client.set_http_client(http_client.clone());
        }
        DEFAULT_HTTP_CLIENT = http_client;
    }
}

fn set_base_url(base_url_str: &'static str) -> Result<(), Error> {
    unsafe {
        if let Some(ref mut default_client) = DEFAULT_CLIENT {
            default_client.set_base_url(base_url_str)?;
        }
        DEFAULT_BASE_URL = base_url_str;
    }
    Ok(())
}

fn set_user_agent(user_agent: &'static str) {
    unsafe {
        if let Some(ref mut default_client) = DEFAULT_CLIENT {
            default_client.set_user_agent(user_agent);
        }
        DEFAULT_USER_AGENT = user_agent;
    }
}

fn set_api_key(api_key: &'static str) {
    unsafe {
        if let Some(ref mut default_client) = DEFAULT_CLIENT {
            default_client.set_api_key(api_key);
        }
        DEFAULT_API_KEY = api_key;
    }
}
