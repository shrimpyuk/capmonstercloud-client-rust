use reqwestplus::Url;

use crate::error::OptionsError::{self, UrlParseError};

pub struct Options {
    pub(crate) client_key: String,
    pub(crate) service_uri: Url,
    pub(crate) soft_id: u32,
}

impl Options {
    const DEFAULT_API_URI: &'static str = "https://api.capmonster.cloud";
    const DEFAULT_SOFT_ID: u32 = 60;
    
    pub fn new(client_key: String) -> Result<Self, OptionsError> {
        Self::new_ex(client_key, None, None)
    }
    
    pub fn new_ex(
        client_key: String,
        service_uri: Option<String>,
        soft_id: Option<u32>,
    ) -> Result<Self, OptionsError> {
        Ok(Self {
            client_key,
            service_uri: Url::parse(&service_uri
                .unwrap_or(Self::DEFAULT_API_URI.to_string()))
                .map_err(|e| UrlParseError(e.to_string()))?,
            soft_id: soft_id.unwrap_or(Self::DEFAULT_SOFT_ID),
        })
    }
}
