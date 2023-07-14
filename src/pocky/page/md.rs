use serde::de::DeserializeOwned;
use std::fs;
use std::path::Path;

use crate::md::markdown_to_html;
use crate::page::text::TextPage;
use crate::AsHtml;

#[derive(Clone, Debug)]
pub struct MarkdownPage<M: DeserializeOwned> {
	pub metadata: Option<M>,
	pub content: String,
}

impl<M: DeserializeOwned> MarkdownPage<M> {
	pub fn parse(content: String) -> Self {
		let TextPage { metadata, content } = TextPage::parse(content);
		let content = markdown_to_html(content);

		MarkdownPage { metadata, content }
	}
}

impl<M, P> From<P> for MarkdownPage<M>
where
	P: AsRef<Path>,
	M: DeserializeOwned,
{
	fn from(path: P) -> Self {
		let content = fs::read_to_string(path).expect("unable to read file");
		MarkdownPage::parse(content)
	}
}

impl<M> AsHtml for MarkdownPage<M>
where
	M: DeserializeOwned,
{
	fn as_html(&self) -> String {
		self.content.clone()
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	type BasicMarkdownPage = MarkdownPage<HashMap<String, String>>;

	use serde::Deserialize;
	use std::collections::HashMap;

	#[test]
	fn no_frontmatter() {
		let page = BasicMarkdownPage::parse("# Hello, friend!\n".to_string());

		assert_eq!(page.metadata, None);
		assert_eq!(page.content, "<h1>Hello, friend!</h1>\n");
	}

	#[test]
	fn with_frontmatter() {
		let page = BasicMarkdownPage::parse(
			"
---
title: Cool video games
---
# Hello, friend!"
				.to_string(),
		);

		assert_eq!(
			page.metadata.unwrap().get("title"),
			Some(&"Cool video games".to_string())
		);
		assert_eq!(page.content, "<h1>Hello, friend!</h1>\n");
	}

	#[test]
	fn with_frontmatter_deserialize() {
		#[derive(Deserialize)]
		struct TitleMetadata {
			pub title: String,
		}

		let page = MarkdownPage::<TitleMetadata>::parse(
			"
---
title: Cool video games
---
# Hello, friend!"
				.to_string(),
		);

		assert_eq!(page.metadata.unwrap().title, "Cool video games".to_string());
		assert_eq!(page.content, "<h1>Hello, friend!</h1>\n");
	}
}
