use dioxus::core::*;
use dioxus::prelude::*;
use pulldown_cmark::Parser;

#[derive(PartialEq, Props)]
pub struct StaticMarkdownProps {
    contents: String,
}

/// Render some text as markdown.
///
/// # Example
///
/// ```rust
///
/// static MARKDOWN: &str = include_str!("./sample.md");
///
/// static App: FC<()> = |(cx, props)| {
///     rsx(cx, div {
///         h1 {"post"}
///         div {
///             class: "post-body"
///             StaticMarkdown {
///                 contents: MARKDOWN.to_string(),
///             }
///         }
///     })
/// }
/// ```
pub static StaticMarkdown: FC<StaticMarkdownProps> =
    |(cx, props)| render_markdown(cx, &props.contents);

pub fn render_markdown<'a>(cx: Context<'a>, text: &str) -> DomTree<'a> {
    let parser = Parser::new(text);

    let mut html_buf = String::new();
    pulldown_cmark::html::push_html(&mut html_buf, parser);

    cx.render(rsx!(div {
        dangerous_inner_html: format_args!("{}", html_buf),
    }))
}
