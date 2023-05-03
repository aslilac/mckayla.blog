use chrono::NaiveDate;
use serde::ser::SerializeStruct;
use serde::Deserialize;
use serde::Deserializer;
use serde::Serialize;
use serde::Serializer;
use std::cmp::Ordering;

#[derive(Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
pub struct External {
	pub canonical_url: String,
	pub path: String,
	#[serde(flatten)]
	pub metadata: ExternalMetadata,
}

impl PartialOrd for External {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.metadata.date.cmp(&other.metadata.date).reverse())
	}
}
impl Ord for External {
	fn cmp(&self, other: &Self) -> Ordering {
		self.metadata.date.cmp(&other.metadata.date).reverse()
	}
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct ExternalMetadata {
	pub title: String,
	pub author: String,
	#[serde(default, deserialize_with = "de_date")]
	pub date: NaiveDate,
	#[serde(default, deserialize_with = "pocky::de::option_markdown")]
	pub summary: Option<String>,
	#[serde(default, deserialize_with = "pocky::de::comma_separated")]
	pub tags: Vec<String>,
}

impl Serialize for ExternalMetadata {
	fn serialize<S>(&self, ser: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		let mut state = ser.serialize_struct("ExternalMetadata", 5)?;
		state.serialize_field("title", &self.title)?;
		state.serialize_field("author", &self.author)?;
		state.serialize_field("date", &self.date.format("%A, %B %-d, %Y").to_string())?;
		state.serialize_field("summary", &self.summary)?;
		state.serialize_field("tags", &self.tags)?;
		state.end()
	}
}

fn de_date<'de, D>(de: D) -> Result<NaiveDate, D::Error>
where
	D: Deserializer<'de>,
{
	let date_string = String::deserialize(de)?;
	let date = NaiveDate::parse_from_str(&date_string, "%Y.%m.%d")
		.unwrap_or_else(|_| panic!("invalid date: {}", date_string));

	Ok(date)
}
