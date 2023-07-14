use std::fs;
use std::path::Path;

use crate::AsHtml;

#[derive(Clone, Debug)]
pub struct HtmlPage {
	pub content: String,
}

impl HtmlPage {
	#[allow(dead_code)]
	pub fn new(content: String) -> Self {
		Self { content }
	}
}

impl<P> From<P> for HtmlPage
where
	P: AsRef<Path>,
{
	fn from(path: P) -> Self {
		let content = fs::read_to_string(path).expect("unable to read file");
		HtmlPage { content }
	}
}

impl AsHtml for HtmlPage {
	fn as_html(&self) -> String {
		self.content.clone()
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn from_string() {
		let page = HtmlPage::new("<h1>Hello, friend!</h1>".to_string());

		assert_eq!(page.content, "<h1>Hello, friend!</h1>");
		assert_eq!(page.as_html(), "<h1>Hello, friend!</h1>");
	}
}
