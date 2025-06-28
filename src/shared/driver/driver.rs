use thirtyfour::{prelude::*, PageLoadStrategy};

use crate::config::CrawlerConfig;

pub trait Driver {}

impl dyn Driver {
	pub async fn get_driver() -> Result<WebDriver, WebDriverError> {
		let config = CrawlerConfig::from_env();
		Self::get_driver_with_headless(config.unwrap().headless_chrome).await
	}

	pub async fn get_driver_with_headless(headless: bool) -> Result<WebDriver, WebDriverError> {

		let mut caps = DesiredCapabilities::chrome();

		// без загрузки изображений
		// caps.insert_browser_option(
		// 	"prefs",
		// 	serde_json::json!({
		// 		"profile.default_content_settings": {
		// 			"images": 2
		// 		},
		// 		"profile.managed_default_content_settings": {
		// 			"images": 2
		// 		}
		// 	}),
		// )?;
		//

		if headless {
			let _ = caps.set_headless();
		}

		let _ = caps.set_page_load_strategy(PageLoadStrategy::Eager)?;
		let _ = caps.add_arg("enable-automation");
		let _ = caps.add_arg("--no-sandbox");
		let _ = caps.add_arg("--disable-extensions");
		let _ = caps.add_arg("--dns-prefetch-disable");
		let _ = caps.add_arg("--disable-gpu");
		let _ = caps.add_arg("--disable-background-mode");
		let _ = caps.add_arg("--disable-sync");
		let _ = caps.add_arg("--disable-translate");
		let _ = caps.add_arg("--disable-plugins");
		let _ = caps.add_arg("--purge-memory-button");
		let _ = caps.add_arg("enable-features=NetworkServiceInProcess");

		let driver = WebDriver::new("http://localhost:9515", caps).await;

		// firefox
		// let mut caps = DesiredCapabilities::firefox();
		// let _ = caps.set_page_load_strategy(PageLoadStrategy::None)?;
		// let driver = WebDriver::new("http://localhost:4444", caps).await;
		driver
	}
}
