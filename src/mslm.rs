use crate::{BaseClient, EmailVerifyClient, OtpClient};
use once_cell::sync::Lazy;
use reqwest::blocking::Client as HttpClient;
use reqwest::Error;

static mut DEFAULT_HTTP_CLIENT: Lazy<HttpClient> =
    Lazy::new(|| HttpClient::new());
static mut DEFAULT_CLIENT: Option<Client> = None;
static mut DEFAULT_BASE_URL: &'static str = "https://mslm.io";
static mut DEFAULT_USER_AGENT: &'static str = "mslm/rust/1.0.0";
static mut DEFAULT_API_KEY: &'static str = "";

pub struct Client {
    // Common client system.
    pub c: BaseClient,

    // The Email Verify API client.
    pub email_verify: EmailVerifyClient,

    // The OTP client.
    pub otp: OtpClient,
}

impl Client {
    pub fn init(api_key: &'static str) -> Self {
        let mut client = Client {
            c: BaseClient::new(),
            email_verify: EmailVerifyClient::new(api_key),
            otp: OtpClient::new(api_key),
        };
        client.set_http_client(HttpClient::new());
        unsafe {
            client.set_base_url(DEFAULT_BASE_URL).unwrap();
            client.set_user_agent(DEFAULT_USER_AGENT);
        }
        client.set_api_key(api_key);
        client
    }

    pub fn init_defaults() -> Self {
        unsafe { Client::init(DEFAULT_API_KEY) }
    }

    pub fn set_http_client(&mut self, http_client: HttpClient) {
        self.c.set_http_client(http_client.clone());
        self.email_verify.set_http_client(http_client.clone());
        self.otp.set_http_client(http_client);
    }

    pub fn set_base_url(
        &mut self,
        base_url_str: &'static str,
    ) -> Result<(), Error> {
        let _ = self.c.set_base_url(base_url_str);
        let _ = self.email_verify.set_base_url(base_url_str);
        let _ = self.otp.set_base_url(base_url_str);
        Ok(())
    }

    pub fn set_user_agent(&mut self, user_agent: &'static str) {
        self.c.set_user_agent(user_agent);
        self.email_verify.set_user_agent(user_agent);
        self.otp.set_user_agent(user_agent);
    }

    pub fn set_api_key(&mut self, api_key: &'static str) {
        self.c.set_api_key(api_key);
        self.email_verify.set_api_key(api_key);
        self.otp.set_api_key(api_key);
    }
}

pub fn set_http_client(http_client: HttpClient) {
    unsafe {
        if let Some(ref mut default_client) = DEFAULT_CLIENT {
            default_client.set_http_client(http_client.clone());
        }
        *DEFAULT_HTTP_CLIENT = http_client;
    }
}

pub fn set_base_url(base_url_str: &'static str) -> Result<(), Error> {
    unsafe {
        if let Some(ref mut default_client) = DEFAULT_CLIENT {
            default_client.set_base_url(base_url_str)?;
        }
        DEFAULT_BASE_URL = base_url_str;
    }
    Ok(())
}

pub fn set_user_agent(user_agent: &'static str) {
    unsafe {
        if let Some(ref mut default_client) = DEFAULT_CLIENT {
            default_client.set_user_agent(user_agent);
        }
        DEFAULT_USER_AGENT = user_agent;
    }
}

pub fn set_api_key(api_key: &'static str) {
    unsafe {
        if let Some(ref mut default_client) = DEFAULT_CLIENT {
            default_client.set_api_key(api_key);
        }
        DEFAULT_API_KEY = api_key;
    }
}
