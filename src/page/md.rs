use std::collections::HashMap;
use std::fs;
use std::path::Path;

pub struct MarkdownPage {
	pub metadata: HashMap<String, String>,
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
			metadata,
			content: lines.map(|line| format!("{}\n", line)).collect(),
		}
	}
}
