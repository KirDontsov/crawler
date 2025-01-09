mod controllers;
mod api;

use dotenv::dotenv;
use std::env;
use std::error::Error;

use crate::controllers::avito_crawler_handler;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
	dotenv().ok();

	if std::env::var_os("RUST_LOG").is_none() {
		std::env::set_var("RUST_LOG", "actix_web=info");
	}
	env_logger::init();

	println!("Starting process...");

	let processing_type = env::var("PROCESSING_TYPE").expect("PROCESSING_TYPE not set");
	println!("PROCESSING_TYPE: {}", &processing_type);

	match processing_type.as_str() {
		"avito" => avito_crawler_handler().await?,
		// "smth_else" => avito_crawler_handler().await?,
		_ => println!("error in env (no such handler)!"),
	}

	Ok(())
}
