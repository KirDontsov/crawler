pub trait Constants {}

impl dyn Constants {
	pub fn get_ads_crawler_table_headers() -> [&'static str; 27] {
		[
			"Мое",
			"Дата прогона",
			"Город (запрос)",
			"Поиск (запрос)",
			"Поз.",
			"Просмотров",
			"Просмотров сегодня",
			"Продвижение",
			"Доставка",
			"Дата объявления",
			"id",
			"Название",
			"Цена",
			"Ссылка",
			"Категории",
			"id Продавца",
			"Продавец",
			"Тип продавца",
			"Дата регистрации",
			"Время ответа",
			"Рейтинг",
			"Кол. отзывов",
			"Кол. объявлений",
			"Кол. закрытых",
			"Фото",
			"Адрес",
			"Описание",
		]
	}

	pub fn get_vacancies_crawler_table_headers() -> [&'static str; 25] {
		[
			"Мое",
			"Дата прогона",
			"Город (запрос)",
			"Поиск (запрос)",
			"Поз.",
			"Просмотров",
			"Просмотров сегодня",
			"Продвижение",
			"Дата объявления",
			"id",
			"Название",
			"Зар. плата",
			"Ссылка",
			"Категории",
			"id Продавца",
			"Продавец",
			"Тип продавца",
			"Дата регистрации",
			"Время ответа",
			"Рейтинг",
			"Кол. отзывов",
			"Кол. объявлений",
			"Кол. закрытых",
			"Адрес",
			"Описание",
		]
	}
}
