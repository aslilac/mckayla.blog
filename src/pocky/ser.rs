use chrono::NaiveDate;
use serde::Serializer;

pub fn date_display<S>(date: &NaiveDate, ser: S) -> Result<S::Ok, S::Error>
where
	S: Serializer,
{
	ser.serialize_str(&date.format("%A, %B %-d, %Y").to_string())
}

pub fn date_display_option<S>(date: &Option<NaiveDate>, ser: S) -> Result<S::Ok, S::Error>
where
	S: Serializer,
{
	match date {
		Some(date) => ser.serialize_some(&date.format("%A, %B %-d, %Y").to_string()),
		None => ser.serialize_none(),
	}
}
