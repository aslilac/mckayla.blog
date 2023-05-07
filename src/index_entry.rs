use chrono::NaiveDate;
use serde::Serialize;
use std::cmp::Ordering;

use crate::blog_post::BlogPost;
use crate::external::External;

#[derive(Clone, Debug, Serialize, Eq, PartialEq)]
#[serde(untagged)]
pub enum IndexEntry {
	BlogPost(BlogPost),
	External(External),
}

impl IndexEntry {
	fn sort_criteria(&self) -> Option<&NaiveDate> {
		match self {
			IndexEntry::BlogPost(blog_post) => blog_post.metadata.date.as_ref(),
			IndexEntry::External(external) => Some(&external.metadata.date),
		}
	}
}

impl PartialOrd for IndexEntry {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.sort_criteria().cmp(&other.sort_criteria()).reverse())
	}
}
impl Ord for IndexEntry {
	fn cmp(&self, other: &Self) -> Ordering {
		self.sort_criteria().cmp(&other.sort_criteria()).reverse()
	}
}

impl From<BlogPost> for IndexEntry {
	fn from(post: BlogPost) -> Self {
		IndexEntry::BlogPost(post)
	}
}

impl From<External> for IndexEntry {
	fn from(external: External) -> Self {
		IndexEntry::External(external)
	}
}
