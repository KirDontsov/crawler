use thirtyfour::prelude::*;
use tokio::time::{sleep, Duration};

use crate::shared::Crawler;

pub trait Feed {}

impl dyn Feed {
	pub async fn get_feed(driver: WebDriver) -> Result<Vec<WebElement>, WebDriverError> {
		let blocks = match <dyn Crawler>::find_elements(
			driver.clone(),
			"//div[contains(@class, \"items-items\")][1]/div[contains(@class, \"iva-item-root\")]".to_string(),
			"//body/div[1]/div/buyer-location/div/div/div[2]/div/div[2]/div[3]/div[3]/div[4]/div[2]/div[contains(@class, \"iva-item-root\")]".to_string(),
		)
		.await
		{
			Ok(res) => res,
			Err(e) => {
				println!("error while searching blocks block: {}", e);
				driver.clone().quit().await?;
				Vec::new()
			}
		};

		Ok(blocks)
	}

	pub async fn get_href(
		driver: WebDriver,
		xpath: String,
		xpath2: String,
	) -> Result<String, WebDriverError> {
		let href_full =
			match <dyn Crawler>::find_attr(driver.clone(), xpath, xpath2, "href".to_string()).await
			{
				Ok(elem) => elem,
				Err(e) => {
					println!("error while searching href block: {}", e);
					driver.clone().quit().await?;
					"".to_string()
				}
			};

		let href_arr = href_full.split("?").collect::<Vec<&str>>();
		let href_str = href_arr.get(0).expect("no href_str");

		Ok(format!("https://avito.ru{}", href_str.to_owned()))
	}

	pub async fn get_text(
		driver: WebDriver,
		xpath: String,
		xpath2: String,
	) -> Result<String, WebDriverError> {
		let text = match <dyn Crawler>::find_text(driver.clone(), xpath, xpath2).await {
			Ok(elem) => elem,
			Err(e) => {
				println!("error while searching href block: {}", e);
				driver.clone().quit().await?;
				"".to_string()
			}
		};

		Ok(text)
	}

	pub async fn get_price(
		driver: WebDriver,
		xpath: String,
		xpath2: String,
	) -> Result<String, WebDriverError> {
		let price =
			match <dyn Crawler>::find_attr(driver.clone(), xpath, xpath2, "content".to_string())
				.await
			{
				Ok(elem) => elem,
				Err(e) => {
					println!("error while searching price_block block: {}", e);
					driver.clone().quit().await?;
					"".to_string()
				}
			};

		Ok(price)
	}

	pub async fn move_mouse_to_paid(driver: WebDriver, xpath: String) -> Result<(), WebDriverError> {
		let paid_icon_arr = match <dyn Crawler>::find_elements(driver.clone(), xpath, "".to_string()).await
		{
			Ok(res) => res,
			Err(e) => {
				println!("error while searching seller_name block: {}", e);
				driver.clone().quit().await?;
				Vec::new()
			}
		};



		if paid_icon_arr.len() != 0 {
			let paid_icon = paid_icon_arr.get(0).expect("no paid_icon");

			driver
				.action_chain()
				.move_to_element_center(&paid_icon)
				.perform()
				.await?;
		}

		Ok(())
	}

	pub async fn get_paid_imgs(
		driver: WebDriver,
		xpath: String,
		xpath2: String,
	) -> Result<Vec<WebElement>, WebDriverError> {
		let imgs_arr =
			match <dyn Crawler>::find_elements(driver.clone(), xpath, xpath2).await
			{
				Ok(elem) => elem,
				Err(e) => {
					println!("error while searching href block: {}", e);
					driver.clone().quit().await?;
					Vec::new()
				}
			};

		Ok(imgs_arr)
	}

	pub async fn get_feed_parent_block(driver: WebDriver) -> Result<WebElement, WebDriverError> {
		let main_arr = match <dyn Crawler>::find_elements(
			driver.clone(),
			"//div[contains(@class, \"index-content\")]".to_string(),
			"//div[contains(@class, \"index-inner\")]".to_string(),
		)
		.await
		{
			Ok(elem) => elem,
			Err(e) => {
				println!("error while searching main_arr block: {}", e);
				Vec::new()
			}
		};

		let parent = main_arr.get(0).expect("no parent");

		Ok(parent.to_owned())
	}

	pub async fn get_paid_img(
		driver: WebDriver,
		xpath: String,
		xpath2: String,
	) -> Result<String, WebDriverError> {
		let img_src_full =
			match <dyn Crawler>::find_attr(driver.clone(), xpath, xpath2, "src".to_string()).await
			{
				Ok(elem) => elem,
				Err(e) => {
					println!("error while searching href block: {}", e);
					driver.clone().quit().await?;
					"".to_string()
				}
			};

		let img_src_arr = img_src_full.split("/").collect::<Vec<&str>>();
		let img_src_str = img_src_arr.last().expect("no img src");

		Ok(img_src_str.replace(".svg", "").to_owned().to_string())
	}

	pub async fn click_ad_title_link(
		driver: WebDriver,
		xpath: String,
		xpath2: String,
	) -> Result<(), WebDriverError> {
		let title_link_arr = match <dyn Crawler>::find_elements(driver.clone(), xpath, xpath2).await
		{
			Ok(res) => res,
			Err(e) => {
				println!("error while searching seller_name block: {}", e);
				driver.clone().quit().await?;
				Vec::new()
			}
		};

		let title_link = title_link_arr.get(0).expect("no title_link");

		driver
			.action_chain()
			.move_to_element_center(&title_link)
			.click()
			.perform()
			.await?;

		Ok(())
	}

	pub async fn click_pagination_next_btn(driver: WebDriver) -> Result<(), WebDriverError> {
		let button_arr = match <dyn Crawler>::find_elements(
			driver.clone(),
			"//div[contains(@class, \"pagination-pagination\")]/nav/ul/li[last()]/a".to_string(),
			"//body/div[1]/div/div[4]/div/div[2]/div[3]/div[3]/div[5]/nav/ul/li[6]/a".to_string(),
		)
		.await
		{
			Ok(elem) => elem,
			Err(e) => {
				println!("error while searching button_arr block: {}", e);
				driver.clone().quit().await?;
				Vec::new()
			}
		};

		let button = button_arr.get(0).expect("no button");

		button.click().await?;
		sleep(Duration::from_secs(5)).await;

		Ok(())
	}
}
