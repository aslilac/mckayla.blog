use chrono::NaiveDate;
use pocky::AsHtml;
use pocky::MarkdownPage;
use serde::Deserialize;
use serde::Deserializer;
use std::cmp::Ordering;
use std::path::Path;
use std::path::PathBuf;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct BlogPostMetadata {
	pub title: String,
	pub author: String,
	#[serde(default, deserialize_with = "de_date")]
	pub date: Option<NaiveDate>,
	pub summary: Option<String>,
	#[serde(default, deserialize_with = "de_tags")]
	pub tags: Vec<String>,
	pub cover: Option<String>,
	#[serde(default)]
	pub status: BlogPostStatus,
}

fn de_tags<'de, D>(de: D) -> Result<Vec<String>, D::Error>
where
	D: Deserializer<'de>,
{
	let tags = String::deserialize(de)?
		.split(',')
		.map(|tag| tag.trim().to_string())
		.collect();

	Ok(tags)
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

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum BlogPostStatus {
	Draft,
	Test,
	#[default]
	Published,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BlogPost {
	pub path: PathBuf,
	pub metadata: BlogPostMetadata,
	pub content: String,
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
	/*
			<link rel=\"stylesheet\" href=\"https://unpkg.com/prismjs@1.29.0/themes/prism.css\" />\
			<script src=\"https://unpkg.com/prismjs@1.29.0/components/prism-core.min.js\"></script>\
			<script src=\"https://unpkg.com/prismjs@1.29.0/plugins/autoloader/prism-autoloader.min.js\"></script>\

			<link rel=\"stylesheet\" href=\"https://unpkg.com/@highlightjs/cdn-assets@11.6.0/styles/github.min.css\" />\
			<script src=\"https://unpkg.com/@highlightjs/cdn-assets@11.6.0/highlight.min.js\"></script>\
	*/

	fn as_html(&self) -> String {
		format!(
			"\
			<!doctype html>\n\
			<html lang=\"en-US\">\n\
			<head>\
			<title>{title}</title>\
			<meta charset=\"utf-8\" />\
			<link rel=\"icon\" href=\"https://cdn.mckayla.cloud/-/764b1512ee1f490a951e9c00d9ded4b2/Doodle.avif\" />\
			<link rel=\"preload\" href=\"https://cdn.mckayla.cloud/fonts/outfit/Outfit-Variable.woff2\" \
				as=\"font\" type=\"font/woff2\" crossorigin=\"anonymous\" />\
			<link rel=\"stylesheet\" href=\"https://cdn.mckayla.cloud/nothing.css\" />\
			<link rel=\"stylesheet\" href=\"/blog.css\" />\
			<!--<link rel=\"stylesheet\" href=\"https://unpkg.com/prismjs@1.29.0/themes/prism.min.css\" />-->\
			<link rel=\"stylesheet\" href=\"https://unpkg.com/prismjs@1.29.0/themes/prism.min.css\" media=\"(prefers-color-scheme: light)\" />\
			<link rel=\"stylesheet\" href=\"https://unpkg.com/prismjs@1.29.0/themes/prism-tomorrow.min.css\" media=\"(prefers-color-scheme: dark)\" />\
			<meta name=\"viewport\" content=\"width=device-width, initial-scale=1\" />\
			<link rel=\"og:title\" href=\"{title}\" />\
			<link rel=\"og:type\" href=\"website\" />\
			<link rel=\"og:image\" href=\"https://cdn.mckayla.cloud/-/97ef05b2b92b44c687dfcccfb32dff16/cute3.avif\" />\
			<link rel=\"og:image:secure_url\" href=\"https://cdn.mckayla.cloud/-/97ef05b2b92b44c687dfcccfb32dff16/cute3.avif\" />\
			<body>\
			<main>\
			<nav><a href=\"/\">Home</a></nav>\
			<header>\
			{cover}\
			<h1>{title}</h1>\
			</header>\
			<hr />\
			<article>{content}</article>\
			<footer>\
			<div>\
			&hearts; <a href=\"https://mckayla.dev\">McKayla</a>\
			</div>\
			</footer>\
			</main>\
			<script src=\"https://unpkg.com/prismjs@1.29.0/components/prism-core.min.js\"></script>\
			<script src=\"https://unpkg.com/prismjs@1.29.0/plugins/autoloader/prism-autoloader.min.js\"></script>\
			</body>\n\
			</html>\n\
			",
			title = self.metadata.title,
			cover = self.metadata.cover.as_ref().map(|url| format!("<img src=\"{}\" />", url)).unwrap_or_default(),
			content = self.content,
		)
	}
}
