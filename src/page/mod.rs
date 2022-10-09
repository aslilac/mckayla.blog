mod md;

use std::path::Path;

pub use md::MarkdownPage;

pub trait Page {
	fn as_html(&self) -> String;
}

pub trait FromFilePath {
	fn from_file_path<P: AsRef<Path>>(path: P) -> Self;
}
