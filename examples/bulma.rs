use dioxus::prelude::*;
use dioxus_markdown::Markdown;

fn main() {
    dioxus::desktop::launch(app);
}

fn app(cx: Scope) -> Element {
    cx.render(rsx! {
        link {
            rel: "stylesheet",
            href: "https://cdn.jsdelivr.net/npm/bulma@0.9.4/css/bulma.min.css",
        }
        div {
            class: "container is-fluid",
            Markdown {
                class: "content",
                content: include_str!("../README.md"),
            }
        }
    })
}