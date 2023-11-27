use crate::Client;
use reqwest::blocking::Client as HttpClient;
use url::form_urlencoded;
use std::fmt;
use std::collections::HashMap;
use crate::ReqOpts;

pub struct EmailVerifyClient {
    // Common client system.
    pub client: Client,
}

pub struct SingleVerifyReqOpts {
    pub req_opts: ReqOpts,
    pub disable_url_encode: Option<bool>,
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

    pub fn single_verify(&self, email_addr: &str) -> Result<SingleVerifyResp, reqwest::Error> {
        let email_addr = form_urlencoded::byte_serialize(email_addr.as_bytes()).collect();
        let qp: HashMap<&str, String> = [("email", email_addr)].iter().cloned().collect();

        let res = self.client.http.get("https://mslm.io/api/sv/v1")
            .query(&qp)
            .send()?;

        let res = res.error_for_status()?;

        let sv_resp: SingleVerifyResp = res.json()?;

        Ok(sv_resp)
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

impl fmt::Display for SingleVerifyRespMx {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, r#"{{"host":"{}","pref":{}}}"#, self.host, self.pref)
    }
}

impl fmt::Display for SingleVerifyResp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, r#"{{"email":"{}","username":"{}","domain":"{}","malformed":{},"suggestion":"{}","status":"{}","has_mailbox":{},"accept_all":{},"disposable":{},"free":{},"role":{},"mx":[{}]}}"#,
               self.email, self.username, self.domain, self.malformed, self.suggestion, self.status, self.has_mailbox, self.accept_all, self.disposable, self.free, self.role,
               self.mx.iter().map(|mx| mx.to_string()).collect::<Vec<String>>().join(","))
    }
}
