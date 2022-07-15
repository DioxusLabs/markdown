use dioxus::prelude::*;
use pulldown_cmark::Parser;

#[derive(Props)]
pub struct MarkdownProps<'a> {
    content: &'a str
}

/// Render some text as markdown.
pub fn render_markdown<'a>(cx: Scope<'a, MarkdownProps<'a>>) -> Element {
    let parser = Parser::new(cx.props.content);

    let mut html_buf = String::new();
    pulldown_cmark::html::push_html(&mut html_buf, parser);

    cx.render(rsx!(div {
        dangerous_inner_html: format_args!("{}", html_buf),
    }))
}
