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
	pub published: NaiveDate,
	pub summary: Option<String>,
	pub tags: Vec<String>,
}

impl From<HashMap<String, String>> for PageMetadata {
	fn from(map: HashMap<String, String>) -> Self {
		PageMetadata {
			title: map.get("title").unwrap().clone(),
			author: map.get("author").unwrap().clone(),
			published: NaiveDate::parse_from_str(map.get("published").unwrap(), "%Y.%m.%d")
				.unwrap(),
			summary: map.get("summary").cloned(),
			tags: map
				.get("tags")
				.map(|tags| {
					tags.split(',')
						.map(|tag| tag.trim().to_string())
						.collect::<Vec<_>>()
				})
				.unwrap_or_default(),
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
					metadata.insert(key.trim().to_string(), value.trim().to_string());
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
		Some(
			self.metadata
				.published
				.cmp(&other.metadata.published)
				.reverse(),
		)
	}
}
impl Ord for MarkdownPage {
	fn cmp(&self, other: &Self) -> Ordering {
		self.metadata
			.published
			.cmp(&other.metadata.published)
			.reverse()
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
			<link rel=\"stylesheet\" href=\"https://cdn.mckayla.cloud/nothing.css\" />\
			<link rel=\"stylesheet\" href=\"https://unpkg.com/prismjs@1.29.0/themes/prism.css\" />\
			<style>pre, code {{ font-size: 14px !important; }}</style>\
			</head>\n\
			<body>\
			<header>\
			<h1>{title}</h1>\
			<p>by {author}</p>\
			<p>Published on {published}</p>\
			</header>\
			<hr />\
			<article>{content}</article>\
			<script src=\"https://unpkg.com/prismjs@1.29.0/components/prism-core.min.js\"></script>\
			<script src=\"https://unpkg.com/prismjs@1.29.0/plugins/autoloader/prism-autoloader.min.js\"></script>\
			</body>\n\
			</html>\n\
			",
			title = self.metadata.title,
			author = self.metadata.author,
			published = self.metadata.published.format("%A, %B %-d, %Y"),
			content = markdown::to_html(&self.content),
		)
	}
}
