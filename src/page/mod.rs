mod md;

use std::fs;
use std::io;
use std::path::Path;

pub use md::MarkdownPage;

pub trait Page {
	fn render(&self) -> String;

	#[inline]
	fn render_to_file<P: AsRef<Path>>(&self, path: P) -> io::Result<()> {
		fs::write(path, self.render())
	}
}

pub trait FromFilePath {
	fn from_file_path<P: AsRef<Path>>(path: P) -> Self;
}
