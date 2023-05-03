use chrono::Utc;
use once_cell::sync::Lazy;
use serde::Serialize;
use serde_json::json;
use std::collections::BTreeSet;
use std::collections::HashSet;
use url::Url;

use crate::external::External;
use crate::redirect_config;
use crate::redirect_page::RedirectPage;

#[derive(Clone, Debug, Serialize)]
pub struct BlogMetadata {
	pub canonical_origin: Url,
	pub favicon: &'static str,
	pub thumbnail: &'static str,
	pub title: &'static str,
	pub subtitle: &'static str,
	pub og_title: &'static str,
	pub og_image: &'static str,
	pub updated: String,
}

pub static BLOG: Lazy<BlogMetadata> = Lazy::new(|| BlogMetadata {
	canonical_origin: Url::parse("https://mckayla.blog").unwrap(),
	favicon: "https://cdn.mckayla.cloud/-/764b1512ee1f490a951e9c00d9ded4b2/Doodle.avif",
	thumbnail: "https://cdn.mckayla.cloud/-/764b1512ee1f490a951e9c00d9ded4b2/Doodle.avif",
	title: "Kayla",
	subtitle: "Aspiring wannabe, human shaped, slightly minty • she/her 🏳️‍⚧️",
	og_title: "mckayla.blog",
	og_image: "https://cdn.mckayla.cloud/-/97ef05b2b92b44c687dfcccfb32dff16/cute3.avif",
	updated: Utc::now().format("%Y-%m-%dT%H:%M:00.000Z").to_string(),
});

// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
// * Configure external posts here!!                                                     *
// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
pub static EXTERNAL_LINKS: Lazy<BTreeSet<External>> = Lazy::new(|| {
	serde_json::from_value(json!([
			// {
			// 	"canonical_url": "https://xaslilac.github.io/TeenageWeb/",
			// 	"path": "https://xaslilac.github.io/TeenageWeb/",
			// 	"title": "The Teenage Web",
			// 	"author": "Kayla Washburn",
			// 	"date": "2023.5.2",
			// 	"summary": "The web used to be a lot more fun. and a lot easier, too.",
			// }
		]))
	.expect("invalid external link set")
});

// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
// * Configure redirects here!!                                                          *
// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
pub static REDIRECTS: Lazy<HashSet<RedirectPage>> = redirect_config!(
  "/posts/gleam-traits.html" => "/posts/all-you-need-is-data-and-functions.html",
);
