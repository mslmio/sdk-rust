use crate::{BaseClient, ReqOpts, RequestError};
use reqwest::blocking::Client as HttpClient;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
pub struct OtpSendResp {
    code: i64,
    msg: String,
}

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
pub struct OtpSendReq {
    pub phone: String,
    pub tmpl_sms: String,
    pub token_len: i32,
    pub expire_seconds: i32,
}

pub struct OtpSendReqOpts {
    pub req_opts: ReqOpts,
}

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
pub struct OtpTokenVerifyResp {
    code: i64,
    msg: String,
}

pub struct OtpTokenVerifyReqOpts {
    // Common request options.
    pub req_opts: ReqOpts,
}

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
pub struct OtpTokenVerifyReq {
    pub phone: String,
    pub token: String,
    pub consume: Option<bool>,
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
        let t_url =
            self.client
                .prepare_url("api/otp/v1/send", &qp, &opt.req_opts)?;

        // Serialize.
        let data = serde_json::to_vec(otp_send_req)?;

        // Get data.
        let resp = self
            .client
            .req_and_resp(
                reqwest::Method::POST,
                t_url,
                Some(data),
                &opt.req_opts,
            )
            .await?;

        Ok(resp)
    }

    pub async fn verify(
        &self,
        otp_token_verify_req: &OtpTokenVerifyReq,
        opts: Option<&OtpTokenVerifyReqOpts>,
    ) -> Result<OtpTokenVerifyResp, RequestError> {
        // Prepare options.
        let default_opts = OtpTokenVerifyReqOpts {
            req_opts: self.client.prepare_opts(&ReqOpts::default()),
        };

        let opt = opts.unwrap_or(&default_opts);

        // Prepare URL.
        let qp: HashMap<String, String> = HashMap::new();
        let t_url = self.client.prepare_url(
            "api/otp/v1/token_verify",
            &qp,
            &opt.req_opts,
        )?;

        // Serialize.
        let data = serde_json::to_vec(otp_token_verify_req)?;

        // Get data.
        let resp: OtpTokenVerifyResp = self
            .client
            .req_and_resp(
                reqwest::Method::POST,
                t_url,
                Some(data),
                &opt.req_opts,
            )
            .await?;

        Ok(resp)
    }
}
