use thirtyfour::prelude::*;

use crate::api::Crawler;

pub trait Ad {}

impl dyn Ad {
	pub async fn get_seller_name_arr(
		driver: WebDriver,
	) -> Result<(String, String), WebDriverError> {
		let seller_name_arr = match <dyn Crawler>::find_elements(
			driver.clone(),
			"//div[contains(@class, \"style-seller-info-name\")]//a".to_string(),
			"//body/div[1]/div/div[3]/div[1]/div/div[2]/div[3]/div/div[2]/div[1]/div/div/div[3]/div[2]/div/div/div/div[1]/div[1]/div[1]/div[1]/div[1]//a".to_string(),
		)
		.await
		{
			Ok(res) => res,
			Err(e) => {
				println!("error while searching seller_name block: {}", e);
				driver.clone().quit().await?;
				Vec::new()
			}
		};

		let seller_link_id_full = match seller_name_arr.get(0) {
			Some(x) => x.attr("href").await?.expect("no seller_link_id"),
			None => "".to_string(),
		};

		let seller_id = match seller_link_id_full.split("?").collect::<Vec<&str>>().get(0) {
			Some(x) => {
				if x.contains("avito.ru") {
					x.to_owned().to_string()
				} else if *x != "" {
					format!("https://avito.ru{}", x)
				} else {
					"".to_string()
				}
			}
			None => "".to_string(),
		};

		let seller_name = match seller_name_arr.get(0) {
			Some(x) => x.text().await?,
			None => "".to_string(),
		};

		Ok((seller_id.to_string(), seller_name))
	}

	pub async fn get_rating(driver: WebDriver) -> Result<String, WebDriverError> {
		let rating_arr = match <dyn Crawler>::find_elements(
			driver.clone(),
			"//div[contains(@class, \"style-seller-info-rating-score\")]".to_string(),
			"//body/div[1]/div/div[3]/div[1]/div/div[2]/div[3]/div/div[2]/div[1]/div/div/div[3]/div[2]/div/div/div/div[1]/div[1]/div[1]/div[1]/div[2]/span[1]".to_string(),
		)
		.await
		{
			Ok(res) => res,
			Err(e) => {
				println!("error while searching rating block: {}", e);
				driver.clone().quit().await?;
				Vec::new()
			}
		};

		let rating = match rating_arr.get(0) {
			Some(x) => x.text().await?,
			None => "".to_string(),
		};

		Ok(rating)
	}

	pub async fn get_reviews(driver: WebDriver) -> Result<String, WebDriverError> {
		let reviews_arr = match <dyn Crawler>::find_elements(
			driver.clone(),
			"//div[contains(@class, \"style-seller-info-rating\")]/a".to_string(),
			"//body/div[1]/div/div[3]/div[1]/div/div[2]/div[3]/div/div[2]/div[1]/div/div/div[3]/div[2]/div/div/div/div[1]/div[1]/div[1]/div[1]/div[2]/a".to_string(),
		)
		.await
		{
			Ok(res) => res,
			Err(e) => {
				println!("error while searching reviews block: {}", e);
				Vec::new()
			}
		};

		let reviews = match reviews_arr.get(0) {
			Some(x) => x
				.text()
				.await?
				.replace("отзывов", "")
				.replace("отзыва", "")
				.replace("отзыв", "")
				.replace(" ", ""),
			None => "".to_string(),
		};

		Ok(reviews)
	}

	pub async fn get_register_date(driver: WebDriver) -> Result<String, WebDriverError> {
		// проверяем есть ли рекламное объявление
		let seller_info_redesign = match <dyn Crawler>::check_if_block_exists(driver.clone(),
			"//div[contains(@class, \"style-sellerInfoColRedesign\")]".to_string(),
			"//body/div[1]/div/div[3]/div[1]/div/div[2]/div[3]/div/div[2]/div/div/div/div[3]/div[2]/div/div/div/div[1]/div/div[1]".to_string()
		).await {
			Ok(elem) => elem,
			Err(e) => {
				println!("error while searching ads_banner_arr block: {}", e);
				false
			}
		};

		let register_date_arr;

		if !seller_info_redesign {
			register_date_arr = match <dyn Crawler>::find_elements(
				driver.clone(),
				"//div[contains(@class, \"style-seller-info-value\")][last()]/div[last()]".to_string(),
				"//body/div[1]/div/div[3]/div[1]/div/div[2]/div[3]/div/div[2]/div[1]/div/div/div[3]/div[2]/div/div/div/div[1]/div[1]/div[1]/div[3]/div".to_string(),
			)
			.await
			{
				Ok(res) => res,
				Err(e) => {
					println!("error while searching register_date block: {}", e);
					Vec::new()
				}
			};
		} else {
			register_date_arr = match <dyn Crawler>::find_elements(
				driver.clone(),
				"//div[contains(@class, \"style-sellerInfoColRedesign\")]/p".to_string(),
				"//body/div[1]/div/div[3]/div[1]/div/div[2]/div[3]/div/div[2]/div/div/div/div[3]/div[2]/div/div/div/div[1]/div/div[1]/p".to_string(),
			)
			.await
			{
				Ok(res) => res,
				Err(e) => {
					println!("error while searching register_date block: {}", e);
					Vec::new()
				}
			};
		}

		let register_date_text = match register_date_arr.get(0) {
			Some(x) => x.text().await?,
			None => "".to_string(),
		};

		let register_date = if register_date_text.contains("Пользователь")
			|| register_date_text.contains("отзыв")
			|| register_date_text.contains("Завершено")
		{
			"".to_string()
		} else {
			register_date_text.replace("На Авито ", "")
		};

		Ok(register_date)
	}

