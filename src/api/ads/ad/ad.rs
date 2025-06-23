use thirtyfour::prelude::*;

use crate::shared::Crawler;

pub trait AdsAd {}

impl dyn AdsAd {
	pub async fn get_seller_name_arr(
		driver: WebDriver,
	) -> Result<(String, String), WebDriverError> {
		//div[contains(@class, "style-seller-info-name")]//a
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

	//*[@data-marker="messenger-button/link"]//span[contains(@class, "styles-module-secondLine")]
	pub async fn get_answer_time(driver: WebDriver) -> Result<String, WebDriverError> {
		let rating_arr = match <dyn Crawler>::find_elements(
			driver.clone(),
			"//*[@data-marker=\"messenger-button/link\"]//span[contains(@class, \"styles-module-secondLine\")]".to_string(),
			"//*[text()[contains(.,'Отвечает за')]]".to_string(),
		)
		.await
		{
			Ok(res) => res,
			Err(e) => {
				println!("error while searching answer_time block: {}", e);
				Vec::new()
			}
		};

		let rating = match rating_arr.get(0) {
			Some(x) => x.text().await?,
			None => "".to_string(),
		};

		Ok(rating)
	}

	pub async fn get_seller_type(driver: WebDriver) -> Result<String, WebDriverError> {
		let seller_type_arr = match <dyn Crawler>::find_elements(
			driver.clone(),
			"//div[contains(@class, \"style-seller-info-col\")]//*[@data-marker=\"seller-info/label\"]".to_string(),
			"//body/div[1]/div/div[3]/div[1]/div/div[2]/div[3]/div/div[2]/div[1]/div/div/div[3]/div[2]/div/div/div/div[1]/div/div[1]/div[2]".to_string(),
		)
		.await
		{
			Ok(res) => res,
			Err(e) => {
				println!("error while searching seller_type block: {}", e);
				Vec::new()
			}
		};

		let seller_type = match seller_type_arr.get(0) {
			Some(x) => x.text().await?,
			None => "".to_string(),
		};

		Ok(seller_type.replace("·", "").replace("  ", ""))
	}

	//div[contains(@class, "style-seller-info-rating-score")]
	pub async fn get_rating(driver: WebDriver) -> Result<String, WebDriverError> {
		let rating_arr = match <dyn Crawler>::find_elements(
			driver.clone(),
			"//*[contains(@class, \"style-seller-info-rating-score\")]".to_string(),
			"//body/div[1]/div/div[3]/div[1]/div/div[2]/div[3]/div/div[2]/div[1]/div/div/div[4]/div/div/div/div/div[1]/div[1]/div/div/div[2]/span[1]".to_string(),
		)
		.await
		{
			Ok(res) => res,
			Err(e) => {
				println!("error while searching rating block: {}", e);
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
		let seller_info_redesign = match <dyn Crawler>::check_if_block_exists(
			driver.clone(),
			"//div[contains(@class, \"style-sellerInfoColRedesign\")]".to_string(),
			"".to_string(),
		)
		.await
		{
			Ok(elem) => elem,
			Err(e) => {
				println!("error while searching seller_info_redesign block: {}", e);
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

	pub async fn get_seller_closed_ads_count(driver: WebDriver) -> Result<String, WebDriverError> {
		let seller_ads_count_arr = match <dyn Crawler>::find_elements(
			driver.clone(),
			"//div[contains(@class, \"style-seller-info-col\")]/div[last()]".to_string(),
			"//body/div[1]/div/div[3]/div[1]/div/div[2]/div[3]/div/div[2]/div/div/div/div[3]/div[2]/div/div/div/div[1]/div/div[1]/div[2]".to_string(),
		)
		.await
		{
			Ok(res) => res,
			Err(e) => {
				println!("error while searching seller_closed_ads_count block: {}", e);
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

		let res = if seller_ads_count.contains("Завершено") {
			seller_ads_count.replace("Завершено", "")
		} else {
			"".to_string()
		};

		Ok(res)
	}

	//div[contains(@class, "style-seller-info-col")]//*[@data-marker="delivery/landing"]
	//*[@data-marker="delivery-item-button-main"]
	pub async fn get_delivery(driver: WebDriver) -> Result<String, WebDriverError> {
		// Описание
		let delivery_blocks_arr = match <dyn Crawler>::find_elements(
			driver.clone(),
			"//div[contains(@class, \"style-seller-info-col\")]//*[@data-marker=\"delivery/landing\"]".to_string(),
			"//*[@data-marker=\"delivery-item-button-main\"]".to_string(),
		)
		.await
		{
			Ok(res) => res,
			Err(e) => {
				println!("error while searching description block: {}", e);
				Vec::new()
			}
		};

		let delivery_block = match delivery_blocks_arr.get(0) {
			Some(x) => x
				.text()
				.await?
				.replace("Об", "")
				.replace("Авито", "")
				.replace("Доставке", "*")
				.replace("Купить", "")
				.replace("доставкой", "*")
				.replace("с", "")
				.replace(" ", ""),
			None => "".to_string(),
		};

		Ok(delivery_block)
	}

	pub async fn get_description(driver: WebDriver) -> Result<String, WebDriverError> {
		// Описание
		let description_blocks_arr = match <dyn Crawler>::find_elements(
			driver.clone(),
			"//div[contains(@class, \"style-item-description\")]/p".to_string(),
			"//body/div[1]/div/div[3]/div[1]/div/div[2]/div[3]/div/div[1]/div[2]/div[3]/div/div/p"
				.to_string(),
		)
		.await
		{
			Ok(res) => res,
			Err(e) => {
				println!("error while searching description block: {}", e);
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

	pub async fn get_images(driver: WebDriver) -> Result<String, WebDriverError> {
		let imgs_blocks_exists = match <dyn Crawler>::check_if_block_exists(
			driver.clone(),
			"//li[contains(@class, \"images-preview-previewImageWrapper\")]".to_string(),
			"".to_string(),
		)
		.await
		{
			Ok(elem) => elem,
			Err(e) => {
				println!("error while searching footer_article block: {}", e);
				false
			}
		};

		if imgs_blocks_exists {
			let imgs_blocks_arr = match <dyn Crawler>::find_elements(
				driver.clone(),
				"//li[contains(@class, \"images-preview-previewImageWrapper\")]".to_string(),
				"//body/div[1]/div/div[3]/div[1]/div/div[2]/div[3]/div/div[1]/div[2]/div[1]/div[1]/div/div/ul/li[1]".to_string(),
			)
			.await
			{
				Ok(res) => res,
				Err(e) => {
					println!("error while searching address block: {}", e);
					Vec::new()
				}
			};
			Ok(imgs_blocks_arr.len().to_string())
		} else {
			Ok(0.to_string())
		}
	}

	pub async fn get_date(
		driver: WebDriver,
	) -> Result<String, WebDriverError> {
		let date_arr = match <dyn Crawler>::find_elements(
			driver.clone(),
			"//*[@data-marker=\"item-view/item-date\"]".to_string(),
			"//body/div[1]/div/div[4]/div[1]/div/div[2]/div[3]/div/div[1]/div/div[2]/div[3]/div/div/div[1]/article/p/span[2]".to_string(),
		)
		.await
		{
			Ok(res) => res,
			Err(e) => {
				println!("error while searching date block: {}", e);
				Vec::new()
			}
		};

		let date = match date_arr.get(0) {
			Some(x) => x.text().await?.replace("На Авито ", "").replace("· ", ""),
			None => "".to_string(),
		};

		Ok(date)
	}

	pub async fn get_views_and_views_today(
		driver: WebDriver,
	) -> Result<(String, String), WebDriverError> {
		let views_arr = match <dyn Crawler>::find_elements(
			driver.clone(),
			"//*[@data-marker=\"item-view/total-views\"]".to_string(),
			"//body/div[1]/div/div[4]/div[1]/div/div[2]/div[3]/div/div[1]/div/div[2]/div[3]/div/div/div[1]/article/p/span[3]/span[1]".to_string(),
		)
		.await
		{
			Ok(res) => res,
			Err(e) => {
				println!("error while searching views_and_views_today block: {}", e);
				Vec::new()
			}
		};

		let views_total = match views_arr.get(0) {
			Some(x) => x
				.text()
				.await?
				.replace("просмотра", "")
				.replace("просмотров", "")
				.replace("просмотр", "")
				.replace("· ", "")
				.replace("  ", "")
				.replace(" ", "")
				.replace("&nbsp;", ""),
			None => "".to_string(),
		};

		let views_today_arr = match <dyn Crawler>::find_elements(
			driver.clone(),
			"//*[@data-marker=\"item-view/today-views\"]".to_string(),
			"//body/div[1]/div/div[4]/div[1]/div/div[2]/div[3]/div/div[1]/div/div[2]/div[3]/div/div/div[1]/article/p/span[3]/span[2]".to_string(),
		)
		.await
		{
			Ok(res) => res,
			Err(e) => {
				println!("error while searching views_and_views_today block: {}", e);
				Vec::new()
			}
		};

		let views_today = match views_today_arr.get(0) {
			Some(x) => x
				.text()
				.await?
				.replace("(", "")
				.replace("+", "")
				.replace(" ", "")
				.replace("  ", "")
				.replace("сегодня", "")
				.replace(")", "")
				.replace("&nbsp;", ""),
			None => "".to_string(),
		};

		Ok((views_total, views_today))
	}
}
