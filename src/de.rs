use chrono::NaiveDate;
use serde::Deserialize;
use serde::Deserializer;

pub fn date<'de, D>(de: D) -> Result<NaiveDate, D::Error>
where
	D: Deserializer<'de>,
{
	let date_string = String::deserialize(de)?;
	let date = NaiveDate::parse_from_str(&date_string, "%Y.%m.%d")
		.unwrap_or_else(|_| panic!("invalid date: {}", date_string));

	Ok(date)
}

pub fn date_option<'de, D>(de: D) -> Result<Option<NaiveDate>, D::Error>
where
	D: Deserializer<'de>,
{
	let date = Option::<String>::deserialize(de)?.map(|date_string| {
		NaiveDate::parse_from_str(&date_string, "%Y.%m.%d")
			.unwrap_or_else(|_| panic!("invalid date: {}", date_string))
	});

	Ok(date)
}
