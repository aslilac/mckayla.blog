pub mod html;
pub mod md;
pub mod text;

use std::fs;
use std::path::Path;
use std::path::PathBuf;

pub trait AsHtml {
	fn as_html(&self) -> String;
}

pub fn pages_from_directory<E, P>(path: P) -> impl Iterator<Item = E>
where
	P: AsRef<Path>,
	E: AsHtml + From<PathBuf>,
{
	fs::read_dir(path)
		.expect("could not read directory contents")
		.flatten()
		.filter(|entry| {
			entry
				.file_type()
				.map(|file_type| file_type.is_file())
				.unwrap_or(false)
		})
		.map(|entry| E::from(entry.path()))
}
