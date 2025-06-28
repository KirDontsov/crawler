use std::env;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Environment variable {0} not set")]
    MissingEnvVar(String),
    #[error("Invalid value for {0}: {1}")]
    InvalidValue(String, String),
}

#[derive(Debug, Clone)]
pub struct CrawlerConfig {
    pub search_query: String,
    pub city_query: String,
    pub url: String,
    pub select_suggest: i32,
    pub fullscreen_mode: bool,
    pub accounts_to_check: Vec<String>,
    pub ads_to_check: Vec<String>,
    pub visit_ads_page: bool,
    pub report_directory: String,
    pub login_delay: u64,
    pub headless_chrome: bool,
}

impl CrawlerConfig {
    pub fn from_env() -> Result<Self, ConfigError> {
        let search_query = env::var("SEARCH_QUERY")
            .map_err(|_| ConfigError::MissingEnvVar("SEARCH_QUERY".to_string()))?;

        let city_query = env::var("CITY_QUERY")
            .map_err(|_| ConfigError::MissingEnvVar("CITY_QUERY".to_string()))?;

        let url = env::var("URL_QUERY")
            .map_err(|_| ConfigError::MissingEnvVar("URL_QUERY".to_string()))?;

        let select_suggest = env::var("SELECT_SUGGEST")
            .map_err(|_| ConfigError::MissingEnvVar("SELECT_SUGGEST".to_string()))?
            .parse::<i32>()
            .map_err(|e| ConfigError::InvalidValue("SELECT_SUGGEST".to_string(), e.to_string()))?;

        let fullscreen_mode = env::var("FULLSCREEN_MODE")
            .map_err(|_| ConfigError::MissingEnvVar("FULLSCREEN_MODE".to_string()))?
            .parse::<bool>()
            .map_err(|e| ConfigError::InvalidValue("FULLSCREEN_MODE".to_string(), e.to_string()))?;

        let accounts_to_check = env::var("ACCAUNTS_TO_CHECK")
            .unwrap_or_default()
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();

        let ads_to_check = env::var("ADS_TO_CHECK")
            .unwrap_or_default()
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();

        let visit_ads_page = env::var("VISIT_ADS_PAGE")
            .map_err(|_| ConfigError::MissingEnvVar("VISIT_ADS_PAGE".to_string()))?
            .parse::<bool>()
            .map_err(|e| ConfigError::InvalidValue("VISIT_ADS_PAGE".to_string(), e.to_string()))?;

        let report_directory = env::var("REPORT_DIRECTORY")
            .map_err(|_| ConfigError::MissingEnvVar("REPORT_DIRECTORY".to_string()))?;

        let login_delay = env::var("LOGIN_DELAY")
            .unwrap_or("1".to_string())
            .parse::<u64>()
            .map_err(|e| ConfigError::InvalidValue("LOGIN_DELAY".to_string(), e.to_string()))?;

        let headless_chrome = env::var("HEADLESS_CHROME")
            .map_err(|_| ConfigError::MissingEnvVar("HEADLESS_CHROME".to_string()))?
            .parse::<bool>()
            .map_err(|e| ConfigError::InvalidValue("HEADLESS_CHROME".to_string(), e.to_string()))?;

        Ok(Self {
            search_query,
            city_query,
            url,
            select_suggest,
            fullscreen_mode,
            accounts_to_check,
            ads_to_check,
            visit_ads_page,
            report_directory,
            login_delay,
            headless_chrome,
        })
    }
}
