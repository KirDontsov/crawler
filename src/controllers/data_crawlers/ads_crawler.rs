use chrono::{DateTime, Utc};
use csv::Writer;
use std::env;
use thirtyfour::prelude::*;
use tokio::time::{sleep, Duration};

use crate::api::{AdsAd, Header, Settings};
use crate::api::Feed;
use crate::shared::{Driver, Firewall, Crawler, Constants};

#[allow(unreachable_code)]
pub async fn ads_crawler() -> WebDriverResult<()> {
	let search_env = env::var("SEARCH_QUERY").expect("SEARCH_QUERY not set");
	let search_query = search_env.as_str();
	let city_env = env::var("CITY_QUERY").expect("CITY_QUERY not set");
	let city_query = city_env.as_str();
	let url = env::var("URL_QUERY").expect("URL_QUERY not set");
	let select_suggest = env::var("SELECT_SUGGEST").expect("SELECT_SUGGEST not set");
	let fullscreen_mode = env::var("FULLSCREEN_MODE").expect("FULLSCREEN_MODE not set");

	let utc: DateTime<Utc> = Utc::now() + chrono::Duration::try_hours(3).expect("hours err");

	let mut wtr = Writer::from_path(format!(
		"./output/ads_{}_{}_{}.csv",
		utc.format("%d-%m-%Y_%H:%M:%S"),
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
		select_suggest.parse().unwrap_or(true),
		false
	)
	.await?;

	let categories = <dyn Header>::get_categories(driver.clone()).await?;
	let ads_count = <dyn Header>::get_ads_count(driver.clone()).await?;

	println!("Start {}", utc.format("%d-%m-%Y_%H:%M:%S"));
	println!("ads_count {}", ads_count.clone());

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
					driver.clone().quit().await?;
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
				format!("//div[contains(@class, \"items-items\")]/div[contains(@class, \"iva-item-root\")][{}]/div/div/div[2]/div[2]/div/a", count),
				format!("//body/div[1]/div/buyer-location/div/div/div[2]/div/div[2]/div[3]/div[3]/div[3]/div[2]/div[contains(@class, \"iva-item-root\")][{}]/div/div/div[2]/div[2]/div/a", count)
			).await?;

			let id = href.split("_").last().expect("no href");

			let title = <dyn Feed>::get_text(driver.clone(),
				format!("//div[contains(@class, \"items-items\")]/div[contains(@class, \"iva-item-root\")][{}]/div/div/div[2]/div[2]/div/a/h3", count),
				format!("//body/div[1]/div/buyer-location/div/div/div[2]/div/div[2]/div[3]/div[3]/div[3]/div[2]/div[contains(@class, \"iva-item-root\")][{}]/div/div/div[2]/div[2]/div/a/h3", count)
			).await?;

			let price = <dyn Feed>::get_price(driver.clone(),
				format!("//div[contains(@class, \"items-items\")]/div[contains(@class, \"iva-item-root\")][{}]/div/div/div[2]/div[3]/span/div/p/meta[2]", count),
				format!("//body/div[1]/div/buyer-location/div/div/div[2]/div/div[2]/div[3]/div[3]/div[3]/div[2]/div[contains(@class, \"iva-item-root\")][{}]/div/div/div[2]/div[3]/span/div/p/meta[2]", count)
			).await?;

			// let _ = <dyn Feed>::get_paid(driver.clone(),
			// 	format!("//div[contains(@class, \"items-items\")]/div[contains(@class, \"iva-item-root\")][{}]/div/div/div[2]/div[last()]/div[2]/div/i", count),
			// ).await?;

			let _ = <dyn Feed>::move_mouse_to_paid(driver.clone(),
				format!("//div[contains(@class, \"items-items\")]/div[contains(@class, \"iva-item-root\")][{}]/div/div/div/div[last()]/div[2]/div/i", count),
			).await?;

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

			// Переход в новую вкладку
			let handle = driver.window().await?;

			let _ = <dyn Feed>::click_ad_title_link(driver.clone(),
				format!("//div[contains(@class, \"items-items\")]/div[contains(@class, \"iva-item-root\")][{}]/div/div/div[2]/div[2]/div/a", count),
				format!("//body/div[1]/div/buyer-location/div/div/div[2]/div/div[2]/div[3]/div[3]/div[3]/div[2]/div[contains(@class, \"iva-item-root\")][{}]/div/div/div[2]/div[2]/div/a", count)
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

			let (seller_id, seller_name) = <dyn AdsAd>::get_seller_name_arr(driver.clone()).await?;
			let rating = <dyn AdsAd>::get_rating(driver.clone()).await?;
			let reviews = <dyn AdsAd>::get_reviews(driver.clone()).await?;
			let register_date = <dyn AdsAd>::get_register_date(driver.clone()).await?;
			let seller_ads_count = <dyn AdsAd>::get_seller_ads_count(driver.clone()).await?;
			let description_string = <dyn AdsAd>::get_description(driver.clone()).await?;
			let address = <dyn AdsAd>::get_address(driver.clone()).await?;
			let footer_article = <dyn AdsAd>::check_footer_article(driver.clone()).await?;
			let date = <dyn AdsAd>::get_date(driver.clone(), footer_article).await?;
			let (views, views_today) = <dyn AdsAd>::get_views_and_views_today(driver.clone(), footer_article).await?;

			// === RESULT ===

			driver.close_window().await?;
			driver.switch_to_window(handle.clone()).await?;
			sleep(Duration::from_secs(2)).await;

			println!("{} из {} - {}", &position, &ads_count.clone(), &id);

			wtr.write_record(&[
				format!("{}", utc.format("%d-%m-%Y_%H:%M:%S")).as_str(),
				position.to_string().as_str(),
				views.as_str(),
				views_today.as_str(),
				paid.to_string().as_str(),
				date.as_str(),
				id,
				title.replace("\"", "").as_str(),
				href.as_str(),
				price.as_str(),
				categories.as_str(),
				search_query,
				seller_id.as_str(),
				seller_name.as_str(),
				rating.as_str(),
				reviews.as_str(),
				register_date.as_str(),
				seller_ads_count.as_str(),
				description_string.as_str(),
				city_query,
				address.as_str(),
			])
			.expect("write record err");
		}

		let parent = <dyn Feed>::get_feed_parent_block(driver.clone()).await?;

		if parent.inner_html().await?.contains("других городов") {
			println!("====== break ======");
			driver.clone().quit().await?;
			break 'outer;
		}

		// пагинация
		let _ = <dyn Feed>::click_pagination_next_btn(driver.clone()).await?;

		println!("{}", "======");
	}
	wtr.flush()?;
	driver.clone().quit().await?;

	Ok(())
}
