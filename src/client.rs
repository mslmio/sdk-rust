use reqwest::blocking::Client as HttpClient;
use serde::de::DeserializeOwned;
use url::Url;

#[derive(Debug, Default)]
pub struct ReqOpts {
    pub http: Option<HttpClient>,
    pub base_url: Option<Url>,
    pub user_agent: Option<String>,
    pub api_key: Option<String>,
}

#[derive(Debug)]
pub struct RequestError {
    pub message: String,
}

impl std::fmt::Display for RequestError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for RequestError {}

impl From<reqwest::Error> for RequestError {
    fn from(err: reqwest::Error) -> Self {
        RequestError {
            message: format!("Request failed: {}", err),
        }
    }
}

impl From<url::ParseError> for RequestError {
    fn from(err: url::ParseError) -> Self {
        RequestError {
            message: format!("URL parsing failed: {}", err),
        }
    }
}

impl From<serde_json::Error> for RequestError {
    fn from(err: serde_json::Error) -> Self {
        RequestError {
            message: format!("JSON parsing failed: {}", err),
        }
    }
}

pub struct BaseClient {
    // HTTP client used for making requests.
    pub http: HttpClient,

    // Base URL for API requests.
    pub base_url: Url,

    // User-agent used when communicating with the API.
    user_agent: String,

    // The API key used for authentication & authorization.
    api_key: String,
}

impl BaseClient {
    pub fn new() -> Self {
        Self {
            http: HttpClient::new(),
            base_url: Url::parse("https://mslm.io")
                .expect("Failed to parse default base URL"),
            user_agent: String::from("mslm/rust/1.0.0"),
            api_key: String::new(),
        }
    }

    pub fn set_http_client(&mut self, http_client: HttpClient) {
        self.http = http_client;
    }

    pub fn set_base_url(
        &mut self,
        base_url_str: &str,
    ) -> Result<(), url::ParseError> {
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

    pub fn prepare_opts(&self, opt: &ReqOpts) -> ReqOpts {
        let http_c = opt.http.clone().unwrap_or_else(|| self.http.clone());
        let base_url = opt
            .base_url
            .clone()
            .unwrap_or_else(|| self.base_url.clone());
        let user_agent = opt
            .user_agent
            .clone()
            .unwrap_or_else(|| self.user_agent.clone());
        let api_key =
            opt.api_key.clone().unwrap_or_else(|| self.api_key.clone());

        ReqOpts {
            http: Some(http_c),
            base_url: Some(base_url),
            user_agent: Some(user_agent),
            api_key: Some(api_key),
        }
    }

    pub fn prepare_url(
        &self,
        url_path: &str,
        qp: &std::collections::HashMap<String, String>,
        opt: &ReqOpts,
    ) -> Result<Url, RequestError> {
        let base_url = opt.base_url.clone().ok_or_else(|| RequestError {
            message: "Base URL not configured.".to_string(),
        })?;

        let mut t_url = Url::parse(&(base_url.to_string() + url_path))?;

        for (k, v) in qp {
            t_url.query_pairs_mut().append_pair(k, v);
        }

        t_url
            .query_pairs_mut()
            .append_pair("apikey", opt.api_key.as_deref().unwrap_or_default());

        Ok(t_url)
    }

    pub async fn req_and_resp<T: DeserializeOwned>(
        &self,
        method: reqwest::Method,
        t_url: Url,
        data: Option<Vec<u8>>,
        opt: &ReqOpts,
    ) -> Result<T, RequestError> {
        let cloned_data = data.clone().unwrap_or_default();
        let body_data: &'static [u8] =
            Box::leak(cloned_data.into_boxed_slice());

        let request = self
            .http
            .request(method, t_url.as_str())
            .header(
                reqwest::header::USER_AGENT,
                opt.user_agent.as_deref().unwrap_or_default(),
            )
            .body::<&[u8]>(body_data);

        let resp = request.send()?;

        let body = resp.text()?;
        let resp_data: T = serde_json::from_str(&body)?;

        Ok(resp_data)
    }
}

pub fn init_defaults() -> BaseClient {
    BaseClient::new()
}
