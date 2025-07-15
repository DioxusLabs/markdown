# Dioxus Markdown

> Easily convert markdown into Dioxus

```rust
let class = use_signal(|| String::from("content"));
rsx! {
    Markdown {
        class: class,
        content: include_str!("../README.md"),
    }
}
```
