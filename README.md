# Dioxus-Markdown: convert markdown to dioxus VNodes

Easily convert markdown into Dioxus VNodes.

```rust
static Example: FC<()> = |(cx, props)| {
    let contents = include_str!("example.md");
    dioxus_markdown::render_markdown(cx, contents)?
};
```

## Features

- Convert strings to vnodes on the fly with `to_vnodes`
- (wip) Statically convert markdown to dioxus with `to_vnodes!` 
- (wip) Load files directly with `from_file!`

## Warning:

- Currently, this crate uses the pulldown-cmark to html converter with no actual intermediate step to Dioxus VNodes. 
- Content is set with `dangerous_inner_html` with no actual translation to VNodes occurring.
- Macros are not currently implemented.

For most use cases, this approach will work fine. However, if you feel brave enough to add a true Markdown to VNode converter, we'd happily coach and assist the implementation/pull request.


## Usage notes

Translation occurs at runtime and can be expensive on first load. Use SSR and hydration to pre-render important pages. 

