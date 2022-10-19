use chrono::NaiveDate;
use handlebars::Handlebars;
use pocky::AsHtml;
use pocky::MarkdownPage;
use serde::Deserialize;
use serde::Deserializer;
use serde::Serialize;
use serde::Serializer;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::path::Path;
use std::path::PathBuf;
use url::Url;

use crate::config::BLOG;

#[derive(Clone, Debug, Serialize, Eq, PartialEq)]
pub struct BlogPost {
	pub canonical_url: Url,
	pub path: PathBuf,
	pub metadata: BlogPostMetadata,
	pub content: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
pub struct BlogPostMetadata {
	// doesn't work well in `<title>`, wraps everything in a `<p>`...
	// #[serde(default, deserialize_with = "pocky::de::markdown")]
	pub title: String,
	pub author: String,
	#[serde(default, deserialize_with = "de_date", serialize_with = "ser_date")]
	pub date: Option<NaiveDate>,
	#[serde(default, deserialize_with = "pocky::de::option_markdown")]
	pub summary: Option<String>,
	#[serde(default, deserialize_with = "pocky::de::comma_separated")]
	pub tags: Vec<String>,
	pub cover: Option<HashMap<String, String>>,
	pub accent_color: Option<String>,
	#[serde(default)]
	pub status: BlogPostStatus,
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum BlogPostStatus {
	Draft,
	Test,
	#[default]
	Published,
	Unlisted,
}

fn de_date<'de, D>(de: D) -> Result<Option<NaiveDate>, D::Error>
where
	D: Deserializer<'de>,
{
	let date = Option::<String>::deserialize(de)?.map(|date_string| {
		NaiveDate::parse_from_str(&date_string, "%Y.%m.%d")
			.unwrap_or_else(|_| panic!("invalid date: {}", date_string))
	});

	Ok(date)
}

fn ser_date<S>(value: &Option<NaiveDate>, ser: S) -> Result<S::Ok, S::Error>
where
	S: Serializer,
{
	if let Some(date) = value {
		ser.serialize_str(&date.format("%A, %B %-d, %Y").to_string())
	} else {
		ser.serialize_none()
	}
}

impl<P> From<P> for BlogPost
where
	P: AsRef<Path>,
{
	fn from(path: P) -> Self {
		let page = MarkdownPage::<BlogPostMetadata>::from(&path);

		let mut path = path.as_ref().to_owned();
		path.set_extension("html");

		BlogPost {
			canonical_url: BLOG
				.canonical_origin
				.join(path.to_str().expect("path contains invalid characters"))
				.expect("failed to create canonical_url"),
			path,
			metadata: page.metadata.expect("missing blog post metadata"),
			content: page.content,
		}
	}
}

impl PartialOrd for BlogPost {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.metadata.date.cmp(&other.metadata.date).reverse())
	}
}
impl Ord for BlogPost {
	fn cmp(&self, other: &Self) -> Ordering {
		self.metadata.date.cmp(&other.metadata.date).reverse()
	}
}

impl AsHtml for BlogPost {
	fn as_html(&self) -> String {
		let renderer = Handlebars::new();
		renderer
			.render_template(include_str!("./templates/blog_post.html"), self)
			.expect("failed to render handlebars")
	}
}
