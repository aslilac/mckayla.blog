use std::collections::HashMap;
use std::fs;
use std::path::Path;

use super::FromFilePath;

pub struct MarkdownPage {
	pub metadata: HashMap<String, String>,
	pub content: String,
}

impl MarkdownPage {
	pub fn from_content(content: String) -> Self {
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
				.take_while(|line| !is_frontmatter_delimiter(line))
				.filter(|line| !line.trim().is_empty());

			for line in metadata_lines {
				let (key, value) = line
					.split_once(':')
					.expect("frontmatter section should only contain key value pairs");

				metadata.insert(
					key.trim().to_ascii_lowercase().to_string(),
					value.trim().to_string(),
				);
			}
		}

		let content =
			markdown::to_html(&lines.map(|line| format!("{}\n", line)).collect::<String>());

		MarkdownPage { metadata, content }
	}
}

impl FromFilePath for MarkdownPage {
	fn from_file_path<P: AsRef<Path>>(path: P) -> Self {
		let content = fs::read_to_string(path).expect("unable to read file");
		MarkdownPage::from_content(content)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn no_frontmatter() {
		let page = MarkdownPage::from_content("# Hello, friend!\n".to_string());

		assert_eq!(page.metadata.len(), 0);
		assert_eq!(
			page.content,
			"<h1 id='hello,_friend!'>Hello, friend!</h1>\n"
		);
	}
	#[test]
	fn with_frontmatter() {
		let page = MarkdownPage::from_content(
			"
---
title: Cool video games
---
# Hello, friend!"
				.to_string(),
		);

		assert_eq!(
			page.metadata.get("title"),
			Some(&"Cool video games".to_string())
		);
		assert_eq!(
			page.content,
			"<h1 id='hello,_friend!'>Hello, friend!</h1>\n"
		);
	}
}