	pub async fn get_seller_ads_count(driver: WebDriver) -> Result<String, WebDriverError> {
		let seller_ads_count_arr = match <dyn Crawler>::find_elements(
			driver.clone(),
			"//div[contains(@class, \"style-seller-info-favorite-seller-buttons\")]/div/div/div[1]/a".to_string(),
			"//body/div[1]/div/div[3]/div[1]/div/div[2]/div[3]/div/div[2]/div[1]/div/div/div[3]/div[2]/div/div/div/div[2]/div/div/div/div/a".to_string(),
		)
		.await
		{
			Ok(res) => res,
			Err(e) => {
				println!("error while searching seller_ads_count block: {}", e);
				Vec::new()
			}
		};

		let seller_ads_count = match seller_ads_count_arr.get(0) {
			Some(x) => x
				.text()
				.await?
				.replace("пользователя", "")
				.replace("объявление", "")
				.replace("объявления", "")
				.replace("объявлений", "")
				.replace(" ", ""),
			None => "".to_string(),
		};

		Ok(seller_ads_count)
	}

	pub async fn get_description(driver: WebDriver) -> Result<String, WebDriverError> {
		// Описание
		let description_blocks_arr = match <dyn Crawler>::find_elements(
			driver.clone(),
			"//div[contains(@class, \"style-item-description-html\")]/p".to_string(),
			"//body/div[1]/div/div[3]/div[1]/div/div[2]/div[3]/div/div[1]/div[2]/div[3]/div/div/p"
				.to_string(),
		)
		.await
		{
			Ok(res) => res,
			Err(e) => {
				println!("error while searching seller_ads_count block: {}", e);
				Vec::new()
			}
		};

		let mut descriptions_arr: Vec<String> = Vec::new();

		for desc_block in description_blocks_arr {
			desc_block.scroll_into_view().await?;

			let description = desc_block.text().await?;
			descriptions_arr.push(description);
		}

		let description_string = descriptions_arr.join(" ");

		Ok(description_string)
	}

	pub async fn get_address(driver: WebDriver) -> Result<String, WebDriverError> {
		let address_arr = match <dyn Crawler>::find_elements(
			driver.clone(),
			"//span[contains(@class, \"style-item-address__string\")]".to_string(),
			"//body/div[1]/div/div[3]/div[1]/div/div[2]/div[3]/div/div[1]/div[1]/div[3]/div/div/div[1]/div[1]/div/p[1]/span".to_string(),
		)
		.await
		{
			Ok(res) => res,
			Err(e) => {
				println!("error while searching address block: {}", e);
				Vec::new()
			}
		};

		let address = match address_arr.get(0) {
			Some(x) => x.text().await?,
			None => "".to_string(),
		};

		Ok(address)
	}

	pub async fn check_footer_article(driver: WebDriver) -> Result<bool, WebDriverError> {
		let footer_article = match <dyn Crawler>::check_if_block_exists(driver.clone(),
			"//article[contains(@class, \"style-item-footer-text\")]".to_string(),
			"//body/div[1]/div/div[3]/div[1]/div/div[2]/div[3]/div/div[1]/div[1]/div[5]/div/div/div[1]/article".to_string()
		).await {
			Ok(elem) => elem,
			Err(e) => {
				println!("error while searching ads_banner_arr block: {}", e);
				false
			}
		};

		Ok(footer_article)
	}

