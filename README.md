# Dioxus-Markdown: convert markdown to dioxus VNodes

Easily convert markdown into Dioxus VNodes.

```rust
static Example: FC<()> = |(cx, props)| {
    let contents = include_str!("example.md");
    cx.render(dioxus_markdown::to_vnodes(contents)?)
};
```

## Features

- Convert strings to vnodes on the fly with `to_vnodes`
- Statically convert markdown to dioxus with `to_vnodes!` 
- Load files directly with `from_file!`

## Usage notes

Translation occurs at runtime and can be expensive on first load. Use SSR and hydration to pre-render important pages. 

## Configuration

A few things are configurable:

- code blocks (by default with `Codeblocks` but can be disabled for the `code` html element)
- tables (by default with the `Table` mechanism but can be swapped out)
- images
- math (by default with a code block, but can be rendered with KaTeX)

Inline code highlighting is given by the accompanying library `Codeblocks`

All nodes are returned as a `Vec<VNode>`. If you have a specific format, you'll need to parse the streamed output.

This markdown input:

    # Test

    Paragraph

    ```rust
    fn main() {}
    ```

Will be roughly equivalent to:

```rust
let nodes = markdown_to_vnodes(ctx, text);
let [header, paragraph, codeblock] = &nodes[..].unwrap();
```

You can then render them directly into your site or reorganize them as you see fit:

```rust
rsx!{
    div {
        div { class: "left-col"
            {header}
            {paragraph}
        }
        div { class: "right-col"
            {codeblock}
        }
    }
}

```
