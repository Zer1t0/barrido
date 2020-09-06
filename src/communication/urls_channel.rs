use reqwest::Url;
use super::channel::{Channel, Receiver};

pub type UrlsReceiver = Receiver<UrlsMessage>;
pub type UrlsChannel = Channel<UrlsMessage>;

#[derive(Clone)]
pub struct UrlsMessage {
    pub base_url: Url,
    pub urls: Vec<Url>,
}

impl UrlsMessage {
    pub fn new(base_url: Url, urls: Vec<Url>) -> Self {
        return Self { base_url, urls };
    }
}