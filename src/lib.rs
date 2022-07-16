#![allow(non_snake_case)]

use dioxus::prelude::*;
use pulldown_cmark::Parser;

#[derive(Props)]
pub struct MarkdownProps<'a> {
    #[props(default)]
    id: &'a str,
    #[props(default)]
    class: &'a str,

    content: &'a str,
}

/// Render some text as markdown.
pub fn Markdown<'a>(cx: Scope<'a, MarkdownProps<'a>>) -> Element {
    let parser = Parser::new(cx.props.content);

    let mut html_buf = String::new();
    pulldown_cmark::html::push_html(&mut html_buf, parser);

    cx.render(rsx! {
        div {
            id: "{cx.props.id}",
            class: "{cx.props.class}",
            dangerous_inner_html: "{html_buf}"
        }
    })
}
