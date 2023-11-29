use crate::BaseClient;
use crate::{ReqOpts, RequestError};
use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
use reqwest;
use reqwest::blocking::Client as HttpClient;
use std::collections::HashMap;
use std::fmt;

pub struct EmailVerifyClient {
    // Common client system.
    pub client: BaseClient,
}

pub struct SingleVerifyReqOpts {
    pub req_opts: ReqOpts,
    pub disable_url_encode: Option<bool>,
}

impl EmailVerifyClient {
    pub fn new(api_key: &str) -> Self {
        let mut client = BaseClient::new();
        client.set_api_key(api_key);
        Self { client }
    }

    pub fn init_defaults() -> Self {
        Self::new("")
    }

    pub fn set_http_client(&mut self, http_client: HttpClient) {
        self.client.set_http_client(http_client);
    }

    pub fn set_base_url(
        &mut self,
        base_url_str: &str,
    ) -> Result<(), url::ParseError> {
        self.client.set_base_url(base_url_str)
    }

    pub fn set_user_agent(&mut self, user_agent: &str) {
        self.client.set_user_agent(user_agent);
    }

    pub fn set_api_key(&mut self, api_key: &str) {
        self.client.set_api_key(api_key);
    }

    pub async fn single_verify(
        &self,
        email_addr: &str,
        opts: Option<&SingleVerifyReqOpts>,
    ) -> Result<SingleVerifyResp, RequestError> {
        let default_opts = SingleVerifyReqOpts {
            req_opts: self.client.prepare_opts(&ReqOpts::default()),
            disable_url_encode: None,
        };

        let opt = opts.unwrap_or(&default_opts);

        let email_addr = if let Some(disable_url_encode) =
            opt.disable_url_encode
        {
            if !disable_url_encode {
                utf8_percent_encode(email_addr, NON_ALPHANUMERIC).to_string()
            } else {
                email_addr.to_string()
            }
        } else {
            email_addr.to_string()
        };

        let mut qp: HashMap<String, String> = HashMap::new();
        qp.insert("email".to_string(), email_addr);

        let t_url_result =
            self.client.prepare_url("api/sv/v1", &qp, &opt.req_opts);
        let t_url = match t_url_result {
            Ok(url) => url,
            Err(err) => return Err(err),
        };

        let res: SingleVerifyResp = self
            .client
            .req_and_resp(reqwest::Method::GET, t_url, None, &opt.req_opts)
            .await?;

        Ok(res)
    }
}

#[derive(serde::Deserialize)]
struct SingleVerifyRespMx {
    host: String,
    pref: i32,
}

#[derive(serde::Deserialize)]
pub struct SingleVerifyResp {
    email: String,
    username: String,
    domain: String,
    malformed: bool,
    suggestion: String,
    status: String,
    has_mailbox: bool,
    accept_all: bool,
    disposable: bool,
    free: bool,
    role: bool,
    mx: Vec<SingleVerifyRespMx>,
}

