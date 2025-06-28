use thirtyfour::prelude::*;

/// Common error type for crawler operations
#[allow(dead_code)]
#[derive(Debug)]
pub enum CrawlerError {
    WebDriver(WebDriverError),
    Config(crate::config::ConfigError),
    Io(std::io::Error),
    Csv(csv::Error),
}

impl From<WebDriverError> for CrawlerError {
    fn from(err: WebDriverError) -> Self {
        CrawlerError::WebDriver(err)
    }
}

impl From<crate::config::ConfigError> for CrawlerError {
    fn from(err: crate::config::ConfigError) -> Self {
        CrawlerError::Config(err)
    }
}

impl From<std::io::Error> for CrawlerError {
    fn from(err: std::io::Error) -> Self {
        CrawlerError::Io(err)
    }
}

impl From<csv::Error> for CrawlerError {
    fn from(err: csv::Error) -> Self {
        CrawlerError::Csv(err)
    }
}
