use serde::de::DeserializeOwned;
use std::fs;
use std::path::Path;

use crate::AsHtml;

#[derive(Clone, Debug)]
pub struct TextPage<M: DeserializeOwned> {
	pub metadata: Option<M>,
	pub content: String,
}

impl<M: DeserializeOwned> TextPage<M> {
	pub fn parse(content: String) -> Self {
		// Skip blank lines from the beginning
		let mut lines = content
			.lines()
			.skip_while(|line| line.trim().is_empty())
			.peekable();

		let is_frontmatter_delimiter =
			|line: &&str| line.len() >= 3 && line.find(|c| c != '-').is_none();

		// Parse the frontmatter section, if the document starts with one
		let metadata = lines.next_if(is_frontmatter_delimiter).map(|_| {
			let metadata_source = lines
				.by_ref()
				.take_while(|line| !is_frontmatter_delimiter(line))
				.map(|line| format!("{}\n", line))
				.collect::<String>();

			serde_yaml::from_str(&metadata_source).expect("failed to deserialize frontmatter metadata")
		});

		// The remaining lines are the actual document content
		let content = lines.map(|line| format!("{}\n", line)).collect::<String>();

		TextPage { metadata, content }
	}
}

impl<M, P> From<P> for TextPage<M>
where
	P: AsRef<Path>,
	M: DeserializeOwned,
{
	fn from(path: P) -> Self {
		let content = fs::read_to_string(path).expect("unable to read file");
		TextPage::parse(content)
	}
}

impl<M> AsHtml for TextPage<M>
where
	M: DeserializeOwned,
{
	fn as_html(&self) -> String {
		self.content.clone()
	}
}
