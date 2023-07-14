pub mod de;
pub mod md;
mod page;
pub mod ser;

pub use page::html::HtmlPage;
pub use page::md::MarkdownPage;
pub use page::text::TextPage;
pub use page::AsHtml;
pub use page::OrderedPageCollection;
pub use page::PageCollection;
