use super::subpaths_scraper::SubPathsScraper;
use crate::communication::{UrlsMessage, Receiver, Channel};
use crate::http::{Response, Url};

use log::{debug, trace};

pub trait ScraperProvider: Send + Sync {
    fn scrap(&self, base_url: Url, response: &Response);
    fn receiver(&self) -> &Receiver<UrlsMessage>;
}

pub struct EmptyScraperProvider {
    channel: Channel<UrlsMessage>
}

impl EmptyScraperProvider {
    pub fn new() -> Self {
        debug!("Scraper: Create EmptyScraperProvider");
        let s = Self {channel: Channel::new()};
        return s;
    }
}

impl ScraperProvider for EmptyScraperProvider {
    fn scrap(&self, _: Url, _: &Response) {}
    fn receiver(&self) -> &Receiver<UrlsMessage> {
        return &self.channel.receiver;
    }
}

pub struct UrlsScraperProvider {
    channel: Channel<UrlsMessage>,
    subpaths_scraper: SubPathsScraper,
}

impl UrlsScraperProvider {
    pub fn new() -> Self {
        debug!("Scraper: Create UrlsScraperProvider");
        return Self {
            channel: Channel::new(),
            subpaths_scraper: SubPathsScraper::new(),
        };
    }

    fn send_urls(&self, message: UrlsMessage) {
        trace!("Scraper: send {} urls", message.urls.len());
        if message.urls.len() != 0 {
            self.send(message);
        }
    }

    fn send(&self, message: UrlsMessage) {
        self.channel.sender.send(message).expect("Scraper: Error sending urls");
    }
}

impl ScraperProvider for UrlsScraperProvider {
    fn scrap(&self, base_url: Url, response: &Response) {
        let urls = self.subpaths_scraper.scrap(&base_url, response).collect();
        let urls_message = UrlsMessage::new(base_url, urls);
        self.send_urls(urls_message);
    }

    fn receiver(&self) -> &Receiver<UrlsMessage> {
        return &self.channel.receiver;
    }
}
