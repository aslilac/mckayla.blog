use chrono::NaiveDate;
use handlebars::Handlebars;
use pocky::AsHtml;
use pocky::MarkdownPage;
use serde::ser::SerializeStruct;
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
	#[serde(flatten)]
	pub metadata: BlogPostMetadata,
	pub content: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct BlogPostMetadata {
	// doesn't work well in `<title>`, wraps everything in a `<p>`...
	// #[serde(default, deserialize_with = "pocky::de::markdown")]
	pub title: String,
	pub author: String,
	#[serde(default, deserialize_with = "de_date")]
	pub date: Option<NaiveDate>,
	#[serde(default, deserialize_with = "pocky::de::option_markdown")]
	pub summary: Option<String>,
	#[serde(default, deserialize_with = "pocky::de::comma_separated")]
	pub tags: Vec<String>,
	pub cover: Option<HashMap<String, String>>,
	pub accent_color: Option<String>,
	#[serde(default)]
	pub status: BlogPostStatus,
	pub hn: Option<String>,
	pub lobsters: Option<String>,
}

impl Serialize for BlogPostMetadata {
	fn serialize<S>(&self, ser: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		let ser_date = |date: &Option<NaiveDate>, format: &str| {
			date.map(|date| date.format(format).to_string())
		};

		let mut state = ser.serialize_struct("BlogPostMetadata", 10)?;
		state.serialize_field("title", &self.title)?;
		state.serialize_field("author", &self.author)?;
		state.serialize_field("date", &ser_date(&self.date, "%A, %B %-d, %Y"))?;
		state.serialize_field("updated", &ser_date(&self.date, "%Y-%m-%dT00:00:00.000Z"))?;
		state.serialize_field("summary", &self.summary)?;
		state.serialize_field("tags", &self.tags)?;
		state.serialize_field("cover", &self.cover)?;
		state.serialize_field("accent_color", &self.accent_color)?;
		state.serialize_field("status", &self.status)?;
		state.serialize_field("hn", &self.hn)?;
		state.serialize_field("lobsters", &self.lobsters)?;
		state.end()
	}
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
