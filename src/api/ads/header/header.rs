use thirtyfour::prelude::*;

use crate::shared::Crawler;

pub trait Header {}

impl dyn Header {
	pub async fn get_categories(driver: WebDriver) -> Result<String, WebDriverError> {
		let categories = match <dyn Crawler>::find_elements(
			driver.clone(),
			"//div[contains(@class, \"breadcrumbs-root\")]/span/a".to_string(),
			"//body/div[1]/div/buyer-location/div/div/div[2]/div/div[2]/div[1]/span[1]/a"
				.to_string(),
		)
		.await
		{
			Ok(res) => res,
			Err(e) => {
				println!("error while searching categories block: {}", e);
				Vec::new()
			}
		};

		let mut category_arr = Vec::new();

		for category in categories {
			let res = category.find(By::Tag("span")).await?.text().await?;
			if res != "Главная" {
				category_arr.push(res);
			}
		}

		let categories_str = category_arr
			.iter()
			.map(|x| format!("{}; ", x.to_string()))
			.collect::<String>();

		Ok(categories_str)
	}

	pub async fn get_ads_count(driver: WebDriver) -> Result<f32, WebDriverError> {
		let ads_count = match <dyn Crawler>::find_text(
			driver.clone(),
			"//span[contains(@class, \"page-title-count\")]".to_string(),
			"//body/div[1]/div/buyer-location/div/div/div[2]/div/div[2]/div[2]/div/span"
				.to_string(),
		)
		.await
		{
			Ok(elem) => elem
				.replace("&nbsp;", "")
				.replace(" ", "")
				.parse::<f32>()
				.unwrap_or(0.0),
			Err(e) => {
				println!("error while searching ads_count block: {}", e);
				0.0
			}
		};

		Ok(ads_count)
	}
}
