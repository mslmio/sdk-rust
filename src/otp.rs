use crate::{BaseClient, ReqOpts, RequestError};
use reqwest::blocking::Client as HttpClient;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Deserialize)]
pub struct OtpSendResp {
    code: i64,
    msg: String,
}

impl fmt::Display for OtpSendResp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, r#"{{"code":"{}","msg":"{}"}}"#, self.code, self.msg)
    }
}

#[derive(Serialize)]
pub struct OtpSendReq {
    pub phone: String,
    pub tmpl_sms: String,
    pub token_len: i32,
    pub expire_seconds: i32,
}

impl fmt::Display for OtpSendReq {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            r#"{{"phone":"{}","tmpl_sms":"{}","token_len":"{}","tmpl_sms":"{}","expire_seconds":"{}" }}"#,
            self.phone, self.tmpl_sms, self.token_len, self.tmpl_sms, self.expire_seconds
        )
    }
}

pub struct OtpSendReqOpts {
    pub req_opts: ReqOpts,
}

pub struct OtpClient {
    pub client: BaseClient,
}

impl OtpClient {
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

    pub fn set_base_url(&mut self, base_url_str: &str) -> Result<(), url::ParseError> {
        self.client.set_base_url(base_url_str)
    }

    pub fn set_user_agent(&mut self, user_agent: &str) {
        self.client.set_user_agent(user_agent);
    }

    pub fn set_api_key(&mut self, api_key: &str) {
        self.client.set_api_key(api_key);
    }

    pub async fn send(
        &self,
        otp_send_req: &OtpSendReq,
        opts: Option<&OtpSendReqOpts>,
    ) -> Result<OtpSendResp, RequestError> {
        // Prepare options.
        let default_opts = OtpSendReqOpts {
            req_opts: self.client.prepare_opts(&ReqOpts::default()),
        };

        let opt = opts.unwrap_or(&default_opts);

        // Prepare URL.
        let qp: HashMap<String, String> = HashMap::new();
        let t_url = self.client.prepare_url("api/otp/v1/send", &qp, &opt.req_opts)?;

        // Serialize.
        let data = serde_json::to_vec(otp_send_req)?;

        // Get data.
        let resp = self
            .client
            .req_and_resp(reqwest::Method::POST, t_url, Some(data), &opt.req_opts)
            .await?;

        Ok(resp)
    }
}
