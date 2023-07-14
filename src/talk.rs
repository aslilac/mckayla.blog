use chrono::NaiveDate;
use handlebars::handlebars_helper;
use handlebars::Handlebars;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use std::path::Path;
use std::path::PathBuf;

use crate::pocky::de;
use crate::pocky::md;
use crate::pocky::ser;
use crate::pocky::AsHtml;
use crate::pocky::TextPage;

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
	// #[serde(default, deserialize_with = "de::markdown")]
	pub title: String,
	pub author: String,
	#[serde(deserialize_with = "de::date", serialize_with = "ser::date_display")]
	pub date: NaiveDate,
	#[serde(default, deserialize_with = "de::markdown_option")]
	pub summary: Option<String>,
	#[serde(default, deserialize_with = "de::comma_separated")]
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

handlebars_helper!(add: |a: i64, b: i64| a + b);

impl AsHtml for Talk {
	fn as_html(&self) -> String {
		let mut renderer = Handlebars::new();
		renderer.register_helper("add", Box::new(add));
		renderer
			.render_template(include_str!("./templates/talk.html"), self)
			.expect("failed to render handlebars")
	}
}
