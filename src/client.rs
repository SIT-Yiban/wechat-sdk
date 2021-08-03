use reqwest::Client;

pub struct WeChatClient {
    /// Wechat mini-program appid.
    appid: String,
    /// Wechat mini-program secret key.
    secret: String,

    /// Reqwest HTTP client
    client: Client,
}

pub struct WeChatClientBuilder {
    /// Wechat mini-program appid.
    appid: Option<String>,
    /// Wechat mini-program secret key.
    secret: Option<String>,
}

impl WeChatClientBuilder {
    pub fn new() -> Self {
        WeChatClientBuilder {
            appid: None,
            secret: None,
        }
    }

    pub fn appid(mut self, appid: String) -> Self {
        self.appid = Some(appid);
        self
    }

    pub fn secret(mut self, secret: &str) -> Self {
        self.secret = Some(secret.to_string());
        self
    }

    pub fn build(self: Self) -> WeChatClient {
        WeChatClient {
            appid: self.appid.unwrap_or_else(|| {
                panic!("Appid is required in WeChatClientBuilder, please call appid method.")
            }),
            secret: self.secret.unwrap_or_else(|| {
                panic!("Secret is required in WeChatClientBuilder, please call secret method.")
            }),
            client: Client::new(),
        }
    }
}
