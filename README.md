# Dioxus Markdown

> Easily convert markdown into Dioxus

```rust
let class = use_signal(|| String::from("content"));
let content = use_signal(|| String::from(include_str!("../README.md")));
rsx! {
    Markdown {
        class: class,
        content: content,
    }
}
```

## Features

- Convert strings to vnodes on the fly with `tvnodes`

## Warning:

- Currently, this crate uses the pulldown-cmark to html converter with no actual intermediate step to Dioxus VNodes.
- Content is set with `dangerous_inner_html` with no actual translation to VNodes occurring.
- Macros are not currently implemented.

For most use cases, this approach will work fine. However, if you feel brave enough to add a true Markdown to VNode converter, we'd happily coach and assist the implementation/pull request.
