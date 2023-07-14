pub fn markdown_to_html<S>(md_source: S) -> String
where
	S: AsRef<str>,
{
	let options = {
		use pulldown_cmark::Options;

		let mut options = Options::empty();
		options.insert(Options::ENABLE_FOOTNOTES);
		options.insert(Options::ENABLE_STRIKETHROUGH);
		options.insert(Options::ENABLE_TABLES);
		options.insert(Options::ENABLE_TASKLISTS);

		options
	};

	// Allocate *roughly* enough room. It'll probably resize at least once, but this
	// should prevent it from needing to resize multiple times.
	let md_source = md_source.as_ref();
	let mut result = String::with_capacity(md_source.len());
	let parser = pulldown_cmark::Parser::new_ext(md_source, options);
	pulldown_cmark::html::push_html(&mut result, parser);
	result
}
