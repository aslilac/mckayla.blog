use chrono::NaiveDate;
use std::cmp::Ordering;
// use std::convert::TryFrom;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

use super::Page;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PageMetadata {
	pub title: String,
	pub author: String,
	pub date: NaiveDate,
	pub summary: Option<String>,
	pub tags: Vec<String>,
	pub cover: Option<String>,
	pub status: PageStatus,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PageStatus {
	Draft,
	Test,
	Published,
}

impl From<HashMap<String, String>> for PageMetadata {
	fn from(map: HashMap<String, String>) -> Self {
		let status = match map
			.get("status")
			.unwrap_or(&"published".to_string())
			.to_ascii_lowercase()
			.as_str()
		{
			"draft" => PageStatus::Draft,
			"test" => PageStatus::Test,
			"published" => PageStatus::Published,
			other => panic!("invalid page status {}", other),
		};

		PageMetadata {
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
pub struct MarkdownPage {
	pub metadata: PageMetadata,
	pub content: String,
}

impl MarkdownPage {
	pub fn from_file_path<P: AsRef<Path>>(path: P) -> Self {
		let content = fs::read_to_string(path).expect("unable to read file");

		let is_frontmatter_delimiter =
			|line: &&str| line.len() >= 3 && line.find(|c| c != '-').is_none();

		// Parse frontmatter metadata
		let mut metadata = HashMap::<String, String>::default();
		let mut lines = content
			.lines()
			.skip_while(|line| line.trim().is_empty())
			.peekable();
		if lines.next_if(is_frontmatter_delimiter).is_some() {
			let metadata_lines = lines
				.by_ref()
				.take_while(|line| !is_frontmatter_delimiter(line));
			metadata_lines
				.filter(|line| !line.trim().is_empty())
				.for_each(|line| {
					let (key, value) = line
						.split_once(':')
						.expect("frontmatter section should only contain key value pairs");
					metadata.insert(
						key.trim().to_ascii_lowercase().to_string(),
						value.trim().to_string(),
					);
				});
		}

		MarkdownPage {
			metadata: metadata.into(),
			content: lines.map(|line| format!("{}\n", line)).collect(),
		}
	}
}

impl PartialOrd for MarkdownPage {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.metadata.date.cmp(&other.metadata.date).reverse())
	}
}
impl Ord for MarkdownPage {
	fn cmp(&self, other: &Self) -> Ordering {
		self.metadata.date.cmp(&other.metadata.date).reverse()
	}
}

impl Page for MarkdownPage {
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
			<link rel=\"stylesheet\" href=\"https://cdn.mckayla.cloud/nothing.css\" />\
			<link rel=\"stylesheet\" href=\"https://unpkg.com/prismjs@1.29.0/themes/prism.css\" />\
			<meta name=\"viewport\" content=\"width=device-width, initial-scale=1\" />\
			<link rel=\"og:title\" href=\"{title}\" />\
			<link rel=\"og:type\" href=\"website\" />\
			<link rel=\"og:image\" href=\"https://cdn.mckayla.cloud/-/97ef05b2b92b44c687dfcccfb32dff16/cute3.avif\" />\
			<link rel=\"og:image:secure_url\" href=\"https://cdn.mckayla.cloud/-/97ef05b2b92b44c687dfcccfb32dff16/cute3.avif\" />\
			<style>pre {{ font-size: 14px !important; }}</style>\
			<style>hr {{ border: 1px solid #7773; }}</style>\
			<style>img {{ margin: auto; max-width: 100%; max-height: 70vh; }}</style>\
			<style>blockquote {{ margin: 0; padding: 0.5em 2em; background-color: #f8f8f8; border-radius: 6px; }}</style>\
			<body>\
			<main>\
			<a href=\"..\">Home</a>\
			<header>\
			{cover}\
			<h1>{title}</h1>\
			</header>\
			<hr />\
			<article>{content}</article>\
			</main>\
			<script src=\"https://unpkg.com/prismjs@1.29.0/components/prism-core.min.js\"></script>\
			<script src=\"https://unpkg.com/prismjs@1.29.0/plugins/autoloader/prism-autoloader.min.js\"></script>\
			</body>\n\
			</html>\n\
			",
			title = self.metadata.title,
			cover = self.metadata.cover.as_ref().map(|url| format!("<img src=\"{}\" />", url)).unwrap_or_default(),
			content = markdown::to_html(&self.content),
		)
	}
}
