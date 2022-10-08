mod page;

use once_cell::sync::Lazy;
use std::cmp::Ordering;
use std::fs;
use std::io;
use std::path::PathBuf;

use page::MarkdownPage;
use page::Page;

#[derive(Clone, Debug, Eq, PartialEq)]
struct BlogPost {
	pub path: PathBuf,
	pub page: MarkdownPage,
}

impl PartialOrd for BlogPost {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.page.cmp(&other.page))
	}
}
impl Ord for BlogPost {
	fn cmp(&self, other: &Self) -> Ordering {
		self.page.cmp(&other.page)
	}
}

static OUTPUT_DIR: Lazy<PathBuf> = Lazy::new(|| PathBuf::from("./output/"));

impl From<PathBuf> for BlogPost {
	fn from(mut path: PathBuf) -> Self {
		let page = MarkdownPage::from_file_path(&path);
		path.set_extension("html");
		BlogPost { path, page }
	}
}

fn main() -> io::Result<()> {
	let dir = fs::read_dir("./posts/")?;
	let mut posts = dir
		.flatten()
		.filter(|entry| entry.file_type().unwrap().is_file())
		.map(|entry| entry.path().into())
		.collect::<Vec<BlogPost>>();
	posts.sort();

	let index = posts
		.iter()
		.map(|post| {
			format!(
				"\
				<article>\
				<a href=\"{}\"><h1>{}</h1></a>\
				<sub>by {}</sub>\
				{}\
				</article>\
				",
				post.path.to_string_lossy(),
				post.page.metadata.title,
				post.page.metadata.author,
				post.page
					.metadata
					.summary
					.as_ref()
					.map(|summary| format!("<p>{}</p>", summary))
					.unwrap_or_default(),
			)
		})
		.collect::<String>();

	let index_page = format!(
		"\
		<!doctype html>\n\
		<html lang=\"en-US\">\n\
		<head>\
		<title>{title}</title>\
		<meta charset=\"utf-8\" />\
		<link rel=\"stylesheet\" href=\"https://cdn.mckayla.cloud/nothing.css\" />\
		</head>\n\
		<body>\
		<header>\
		<picture>\
		<img width=200 height=200 src=\"https://cdn.mckayla.cloud/-/764b1512ee1f490a951e9c00d9ded4b2/Doodle.avif\" />\
		</picture>\
		<h1>{title}</h1>\
		<p>{subtitle}</p>\
		</header>\
		<hr />\
		{index}\
		</body>\n\
		</html>\n\
		",
		title = "Kayla",
		subtitle = "Aspiring wannabe • she/her 🏳️‍⚧️"
	);

	for post in posts {
		// println!("{}: {:?}", post.path.to_str().unwrap(), post.page.metadata);
		let output_path = OUTPUT_DIR.join(&post.path);
		fs::create_dir_all(output_path.parent().unwrap())
			.expect("failed to create output/ directory");
		post.page.render_to_file(output_path);
	}

	fs::write(OUTPUT_DIR.join("index.html"), index_page)?;

	Ok(())
}
