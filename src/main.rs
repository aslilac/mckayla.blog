mod blog_post;
mod config;
mod options;

use chrono::Utc;
use handlebars::Handlebars;
use pocky::AsHtml;
use pocky::PageCollection;
use serde_json::json;
use std::env;
use std::fs;
use std::io;

use blog_post::BlogPost;
use blog_post::BlogPostStatus::{Published, Unlisted};
use config::BLOG;
use options::Options;

fn main() -> io::Result<()> {
	let options = env::args().skip(1).collect::<Options>();

	let mut posts = PageCollection::<BlogPost>::from("./posts/");
	if options.publish {
		// Skip unpublished posts if we're building a version for publishing
		posts.retain(|post| post.metadata.status == Published || post.metadata.status == Unlisted);

		// Ensure that all of the posts have a date
		for post in posts.iter() {
			if post.metadata.date.is_none() {
				panic!("published posts must have a date");
			}
		}
	}

	// Render posts
	for post in posts.iter() {
		let output_path = options.output.join(&post.path);
		fs::create_dir_all(output_path.parent().unwrap())
			.expect("failed to create output directory");
		fs::write(output_path, post.as_html())?;
	}

	// Hide unlisted posts from the index and RSS feeds
	posts.retain(|post| post.metadata.status != Unlisted);
	let posts = posts.into_vec();

	let renderer = Handlebars::new();
	// Render index
	let index_page = renderer
		.render_template(
			include_str!("./templates/index.html"),
			&json!({ "blog": &*BLOG, "posts": &posts }),
		)
		.expect("failed to render handlebars");
	fs::write(options.output.join("index.html"), index_page)?;
	// Render index
	let updated = Utc::now().format("%Y-%m-%dT%h:%m:00.000Z").to_string();
	let rss_feed = renderer
		.render_template(
			include_str!("./templates/feed.xml"),
			&json!({ "blog": &*BLOG, "posts": &posts, "updated": updated }),
		)
		.expect("failed to render handlebars");
	fs::write(options.output.join("feed.xml"), rss_feed)?;

	// Copy assets
	for file in fs::read_dir("./src/static")?
		.flatten()
		.map(|entry| entry.path())
		.filter(|path| path.is_file())
	{
		fs::copy(
			&file
				.canonicalize()
				.expect("failed to resolve file location"),
			options
				.output
				.join(&file.file_name().expect("failed to get file name")),
		)?;
	}

	Ok(())
}
