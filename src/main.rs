mod blog;
mod options;
mod page;

use std::cmp::Ordering;
use std::env;
use std::fs;
use std::io;
use std::path::PathBuf;

use blog::BlogPost;
use blog::BlogPostStatus::Published;
use options::Options;
use page::Page;

#[derive(Clone, Debug, Eq, PartialEq)]
struct BlogPostFile {
	pub path: PathBuf,
	pub page: BlogPost,
}

impl PartialOrd for BlogPostFile {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.page.cmp(&other.page))
	}
}
impl Ord for BlogPostFile {
	fn cmp(&self, other: &Self) -> Ordering {
		self.page.cmp(&other.page)
	}
}

impl From<PathBuf> for BlogPostFile {
	fn from(mut path: PathBuf) -> Self {
		let page = BlogPost::from_file_path(&path);
		path.set_extension("html");
		BlogPostFile { path, page }
	}
}

fn main() -> io::Result<()> {
	let options = env::args().skip(1).collect::<Options>();

	let dir = fs::read_dir("./posts/")?;
	let mut posts = dir
		.flatten()
		.filter(|entry| entry.file_type().unwrap().is_file())
		.map(|entry| entry.path().into())
		.filter(|post: &BlogPostFile| !options.publish || post.page.metadata.status == Published)
		.collect::<Vec<_>>();
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
		<link rel=\"icon\" href=\"https://cdn.mckayla.cloud/-/764b1512ee1f490a951e9c00d9ded4b2/Doodle.avif\" />\
		<link rel=\"stylesheet\" href=\"https://cdn.mckayla.cloud/nothing.css\" />\
		<meta name=\"viewport\" content=\"width=device-width, initial-scale=1\" />\
		<link rel=\"og:title\" href=\"{title}\" />\
		<link rel=\"og:type\" href=\"website\" />\
		<link rel=\"og:image\" href=\"https://cdn.mckayla.cloud/-/97ef05b2b92b44c687dfcccfb32dff16/cute3.avif\" />\
		<link rel=\"og:image:secure_url\" href=\"https://cdn.mckayla.cloud/-/97ef05b2b92b44c687dfcccfb32dff16/cute3.avif\" />\
		<style>hr {{ border: 1px solid #7773; }}</style>\
		<style>h1 {{ margin: 0.5em 0 0; }}</style>\
		<style>article {{ margin: 2em 0; }}</style>\
		</head>\n\
		<body>\
		<main>\
		<header style=\"text-align: center;\">\
		<picture>\
		<img width=200 height=200 src=\"https://cdn.mckayla.cloud/-/764b1512ee1f490a951e9c00d9ded4b2/Doodle.avif\" />\
		</picture>\
		<h1>{title}</h1>\
		<p>{subtitle}</p>\
		</header>\
		<hr />\
		{index}\
		</main>\
		</body>\n\
		</html>\n\
		",
		title = "Kayla",
		subtitle = "Aspiring wannabe • she/her 🏳️‍⚧️"
	);

	for post in posts {
		let output_path = options.output.join(&post.path);
		fs::create_dir_all(output_path.parent().unwrap())
			.expect("failed to create output directory");
		post.page.render_to_file(output_path)?;
	}

	fs::write(options.output.join("index.html"), index_page)?;

	Ok(())
}