mod blog_post;
mod options;

use handlebars::Handlebars;
use pocky::AsHtml;
use pocky::PageCollection;
use serde_json::json;
use std::env;
use std::fs;
use std::io;

use blog_post::BlogPost;
use blog_post::BlogPostStatus::Published;
use options::Options;

fn main() -> io::Result<()> {
	let options = env::args().skip(1).collect::<Options>();

	let mut posts = PageCollection::<BlogPost>::from("./posts/");
	if options.publish {
		// Skip unpublished posts if we're building a version for publishing
		posts.retain(|post| post.metadata.status == Published);

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

	// Render index
	let renderer = Handlebars::new();
	let index_page = renderer
		.render_template(
			include_str!("./index.html"),
			&json!({ "posts": &posts.into_vec() }),
		)
		.expect("failed to render handlebars");
	fs::write(options.output.join("index.html"), index_page)?;

	// Copy assets
	fs::copy("./src/blog.css", options.output.join("blog.css"))?;

	Ok(())
}
