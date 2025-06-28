use thirtyfour::prelude::*;
use tokio::time::{sleep, Duration};

use crate::api::{Header, Settings};
use crate::config::CrawlerConfig;
use crate::shared::{Driver, Firewall};
use crate::error::CrawlerError;

/// Initialize the WebDriver and setup the search
pub async fn initialize_crawler(config: &CrawlerConfig) -> Result<WebDriver, CrawlerError> {
  let driver = <dyn Driver>::get_driver().await?;

  if config.fullscreen_mode {
      driver.maximize_window().await?;
  }

  driver.goto(&config.url).await?;
  sleep(Duration::from_secs(config.login_delay)).await;

  prepare_search(&driver, config).await?;

  Ok(driver)
}

/// Prepare the search by setting location and search query
pub async fn prepare_search(driver: &WebDriver, config: &CrawlerConfig) -> Result<(), CrawlerError> {
  <dyn Settings>::click_open_geo_modal_btn(driver.clone()).await?;
  <dyn Settings>::click_clear_btn(driver.clone()).await?;
  <dyn Settings>::write_region_input(driver.clone(), &config.city_query).await?;
  <dyn Settings>::click_region_suggest(driver.clone()).await?;
  <dyn Settings>::click_geo_confirm(driver.clone()).await?;
  <dyn Settings>::write_search_input(driver.clone(), &config.search_query).await?;
  <dyn Settings>::select_search_suggest(
      driver.clone(),
      config.select_suggest,
      false, // This should be determined by crawler type
  ).await?;

  Ok(())
}

/// Handle firewall detection and waiting
pub async fn handle_firewall(driver: &WebDriver) -> Result<(), CrawlerError> {
	let firewall_msg = <dyn Firewall>::get_firewall(driver.clone()).await?;

	if firewall_msg {
		'firewall: for _ in 0..=3600 {
			println!("====== firewall ======");
			sleep(Duration::from_secs(30)).await;

			let firewall_msg_in_loop = <dyn Firewall>::get_firewall(driver.clone()).await?;

			if !firewall_msg_in_loop {
				break 'firewall;
			}
		}
	}

  Ok(())
}

/// Get search results metadata
pub async fn get_search_metadata(driver: &WebDriver) -> Result<(String, f64), CrawlerError> {
    let categories = <dyn Header>::get_categories(driver.clone()).await?;
    let ads_count = <dyn Header>::get_ads_count(driver.clone()).await?;
    Ok((categories, ads_count.into()))
}

/// Calculate pagination info
pub fn calculate_pagination(ads_count: f64) -> i32 {
    if ads_count > 50.0 {
        (ads_count / 50.0).ceil() as i32
    } else {
        ads_count.ceil() as i32
    }
}
