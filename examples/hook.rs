use dioxus::prelude::*;
use dioxus_markdown::use_markdown;

fn main() {
    dioxus::desktop::launch(app);
}

fn app(cx: Scope) -> Element {
    let result = use_markdown(&cx, include_str!("../README.md"));
    cx.render(rsx! {
        link {
            rel: "stylesheet",
            href: "https://cdn.jsdelivr.net/npm/bulma@0.9.4/css/bulma.min.css",
        }
        div {
            class: "container is-fluid",
            div {
                class: "content",
                result
            }
        }
    })
}