use chrono::Utc;
use once_cell::sync::Lazy;
use serde::Serialize;
use url::Url;

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
	subtitle: "Aspiring wannabe • she/her 🏳️‍⚧️",
	og_title: "mckayla.blog",
	og_image: "https://cdn.mckayla.cloud/-/97ef05b2b92b44c687dfcccfb32dff16/cute3.avif",
	updated: Utc::now().format("%Y-%m-%dT%H:%M:00.000Z").to_string(),
});
