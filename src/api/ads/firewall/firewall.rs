use thirtyfour::prelude::*;

use crate::shared::Crawler;

pub trait Firewall {}

impl dyn Firewall {
	pub async fn get_firewall(driver: WebDriver) -> Result<bool, WebDriverError> {
		let firewall_msg = match <dyn Crawler>::check_if_block_exists(
			driver.clone(),
			"//h2[contains(@class, \"firewall-title\")]".to_string(),
			"".to_string(),
		)
		.await
		{
			Ok(res) => res,
			Err(e) => {
				println!("error while searching firewall_msg block: {}", e);
				false
			}
		};

		Ok(firewall_msg)
	}
}
