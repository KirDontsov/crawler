use thirtyfour::prelude::*;
use tokio::time::{sleep, Duration};

use crate::api::Crawler;

pub trait Settings {}

impl dyn Settings {
	// открытие модального окна
	pub async fn click_open_geo_modal_btn(driver: WebDriver) -> Result<(), WebDriverError> {
		let region_arr = match <dyn Crawler>::find_elements(
			driver.clone(),
			"//div[contains(@class, \"main-richTitleWrapper__content\")]".to_string(),
			"//body/div[1]/div/buyer-location/div/div/div[2]/div/div[1]/div/div/div[4]/div[1]/div"
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

		let region_btn = region_arr.get(0).expect("no region_btn");

		region_btn.click().await?;
		sleep(Duration::from_secs(5)).await;

		Ok(())
	}

	pub async fn click_clear_btn(driver: WebDriver) -> Result<(), WebDriverError> {
		let clear_arr = match <dyn Crawler>::find_elements(
			driver.clone(),
			"//div[contains(@class, \"styles-module-controlIcon\")]".to_string(),
			"//body/div[4]/div[43]/div/div[2]/div/div/div/div/div[1]/div[1]/div/div[2]".to_string(),
		)
		.await
		{
			Ok(res) => res,
			Err(e) => {
				println!("error while searching categories block: {}", e);
				Vec::new()
			}
		};

		let clear_btn = clear_arr.get(0).expect("no clear_btn");
		// очистка строки поиска региона
		clear_btn.click().await?;
		sleep(Duration::from_secs(2)).await;

		Ok(())
	}

	pub async fn write_region_input(
		driver: WebDriver,
		city_query: &str,
	) -> Result<(), WebDriverError> {
		// строка выбора региона
		let region_input_arr = match <dyn Crawler>::find_elements(
			driver.clone(),
			"//input[contains(@class, \"styles-module-searchInput\")]".to_string(),
			"//body/div[4]/div[43]/div/div[2]/div/div/div/div/div[1]/div/div/div[1]/div/input"
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

		let region_input = region_input_arr.get(0).expect("no region_input");
		region_input.send_keys(city_query).await?;
		sleep(Duration::from_secs(2)).await;

		Ok(())
	}

	pub async fn click_region_suggest(driver: WebDriver) -> Result<(), WebDriverError> {
		// подсказки выбора региона
		let region_suggest_arr = match <dyn Crawler>::find_elements(
			driver.clone(),
			"//div[contains(@class, \"styles-module-dropdown\")]/div/div/button[1]".to_string(),
			"//body/div[4]/div[44]/div/div/div/div/div/button[1]".to_string(),
		)
		.await
		{
			Ok(res) => res,
			Err(e) => {
				println!("error while searching categories block: {}", e);
				Vec::new()
			}
		};

		let region_suggest = region_suggest_arr.get(0).expect("no region_suggest");
		// нажимаем на подсказку выбора региона
		region_suggest.click().await?;
		sleep(Duration::from_secs(2)).await;

		Ok(())
	}

	pub async fn click_geo_confirm(driver: WebDriver) -> Result<(), WebDriverError> {
		let confirm_region_btn_arr = match <dyn Crawler>::find_elements(
				driver.clone(),
				"//div[contains(@class, \"popup-buttons-\")]/div[2]/button".to_string(),
				"//body/div[4]/div[43]/div/div[2]/div/div/div/div/div[2]/div[2]/div/div[2]/div[2]/div[2]/button".to_string(),
			)
			.await
			{
				Ok(res) => res,
				Err(e) => {
					println!("error while searching categories block: {}", e);
					Vec::new()
				}
			};

		let confirm_region_btn = confirm_region_btn_arr
			.get(0)
			.expect("no confirm_region_btn");
		// нажимаем на кнопку подтверждение выбора региона
		confirm_region_btn.click().await?;
		sleep(Duration::from_secs(5)).await;

		Ok(())
	}

	pub async fn write_search_input(
		driver: WebDriver,
		search_query: &str,
	) -> Result<(), WebDriverError> {
		// заполнение поиска
		let input_arr = match <dyn Crawler>::find_elements(driver.clone(),
			"//div[contains(@class, \"suggest-input\")]/label/div/div/input".to_string(),
			"//body/div[1]/div/div[4]/div/div[1]/div/div/div[3]/div[2]/div[1]/div/div/label/div/div/input".to_string()
		).await {
			Ok(res) => res,
			Err(e) => {
				println!("error while searching categories block: {}", e);
				Vec::new()
			}
		};

		let input = input_arr.get(0).expect("no input");

		let search_array = search_query.split(" ");
		for item in search_array {
			input.send_keys(item).await?;
			sleep(Duration::from_secs(1)).await;
			input.send_keys(" ").await?;
		}

		sleep(Duration::from_secs(2)).await;

		Ok(())
	}

	pub async fn select_search_suggest(
		driver: WebDriver,
		select_suggest: bool,
	) -> Result<(), WebDriverError> {
		if select_suggest {
			let suggest_arr = match <dyn Crawler>::find_elements(
				driver.clone(),
				"//div[contains(@class, \"suggest-dropdownItems\")]/button[1]".to_string(),
				"//body/div[3]/div[2]/div/div/div/div/div/div".to_string(),
			)
			.await
			{
				Ok(res) => res,
				Err(e) => {
					println!("error while searching categories block: {}", e);
					Vec::new()
				}
			};

			let suggest = suggest_arr.get(0).expect("no suggest");
			// нажимаем на подсказку
			suggest.click().await?;
			sleep(Duration::from_secs(5)).await;
		} else {
			let submit_search_arr = match <dyn Crawler>::find_elements(
				driver.clone(),
				"//div[contains(@class, \"index-form\")]/div[last()]/button".to_string(),
				"//body/div[1]/div/buyer-location/div/div/div[2]/div/div[1]/div/div/div[3]/div[2]/div[2]/button".to_string(),
			)
			.await
			{
				Ok(res) => res,
				Err(e) => {
					println!("error while searching categories block: {}", e);
					Vec::new()
				}
			};

			let submit_search_button = submit_search_arr.get(0).expect("no suggest");
			// нажимаем на подсказку
			submit_search_button.click().await?;
			sleep(Duration::from_secs(5)).await;
		}

		Ok(())
	}
}
