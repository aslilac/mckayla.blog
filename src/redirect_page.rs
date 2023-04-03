use handlebars::Handlebars;
use pocky::AsHtml;
use serde::Serialize;
use std::hash::Hash;
use std::hash::Hasher;
use std::path::PathBuf;

#[derive(Debug, Eq, Serialize)]
pub struct RedirectPage {
	pub from: PathBuf,
	pub to: &'static str,
}

impl PartialEq for RedirectPage {
	fn eq(&self, other: &Self) -> bool {
		self.from == other.from
	}
}

impl Hash for RedirectPage {
	fn hash<H>(&self, state: &mut H)
	where
		H: Hasher,
	{
		self.from.hash(state);
	}
}

impl AsHtml for RedirectPage {
	fn as_html(&self) -> String {
		let renderer = Handlebars::new();
		renderer
			.render_template(include_str!("./templates/redirect.html"), self)
			.expect("failed to render handlebars")
	}
}

#[macro_export]
macro_rules! redirect_config {
	( $( $from:expr => $to:expr ),* $(,)? ) => {{
    use once_cell::sync::Lazy;
    use std::collections::HashSet;

    Lazy::new(|| {
      let mut redirects = HashSet::default();
      $({
        redirects.insert(RedirectPage {
          from: $from.into(),
          to: $to.into(),
        });
      })*
      redirects
    })
  }};
}
