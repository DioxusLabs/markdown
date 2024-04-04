use dioxus::prelude::*;
use dioxus_markdown::Markdown;

fn main() {
    dioxus::launch(app);
}

fn app() -> Element {
    let class = use_signal(|| String::from("content"));
    rsx! {
        link {
            rel: "stylesheet",
            href: "https://cdn.jsdelivr.net/npm/bulma@0.9.4/css/bulma.min.css"
        }
        div { class: "container is-fluid", Markdown { class: class, content: include_str!("../README.md") } }
    }
}
