use chrono::{DateTime, Utc};
use csv::Writer;
use std::env;
use thirtyfour::prelude::*;
use tokio::time::{sleep, Duration};

use crate::api::{Feed, AdsAd, Header, Settings};
use crate::shared::{Constants, Crawler, Driver, Firewall};

#[allow(unreachable_code)]
pub async fn ads_crawler() -> WebDriverResult<()> {
	let search_env = env::var("SEARCH_QUERY").expect("SEARCH_QUERY not set");
	let search_query = search_env.as_str();
	let city_env = env::var("CITY_QUERY").expect("CITY_QUERY not set");
	let city_query = city_env.as_str();
	let url = env::var("URL_QUERY").expect("URL_QUERY not set");
	let select_suggest = env::var("SELECT_SUGGEST").expect("SELECT_SUGGEST not set");
	let fullscreen_mode = env::var("FULLSCREEN_MODE").expect("FULLSCREEN_MODE not set");
	let accaunts_to_check_str = env::var("ACCAUNTS_TO_CHECK").unwrap_or("".to_string());
	let ads_to_check_str = env::var("ADS_TO_CHECK").unwrap_or("".to_string());
	let collect_phone = env::var("COLLECT_PHONE").expect("COLLECT_PHONE not set");
	let visit_ads_page = env::var("VISIT_ADS_PAGE").expect("VISIT_ADS_PAGE not set");

	let accaunts_to_check = if accaunts_to_check_str != "" {
		accaunts_to_check_str.split(" ").collect::<Vec<&str>>()
	} else {
		Vec::new()
	};

	let ads_to_check = if ads_to_check_str != "" {
		ads_to_check_str.split(" ").collect::<Vec<&str>>()
	} else {
		Vec::new()
	};

	let utc: DateTime<Utc> = Utc::now() + chrono::Duration::try_hours(3).expect("hours err");

	let mut wtr = Writer::from_path(format!(
		"./output/ads_{}_{}_{}.csv",
		utc.format("%d-%m-%Y_%H-%M-%S"),
		search_query.replace(" ", "_"),
		city_query.replace(" ", "_")
	))
	.expect("no file");

	let headers = <dyn Constants>::get_ads_crawler_table_headers();

	wtr.write_record(&headers).expect("write record err");

	let driver = <dyn Driver>::get_driver().await?;

	if fullscreen_mode.parse().unwrap() {
		driver.maximize_window().await?;
	}

	driver.goto(url).await?;

	sleep(Duration::from_secs(2)).await;

	let _ = <dyn Settings>::click_open_geo_modal_btn(driver.clone()).await?;
	let _ = <dyn Settings>::click_clear_btn(driver.clone()).await?;
	let _ = <dyn Settings>::write_region_input(driver.clone(), city_query).await?;
	let _ = <dyn Settings>::click_region_suggest(driver.clone()).await?;
	let _ = <dyn Settings>::click_geo_confirm(driver.clone()).await?;
	let _ = <dyn Settings>::write_search_input(driver.clone(), search_query).await?;
	let _ = <dyn Settings>::select_search_suggest(
		driver.clone(),
		select_suggest.parse().unwrap_or(1),
		false,
	)
	.await?;

	let categories = <dyn Header>::get_categories(driver.clone()).await?;
	let ads_count = <dyn Header>::get_ads_count(driver.clone()).await?;

	println!("Start {}", utc.format("%d-%m-%Y_%H:%M:%S"));
	println!("City: {}", &city_query);
	println!("Query: {}", &search_query);
	println!("ads_count: {}", ads_count.clone());

	let mut position;

	let ads_count_res = if ads_count > 50.0 {
		(ads_count / 50.0).ceil() as i32
	} else {
		ads_count.ceil() as i32
	};

	'outer: for j in 0..=ads_count_res {
		// feed
		//scroll

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
			//div[contains(@class, "items-items")]/div[contains(@class, "iva-item-root")][1]//*[@data-marker="item-title"]
			let title = <dyn Feed>::get_text(driver.clone(),
				format!("//div[contains(@class, \"items-items\")]/div[contains(@class, \"iva-item-root\")][{}]//*[@data-marker=\"item-title\"]", count),
				format!("//body/div[1]/div/buyer-location/div/div/div[2]/div/div[2]/div[3]/div[3]/div[3]/div[2]/div[contains(@class, \"iva-item-root\")][{}]//*[@data-marker=\"item-title\"]", count)
			).await?;

			let price = <dyn Feed>::get_price(driver.clone(),
				format!("//div[contains(@class, \"items-items\")]/div[contains(@class, \"iva-item-root\")][{}]//*[@data-marker=\"item-price\"]/meta[2]", count),
				format!("//body/div[1]/div/buyer-location/div/div/div[2]/div/div[2]/div[3]/div[3]/div[3]/div[2]/div[contains(@class, \"iva-item-root\")][{}]/div/div/div[2]/div[3]/span/div/p/meta[2]", count)
			).await?;
			//div[contains(@class, "items-items")]/div[contains(@class, "iva-item-root")][2]/div/div/div//*[contains(@class, "iva-item-dateInfoStep")]//i
			let _ = <dyn Feed>::move_mouse_to_paid(driver.clone(),
				format!("//div[contains(@class, \"items-items\")]/div[contains(@class, \"iva-item-root\")][{}]/div/div/div//*[contains(@class, \"iva-item-dateInfoStep\")]//i", count),
			).await?;
			//div[contains(@class, "styles-entry")]/i[contains(@class, "style-vas-icon")]/img
			let paid_imgs = <dyn Feed>::get_paid_imgs(driver.clone(),
				"//div[contains(@class, \"styles-entry\")]/i[contains(@class, \"style-vas-icon\")]/img".to_string(),
				"".to_string()
			).await?;

			let mut paid_types = Vec::new();

			for (p, _) in paid_imgs.clone().into_iter().enumerate() {
				let piad_img_count = p + 1;

				let img = <dyn Feed>::get_paid_img(driver.clone(), format!("//div[contains(@class, \"styles-entry\")][{}]/i[contains(@class, \"style-vas-icon\")]/img", piad_img_count), "".to_string()).await?;
				paid_types.push(img);
			}

			let paid = paid_types.join(", ");

			if !visit_ads_page.parse::<bool>().unwrap() {
				let mut my_ads = "";

				if ads_to_check.len() > 0 {
					for ad in &ads_to_check {
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
					city_query,
					search_query,
					position.to_string().as_str(),
					"",
					"",
					paid.to_string().as_str(),
					"",
					id,
					title.replace("\"", "").as_str(),
					price.as_str(),
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
					"",
					"",
				])
				.expect("write record err");
			} else {
				// Переход в новую вкладку
				let handle = driver.window().await?;

				let _ = <dyn Feed>::click_ad_title_link(driver.clone(),
					format!("//div[contains(@class, \"items-items\")]/div[contains(@class, \"iva-item-root\")][{}]//*[@data-marker=\"item-title\"]", count),
					format!("//body/div[1]/div/buyer-location/div/div/div[2]/div/div[2]/div[3]/div[3]/div[3]/div[2]/div[contains(@class, \"iva-item-root\")][{}]//*[@data-marker=\"item-title\"]", count)
				).await?;

				let handles = driver.windows().await?;
				driver.switch_to_window(handles[1].clone()).await?;
				sleep(Duration::from_secs(5)).await;

				let firewall_msg_in_page = <dyn Firewall>::get_firewall(driver.clone()).await?;

				if firewall_msg_in_page {
					'firewall_in_page: for _ in 0..=3600 {
						println!("====== firewall ======");
						sleep(Duration::from_secs(30)).await;

						let firewall_msg_in_loop_in_page =
							<dyn Firewall>::get_firewall(driver.clone()).await?;

						if !firewall_msg_in_loop_in_page {
							break 'firewall_in_page;
						}
					}
				}

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
				let footer_article = <dyn AdsAd>::check_footer_article(driver.clone()).await?;
				let date = <dyn AdsAd>::get_date(driver.clone(), footer_article).await?;
				let (views, views_today) =
					<dyn AdsAd>::get_views_and_views_today(driver.clone(), footer_article).await?;
				let imgs_count = <dyn AdsAd>::get_images(driver.clone()).await?;
				let phone = <dyn AdsAd>::get_phone(driver.clone(), collect_phone.parse().unwrap()).await?;

				driver.close_window().await?;
				driver.switch_to_window(handle.clone()).await?;
				sleep(Duration::from_secs(2)).await;

				let mut my_ads = "";

				if accaunts_to_check.len() > 0 {
					for account in &accaunts_to_check {
						if seller_id.contains(account) {
							my_ads = "*";
						};
					}
				}

				if ads_to_check.len() > 0 {
					for ad in &ads_to_check {
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
					city_query,
					search_query,
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
					imgs_count.as_str(),
					phone.as_str(),
				])
				.expect("write record err");
			}
		}

		// пагинация
		let _ = <dyn Feed>::click_pagination_next_btn(driver.clone()).await?;

		println!("{}", "======");
	}
	wtr.flush()?;
	driver.clone().quit().await?;

	Ok(())
}
