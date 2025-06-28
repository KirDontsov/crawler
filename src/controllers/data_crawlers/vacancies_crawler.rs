use chrono::{DateTime, Utc};
use csv::Writer;
use tokio::time::{sleep, Duration};
use std::fs;

use crate::api::{AdsAd, Feed};
use crate::shared::{Constants, Crawler};
use crate::config::CrawlerConfig;
use crate::error::CrawlerError;
use crate::controllers::data_crawlers::common::*;

#[allow(unreachable_code)]
pub async fn vacancies_crawler() -> Result<(), CrawlerError> {
	let config = CrawlerConfig::from_env()?;

	let utc: DateTime<Utc> = Utc::now() + chrono::Duration::try_hours(3).expect("hours err");

	fs::create_dir_all(format!("./output{}", &config.report_directory))?;

	let mut wtr = Writer::from_path(format!(
		"./output{}/ads_{}_{}_{}.csv",
		&config.report_directory,
		utc.format("%d-%m-%Y_%H-%M-%S"),
		&config.search_query.replace(" ", "_"),
		&config.city_query.replace(" ", "_")
	))
	.expect("no file");

	let headers = <dyn Constants>::get_vacancies_crawler_table_headers();

	wtr.write_record(&headers).expect("write record err");

	let driver = initialize_crawler(&config).await?;

	let (categories, ads_count) = get_search_metadata(&driver).await?;

	println!("Start {}", utc.format("%d-%m-%Y_%H:%M:%S"));
	println!("City: {}", &config.city_query);
	println!("Query: {}", &config.search_query);
	println!("ads_count {}", ads_count.clone());

	let mut position;

	let ads_count_res = calculate_pagination(ads_count);

	'outer: for j in 0..=ads_count_res {
		// feed
		//scroll

		handle_firewall(&driver).await?;

		let blocks = <dyn Feed>::get_feed(driver.clone()).await?;

		if blocks.len() == 0 {
			println!("====== break ======");
			break 'outer;
		}

		let last = blocks.last().expect("no blocks");
		last.scroll_into_view().await?;

		for (i, block) in blocks.clone().into_iter().enumerate() {
			let count = i + 1;

			position = count - 1;
			if j != 0 {
				position = (50 * j) as usize + count as usize - 1;
			}

			// проверяем есть ли рекламное объявление
			let ads_banner_exists = match <dyn Crawler>::check_if_block_exists(
				driver.clone(),
				"//div[contains(@class, \"items-banner\")]".to_string(),
				"".to_string(),
			)
			.await
			{
				Ok(elem) => elem,
				Err(e) => {
					println!("error while searching ads_banner_arr block: {}", e);
					false
				}
			};

			if ads_banner_exists {
				if count == 1 {
					continue;
				}
			};

			block.scroll_into_view().await?;

			let href = <dyn Feed>::get_href(driver.clone(),
				format!("//div[contains(@class, \"items-items\")]/div[contains(@class, \"iva-item-root\")][{}]//*[@data-marker=\"item-title\"]", count),
				format!("//body/div[1]/div/buyer-location/div/div/div[2]/div/div[2]/div[3]/div[3]/div[3]/div[2]/div[contains(@class, \"iva-item-root\")][{}]//*[@data-marker=\"item-title\"]", count)
			).await?;

			let id = href.split("_").last().expect("no href");

			let title = <dyn Feed>::get_text(driver.clone(),
				format!("//div[contains(@class, \"items-items\")]/div[contains(@class, \"iva-item-root\")][{}]//*[@data-marker=\"item-title\"]", count),
				format!("//body/div[1]/div/buyer-location/div/div/div[2]/div/div[2]/div[3]/div[3]/div[3]/div[2]/div[contains(@class, \"iva-item-root\")][{}]//*[@data-marker=\"item-title\"]", count)
			).await?;

			let price = <dyn Feed>::get_price(driver.clone(),
				format!("//div[contains(@class, \"items-items\")]/div[contains(@class, \"iva-item-root\")][{}]//*[@data-marker=\"item-price\"]/meta[2]", count),
				format!("//body/div[1]/div/buyer-location/div/div/div[2]/div/div[2]/div[3]/div[3]/div[3]/div[2]/div[contains(@class, \"iva-item-root\")][{}]/div/div/div[2]/div[3]/span/div/p/meta[2]", count)
			).await?;

			let _ = <dyn Feed>::move_mouse_to_paid(driver.clone(),
				format!("//div[contains(@class, \"items-items\")]/div[contains(@class, \"iva-item-root\")][{}]/div/div/div//*[contains(@class, \"iva-item-dateInfoStep\")]//i", count),
			).await?;

			let paid_imgs = <dyn Feed>::get_paid_imgs(driver.clone(),
				"//div[contains(@class, \"styles-entry\")]/i[contains(@class, \"style-vas-icon\")]/img".to_string(),
				"".to_string()
			).await?;

			let mut paid_types = Vec::new();

			for (p, _) in paid_imgs.clone().into_iter().enumerate() {
				let piad_img_count = p + 1;
				// поповер в портале
				let img = <dyn Feed>::get_paid_img(driver.clone(), format!("//div[contains(@class, \"styles-entry\")][{}]/i[contains(@class, \"style-vas-icon\")]/img", piad_img_count), "".to_string()).await?;
				paid_types.push(img);
			}

			let paid = paid_types.join(", ");

			if !config.visit_ads_page {
				let mut my_ads = "";

				if config.ads_to_check.len() > 0 {
					for ad in &config.ads_to_check {
						if id.contains(ad) {
							my_ads = "*";
						};
					}
				}

				println!(
					"{} из {} - {} {}",
					&position,
					&ads_count.clone(),
					&id,
					&my_ads
				);

				// === RESULT ===

				wtr.write_record(&[
					my_ads,
					format!("{}", utc.format("%d-%m-%Y_%H:%M:%S")).as_str(),
					&config.city_query,
					&config.search_query,
					position.to_string().as_str(),
					"",
					"",
					paid.to_string().as_str(),
					"",
					id,
					title.replace("\"", "").as_str(),
					"",
					href.as_str(),
					categories.as_str(),
					"",
					"",
					"",
					"",
					"",
					"",
					"",
					"",
					"",
					"",
					"",
				])
				.expect("write record err");

			} else {
				// Переход в новую вкладку
				let handle = driver.window().await?;

				let _ = <dyn Feed>::click_ad_title_link(driver.clone(),
					format!("//div[contains(@class, \"items-items\")]/div[contains(@class, \"iva-item-root\")][{}]//*[@data-marker=\"item-title\"]", count),
					format!("//body/div[1]/div/buyer-location/div/div/div[2]/div/div[2]/div[3]/div[3]/div[3]/div[2]/div[contains(@class, \"iva-item-root\")][{}]/div/div/div/div[2]/div/a", count)
				).await?;

				let handles = driver.windows().await?;
				driver.switch_to_window(handles[1].clone()).await?;
				sleep(Duration::from_secs(5)).await;

				handle_firewall(&driver).await?;

				let (seller_id, seller_name) =
					<dyn AdsAd>::get_seller_name_arr(driver.clone()).await?;
				let seller_type = <dyn AdsAd>::get_seller_type(driver.clone()).await?;
				let answer_time = <dyn AdsAd>::get_answer_time(driver.clone()).await?;
				let rating = <dyn AdsAd>::get_rating(driver.clone()).await?;
				let reviews = <dyn AdsAd>::get_reviews(driver.clone()).await?;
				let register_date = <dyn AdsAd>::get_register_date(driver.clone()).await?;
				let seller_ads_count = <dyn AdsAd>::get_seller_ads_count(driver.clone()).await?;
				let seller_closed_ads_count =
					<dyn AdsAd>::get_seller_closed_ads_count(driver.clone()).await?;
				let description_string = <dyn AdsAd>::get_description(driver.clone()).await?;
				let address = <dyn AdsAd>::get_address(driver.clone()).await?;
				let date = <dyn AdsAd>::get_date(driver.clone()).await?;
				let (views, views_today) =
					<dyn AdsAd>::get_views_and_views_today(driver.clone()).await?;

				driver.close_window().await?;
				driver.switch_to_window(handle.clone()).await?;
				sleep(Duration::from_secs(2)).await;

				let mut my_ads = "";

				if config.ads_to_check.len() > 0 {
					for ad in &config.ads_to_check {
						if id.contains(ad) {
							my_ads = "*";
						};
					}
				}

				println!(
					"{} из {} - {} {}",
					&position,
					&ads_count.clone(),
					&id,
					&my_ads
				);

				// === RESULT ===

				wtr.write_record(&[
					my_ads,
					format!("{}", utc.format("%d-%m-%Y_%H:%M:%S")).as_str(),
					&config.city_query,
					&config.search_query,
					position.to_string().as_str(),
					views.as_str(),
					views_today.as_str(),
					paid.to_string().as_str(),
					date.as_str(),
					id,
					title.replace("\"", "").as_str(),
					price.as_str(),
					href.as_str(),
					categories.as_str(),
					seller_id.as_str(),
					seller_name.as_str(),
					seller_type.as_str(),
					register_date.as_str(),
					answer_time.as_str(),
					rating.as_str(),
					reviews.as_str(),
					seller_ads_count.as_str(),
					seller_closed_ads_count.as_str(),
					address.as_str(),
					description_string.as_str(),
				])
				.expect("write record err");
			}
		}

		// пагинация
		let _ = <dyn Feed>::click_pagination_next_btn(driver.clone()).await?;

		println!("{}", "======");
	}

	wtr.flush()?;

	Ok(())
}
