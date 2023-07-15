use chrono::NaiveDate;
use serde::Deserialize;
use serde::Serialize;
use std::cmp::Ordering;

use crate::pocky::de;
use crate::pocky::ser;

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

#[derive(Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
pub struct ExternalMetadata {
	pub title: String,
	pub author: String,
	#[serde(deserialize_with = "de::date", serialize_with = "ser::date_display")]
	pub date: NaiveDate,
	#[serde(default, deserialize_with = "de::markdown_option")]
	pub summary: Option<String>,
	#[serde(default, deserialize_with = "de::comma_separated")]
	pub tags: Vec<String>,
}