	pub async fn get_date(
		driver: WebDriver,
		footer_article: bool,
	) -> Result<String, WebDriverError> {
		let date;

		if !footer_article {
			let date_arr = match <dyn Crawler>::find_elements(
				driver.clone(),
				"//div[contains(@class, \"style-item-footer-text\")]/article/p/span[2]".to_string(),
				"//body/div[1]/div/div[3]/div[1]/div/div[2]/div[3]/div/div[1]/div[2]/div[4]/div/article/p/span[2]".to_string(),
			)
			.await
			{
				Ok(res) => res,
				Err(e) => {
					println!("error while searching date block: {}", e);
					Vec::new()
				}
			};

			date = match date_arr.get(0) {
				Some(x) => x.text().await?.replace("На Авито ", "").replace("· ", ""),
				None => "".to_string(),
			};
		} else {
			let date_arr = match <dyn Crawler>::find_elements(
				driver.clone(),
				"//article[contains(@class, \"style-item-footer-text\")]/p/span[2]".to_string(),
				"//body/div[1]/div/div[3]/div[1]/div/div[2]/div[3]/div/div[1]/div[1]/div[5]/div/div/div[1]/article/p/span[2]".to_string(),
			)
			.await
			{
				Ok(res) => res,
				Err(e) => {
					println!("error while searching date block: {}", e);
					Vec::new()
				}
			};

			date = match date_arr.get(0) {
				Some(x) => x.text().await?.replace("На Авито ", "").replace("· ", ""),
				None => "".to_string(),
			};
		}

		Ok(date)
	}

	pub async fn get_views_and_views_today(
		driver: WebDriver,
		footer_article: bool,
	) -> Result<(String, String), WebDriverError> {
		let views;
		let views_today;

		if !footer_article {
			let views_arr = match <dyn Crawler>::find_elements(
				driver.clone(),
				"//div[contains(@class, \"style-item-footer-text\")]/article/p/span[3]".to_string(),
				"//body/div[1]/div/div[3]/div[1]/div/div[2]/div[3]/div/div[1]/div[2]/div[4]/div/article/p/span[3]".to_string(),
			)
			.await
			{
				Ok(res) => res,
				Err(e) => {
					println!("error while searching views block: {}", e);
					Vec::new()
				}
			};

			let views_full = match views_arr.get(0) {
				Some(x) => x
					.text()
					.await?
					.replace("просмотра", "")
					.replace("просмотров", "")
					.replace("просмотр", "")
					.replace("· ", "")
					.replace("  ", ""),
				None => "".to_string(),
			};

			let views_str = views_full.split("+").collect::<Vec<&str>>();
			views = match views_str.get(0) {
				Some(x) => x.replace("(", "").replace("  ", ""),
				None => "".to_string(),
			};

			views_today = match views_str.get(1) {
				Some(x) => x
					.replace(")", "")
					.replace("+", "")
					.replace("сегодня", "")
					.replace(" ", ""),
				None => "".to_string(),
			};
		} else {
			let views_arr = match <dyn Crawler>::find_elements(
				driver.clone(),
				"//article[contains(@class, \"style-item-footer-text\")]/p/span[3]/span[1]".to_string(),
				"//body/div[1]/div/div[3]/div[1]/div/div[2]/div[3]/div/div[1]/div[1]/div[5]/div/div/div[1]/article/p/span[3]/span[1]".to_string(),
			)
			.await
			{
				Ok(res) => res,
				Err(e) => {
					println!("error while searching views block: {}", e);
					Vec::new()
				}
			};

			views = match views_arr.get(0) {
				Some(x) => x
					.text()
					.await?
					.replace("просмотра", "")
					.replace("просмотров", "")
					.replace("просмотр", "")
					.replace("· ", "")
					.replace("  ", ""),
				None => "".to_string(),
			};

			let views_today_arr = match <dyn Crawler>::find_elements(
				driver.clone(),
				"//article[contains(@class, \"style-item-footer-text\")]/p/span[3]/span[2]".to_string(),
				"//body/div[1]/div/div[3]/div[1]/div/div[2]/div[3]/div/div[1]/div[1]/div[5]/div/div/div[1]/article/p/span[3]/span[2]".to_string(),
			)
			.await
			{
				Ok(res) => res,
				Err(e) => {
					println!("error while searching views block: {}", e);
					Vec::new()
				}
			};

			views_today = match views_today_arr.get(0) {
				Some(x) => x
					.text()
					.await?
					.replace("(", "")
					.replace(")", "")
					.replace("+", "")
					.replace("сегодня", "")
					.replace(" ", ""),
				None => "".to_string(),
			};
		}

		Ok((views, views_today))
	}
}
