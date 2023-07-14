use chrono::NaiveDate;
use handlebars::Handlebars;
use pocky::md;
use pocky::AsHtml;
use pocky::TextPage;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use std::path::Path;
use std::path::PathBuf;

use crate::de;
use crate::ser;

#[derive(Clone, Debug, Serialize, Eq, PartialEq)]
pub struct Talk {
	pub path: PathBuf,
	#[serde(flatten)]
	pub metadata: TalkMetadata,
	pub content: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
pub struct TalkMetadata {
	// doesn't work well in `<title>`, wraps everything in a `<p>`...
	// #[serde(default, deserialize_with = "pocky::de::markdown")]
	pub title: String,
	pub author: String,
	#[serde(deserialize_with = "de::date", serialize_with = "ser::date_display")]
	pub date: NaiveDate,
	#[serde(default, deserialize_with = "pocky::de::option_markdown")]
	pub summary: Option<String>,
	#[serde(default, deserialize_with = "pocky::de::comma_separated")]
	pub tags: Vec<String>,
	pub cover: Option<HashMap<String, String>>,
	pub accent_color: Option<String>,
}

impl<P> From<P> for Talk
where
	P: AsRef<Path>,
{
	fn from(path: P) -> Self {
		let TextPage { metadata, content } = TextPage::<TalkMetadata>::from(&path);

		let mut path = path.as_ref().to_owned();
		path.set_extension("html");

		let metadata = metadata.expect(&format!("missing talk metadata in {}", path.display()));

		let content = content
			.split("+++\n")
			.map(md::markdown_to_html)
			.collect::<Vec<_>>();

		Talk {
			path,
			metadata,
			content,
		}
	}
}

impl AsHtml for Talk {
	fn as_html(&self) -> String {
		let renderer = Handlebars::new();
		renderer
			.render_template(include_str!("./templates/talk.html"), self)
			.expect("failed to render handlebars")
	}
}
