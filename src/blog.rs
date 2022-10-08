use chrono::NaiveDate;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::path::Path;

use crate::page::FromFilePath;
use crate::page::MarkdownPage;
use crate::page::Page;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BlogPostMetadata {
	pub title: String,
	pub author: String,
	pub date: NaiveDate,
	pub summary: Option<String>,
	pub tags: Vec<String>,
	pub cover: Option<String>,
	pub status: BlogPostStatus,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum BlogPostStatus {
	Draft,
	Test,
	Published,
}

impl From<HashMap<String, String>> for BlogPostMetadata {
	fn from(map: HashMap<String, String>) -> Self {
		let status = match map
			.get("status")
			.unwrap_or(&"published".to_string())
			.to_ascii_lowercase()
			.as_str()
		{
			"draft" => BlogPostStatus::Draft,
			"test" => BlogPostStatus::Test,
			"published" => BlogPostStatus::Published,
			other => panic!("invalid page status {}", other),
		};

		BlogPostMetadata {
			title: map.get("title").unwrap().clone(),
			author: map.get("author").unwrap().clone(),
			date: NaiveDate::parse_from_str(map.get("date").unwrap(), "%Y.%m.%d").unwrap(),
			summary: map.get("summary").cloned(),
			tags: map
				.get("tags")
				.map(|tags| {
					tags.split(',')
						.map(|tag| tag.trim().to_string())
						.collect::<Vec<_>>()
				})
				.unwrap_or_default(),
			cover: map.get("cover").cloned(),
			status,
		}
	}
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BlogPost {
	pub metadata: BlogPostMetadata,
	pub content: String,
}

impl FromFilePath for BlogPost {
	fn from_file_path<P: AsRef<Path>>(path: P) -> Self {
		let page = MarkdownPage::from_file_path(path);

		BlogPost {
			metadata: page.metadata.into(),
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

impl Page for BlogPost {
	/*
			<link rel=\"stylesheet\" href=\"https://unpkg.com/prismjs@1.29.0/themes/prism.css\" />\
			<script src=\"https://unpkg.com/prismjs@1.29.0/components/prism-core.min.js\"></script>\
			<script src=\"https://unpkg.com/prismjs@1.29.0/plugins/autoloader/prism-autoloader.min.js\"></script>\

			<link rel=\"stylesheet\" href=\"https://unpkg.com/@highlightjs/cdn-assets@11.6.0/styles/github.min.css\" />\
			<script src=\"https://unpkg.com/@highlightjs/cdn-assets@11.6.0/highlight.min.js\"></script>\
	*/

	fn render(&self) -> String {
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
			<link rel=\"stylesheet\" href=\"https://unpkg.com/prismjs@1.29.0/themes/prism.min.css\" />\
			<!--<link rel=\"stylesheet\" href=\"https://unpkg.com/prismjs@1.29.0/themes/prism.min.css\" media=\"(prefers-color-scheme: light)\" />-->\
			<!--<link rel=\"stylesheet\" href=\"https://unpkg.com/prismjs@1.29.0/themes/prism-tomorrow.min.css\" media=\"(prefers-color-scheme: dark)\" />-->\
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
