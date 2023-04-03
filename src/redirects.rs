use once_cell::sync::Lazy;
use std::collections::HashSet;

use crate::redirect_config;
use crate::redirect_page::RedirectPage;

// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
// * Configure redirects here!!                                                          *
// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
pub static REDIRECTS: Lazy<HashSet<RedirectPage>> = redirect_config!(
  "/posts/gleam-traits.html" => "/posts/all-you-need-is-data-and-functions.html",
);
