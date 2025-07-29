use secrecy::{ExposeSecret, SecretString};
use worker::{Method, Url};

use crate::http::RequestBuilder;

#[derive(Debug, Clone)]
pub struct ApiToken(SecretString);

impl From<String> for ApiToken {
    fn from(api_token: String) -> Self {
        Self(SecretString::from(api_token))
    }
}

impl ExposeSecret<str> for ApiToken {
    fn expose_secret(&self) -> &str {
        self.0.expose_secret()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ApiVersion {
    V1,
    V2,
    #[allow(dead_code)]
    V3,
}

#[derive(Debug)]
pub struct Client {
    dash_origin: Url,
    api_token: ApiToken,
}

impl ApiVersion {
    fn url(&self, origin: &str) -> String {
        let url_prefix = match self {
            ApiVersion::V1 => "api/v1",
            ApiVersion::V2 => "api/v2",
            ApiVersion::V3 => "api/v3",
        };

        format!("{origin}{url_prefix}")
    }
}

impl Client {
    pub fn new(dash_origin: Url, api_token: ApiToken) -> Self {
        Self {
            dash_origin,
            api_token,
        }
    }

    fn build_request(&self, version: ApiVersion, method: Method, path: &str) -> RequestBuilder {
        let endpoint = format!("{}{}", version.url(self.dash_origin.as_str()), path);

        RequestBuilder::new(method, &endpoint)
            .with_header("Xc-Token", self.api_token.0.expose_secret())
            .with_header("Accept", "application/json")
    }

    // Currently just used for the health check endpoint.
    pub fn build_request_v1(&self, method: Method, path: &str) -> RequestBuilder {
        self.build_request(ApiVersion::V1, method, path)
    }

    // Once it's stable, we can migrate to v3 of the NocoDB API.
    pub fn build_request_v2(&self, method: Method, path: &str) -> RequestBuilder {
        self.build_request(ApiVersion::V2, method, path)
    }
}
