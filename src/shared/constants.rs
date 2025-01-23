pub trait Constants {}

impl dyn Constants {
	pub fn get_ads_crawler_table_headers() -> [&'static str; 21] {
		[
			"Дата прогона",
			"Поз.",
			"Просмотров",
			"Просмотров сегодня",
			"Продвижение",
			"Дата объявления",
			"id",
			"Название",
			"Ссылка",
			"Цена",
			"Категории",
			"Поиск (запрос)",
			"id Продавца",
			"Продавец",
			"Рейтинг",
			"Кол-во отзывов",
			"Дата регистрации",
			"Кол-во объявлений",
			"Описание",
			"Город (запрос)",
			"Адрес",
		]
	}
}
