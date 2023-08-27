use handlebars::Handlebars;
use serde_json::json;
use std::collections::BTreeSet;
use std::env;
use std::fs;
use std::io;

mod blog_post;
mod config;
mod external;
mod index_entry;
mod options;
mod pocky;
mod redirect_page;
mod talk;

use blog_post::BlogPost;
use blog_post::BlogPostStatus::{Published, Unlisted};
use config::BLOG;
use config::EXTERNAL_LINKS;
use config::REDIRECTS;
use index_entry::IndexEntry;
use options::Options;
use pocky::pages_from_directory;
use pocky::AsHtml;
use talk::Talk;

fn main() -> io::Result<()> {
	let options = env::args().skip(1).collect::<Options>();

	// Render redirects
	for redirect in REDIRECTS.iter() {
		let output_path = options.output.join(
			redirect
				.from
				.strip_prefix("/")
				.expect("`from` for redirect should be an absolute url"),
		);
		fs::create_dir_all(output_path.parent().unwrap()).expect("failed to create output directory");
		fs::write(output_path, redirect.as_html())?;
	}

	// Collect posts into a `PageCollection`
	let mut posts = pages_from_directory("content/posts/").collect::<Vec<BlogPost>>();
	posts.iter_mut().for_each(|post| {
		post.path = post.path.strip_prefix("content/").unwrap().to_owned();
		post.canonicalize();
	});
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
		fs::create_dir_all(output_path.parent().unwrap()).expect("failed to create output directory");
		fs::write(output_path, post.as_html())?;
	}

	// Hide unlisted posts from the index and RSS feeds
	posts.retain(|post| post.metadata.status != Unlisted);

	// Collect talks into a `PageCollection`
	let mut talks = pages_from_directory("content/talks/").collect::<Vec<Talk>>();
	talks.iter_mut().for_each(|talk| {
		talk.path = talk.path.strip_prefix("content/").unwrap().to_owned();
	});

	// Render talks
	for talk in talks.iter() {
		let output_path = options.output.join(&talk.path);
		fs::create_dir_all(output_path.parent().unwrap()).expect("failed to create output directory");
		fs::write(output_path, talk.as_html())?;
	}

	// Create the index entries set from posts and external links
	let post_entries = posts.iter().cloned().map(Into::into);
	let talk_entries = talks.iter().cloned().map(Into::into);
	let external_link_entries = EXTERNAL_LINKS.iter().cloned().map(Into::into);
	let index_entries = post_entries
		.chain(talk_entries)
		.chain(external_link_entries)
		.collect::<BTreeSet<IndexEntry>>();

	let renderer = Handlebars::new();
	// Render index
	let index_page = renderer
		.render_template(
			include_str!("./templates/index.html"),
			&json!({ "blog": &*BLOG, "posts": &index_entries }),
		)
		.expect("failed to render handlebars");
	fs::write(options.output.join("index.html"), index_page)?;
	// Render feed.xml
	let rss_feed = renderer
		.render_template(
			include_str!("./templates/feed.xml"),
			&json!({ "blog": &*BLOG, "posts": &posts }),
		)
		.expect("failed to render handlebars");
	fs::write(options.output.join("feed.xml"), rss_feed)?;

	// Copy assets from content/resources/ to the output/resources/ directory
	let output_path = options.output.join("resources/");
	fs::create_dir_all(&output_path).expect("failed to create output directory");
	for file in fs::read_dir("content/resources/")?
		.flatten()
		.map(|entry| entry.path())
		.filter(|path| path.is_file())
	{
		fs::copy(
			&file
				.canonicalize()
				.expect("failed to resolve file location"),
			output_path.join(file.file_name().expect("failed to get file name")),
		)?;
	}

	Ok(())
}
