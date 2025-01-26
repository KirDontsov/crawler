use thirtyfour::prelude::*;

pub trait Crawler {}

impl dyn Crawler {
	pub async fn find_elements(
		driver: WebDriver,
		xpath: String,
		xpath2: String,
	) -> Result<Vec<WebElement>, WebDriverError> {
		let el_exists = match Self::check_if_block_exists(
			driver.clone(),
			xpath.clone(),
			xpath2.clone(),
		)
		.await
		{
			Ok(elem) => elem,
			Err(e) => {
				println!("error while searching ads_banner_arr block: {}", e);
				driver.clone().quit().await?;
				false
			}
		};

		if el_exists {
			let elems = driver
				.query(By::XPath(&xpath))
				.or(By::XPath(&xpath2))
				.nowait()
				.all_from_selector_required()
				.await?;

			Ok(elems)
		} else {
			Ok(Vec::new())
		}
	}

	pub async fn check_if_block_exists(
		driver: WebDriver,
		xpath: String,
		xpath2: String,
	) -> Result<bool, WebDriverError> {
		let exists;

		if xpath2 != "" {
			exists = driver
				.query(By::XPath(&xpath))
				.or(By::XPath(&xpath2))
				.nowait()
				.exists()
				.await?;
		} else {
			exists = driver.query(By::XPath(&xpath)).nowait().exists().await?;
		}

		Ok(exists)
	}

	pub async fn find_attr(
		driver: WebDriver,
		xpath: String,
		xpath2: String,
		attr: String,
	) -> Result<String, WebDriverError> {
		let el_exists = match Self::check_if_block_exists(
			driver.clone(),
			xpath.clone(),
			xpath2.clone(),
		)
		.await
		{
			Ok(elem) => elem,
			Err(e) => {
				println!("error while searching find_text block: {}", e);
				driver.clone().quit().await?;
				false
			}
		};

		if el_exists {
			let elem = match driver
				.query(By::XPath(&xpath))
				.or(By::XPath(&xpath2))
				.nowait()
				.first()
				.await?
				.attr(attr)
				.await? {
					Some(x) => x,
					None => "".to_string()
				};

			Ok(elem)
		} else {
			Ok("".to_string())
		}
	}

	pub async fn find_text(
		driver: WebDriver,
		xpath: String,
		xpath2: String,
	) -> Result<String, WebDriverError> {
		let el_exists = match Self::check_if_block_exists(
			driver.clone(),
			xpath.clone(),
			xpath2.clone(),
		)
		.await
		{
			Ok(elem) => elem,
			Err(e) => {
				println!("error while searching find_text block: {}", e);
				driver.clone().quit().await?;
				false
			}
		};

		if el_exists {
			let elem = driver
				.query(By::XPath(&xpath))
				.or(By::XPath(&xpath2))
				.nowait()
				.first()
				.await?
				.text()
				.await?;

			Ok(elem)
		} else {
			Ok("".to_string())
		}
	}
}
