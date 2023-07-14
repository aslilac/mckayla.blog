use chrono::NaiveDate;
use serde::Deserialize;
use serde::Deserializer;

use crate::pocky::md;

pub fn comma_separated<'de, D>(de: D) -> Result<Vec<String>, D::Error>
where
	D: Deserializer<'de>,
{
	let tags = String::deserialize(de)?
		.split(',')
		.map(|tag| tag.trim().to_string())
		.collect();

	Ok(tags)
}

pub fn markdown<'de, D>(de: D) -> Result<String, D::Error>
where
	D: Deserializer<'de>,
{
	Ok(md::markdown_to_html(String::deserialize(de)?))
}

pub fn markdown_option<'de, D>(de: D) -> Result<String, D::Error>
where
	D: Deserializer<'de>,
{
	Ok(Option::<String>::deserialize(de)?.map(md::markdown_to_html))
}

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
