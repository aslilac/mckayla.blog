mod md;

use std::fs;
use std::path::Path;

pub use md::MarkdownPage;
pub use md::PageStatus;

pub trait Page {
	fn render(&self) -> String;

	fn render_to_file<P: AsRef<Path>>(&self, path: P) {
		fs::write(path, self.render()).expect("unable to write to file");
	}
}
