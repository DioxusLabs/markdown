#![allow(non_snake_case)]

use dioxus::prelude::*;
use markdown::{self, mdast, Constructs, ParseOptions};

#[derive(Props, Clone, PartialEq)]
pub struct MarkdownProps {
    #[props(default)]
    id: Signal<String>,
    #[props(default)]
    class: Signal<String>,
    content: ReadOnlySignal<String>,
    #[props(default)]
    error_view: Option<Element>
}

/// Render some text as markdown.
pub fn Markdown(props: MarkdownProps) -> Element {
    let content = &*props.content.read();
    // TODO: Ideally we should allow parse options to be customizable externally
    let parse_options = ParseOptions {
        constructs: Constructs {
            attention: true,
            autolink: true,
            block_quote: true,
            character_escape: true,
            character_reference: true,
            code_indented: true,
            code_fenced: true,
            code_text: true,
            definition: true,
            frontmatter: false,
            gfm_autolink_literal: true,
            gfm_label_start_footnote: false,
            gfm_footnote_definition: false,
            gfm_strikethrough: true,
            gfm_table: true,
            gfm_task_list_item: true,
            hard_break_escape: true,
            hard_break_trailing: true,
            heading_atx: true,
            heading_setext: true,
            html_flow: false,
            html_text: false,
            label_start_image: true,
            label_start_link: true,
            label_end: true,
            list_item: true,
            math_flow: true,
            math_text: true,
            mdx_esm: false,
            mdx_expression_flow: false,
            mdx_expression_text: false,
            mdx_jsx_flow: false,
            mdx_jsx_text: false,
            thematic_break: true,
        },
        gfm_strikethrough_single_tilde: false,
        math_text_single_dollar: false,
        mdx_expression_parse: None,
        mdx_esm_parse: None,
    };
    let mdast = match markdown::to_mdast(content, &parse_options) {
        Ok(mdast) => mdast,
        Err(err) => return props.error_view.unwrap_or(rsx! { "Error parsing markdown: {err}" })
    };

    rsx! {
        div {
            id: "{props.id}",
            class: "{props.class}",
            ASTNodeView { node: mdast }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct ASTNodeViewProps {
    node: mdast::Node,
}

pub fn ASTNodeView(props: ASTNodeViewProps) -> Element {
    // TODO: Allow customization for views that want to customize behavior for each markdown node
    match props.node {
        mdast::Node::Root(root) => {
            rsx! {
                article {
                    for child in root.children {
                        ASTNodeView { node: child }
                    }
                }
            }
        },
        mdast::Node::Blockquote(blockquote) => {
            rsx! {
                blockquote { 
                    for child in blockquote.children {
                        ASTNodeView { node: child }
                    }
                }
            }
        },
        mdast::Node::FootnoteDefinition(footnote_definition) => {
            rsx! {
                div {
                    for child in footnote_definition.children {
                        ASTNodeView { node: child }
                    }
                }
            }
        },
        mdast::Node::MdxJsxFlowElement(mdx_jsx_flow_element) => {
            rsx! {
                div {
                    for child in mdx_jsx_flow_element.children {
                        ASTNodeView { node: child }
                    }
                }
            }
        },
        mdast::Node::List(list) => {
            if list.ordered {
                rsx! {
                    ol {
                        start: if list.start.unwrap_or(1) != 1 { "{list.start.unwrap_or(1)}" } else { "" },
                        for child in list.children {
                            ASTNodeView { node: child }
                        }
                    }
                }
            } else {
                rsx! {
                    ul {
                        for child in list.children {
                            ASTNodeView { node: child }
                        }
                    }
                }
            }
        },
        mdast::Node::MdxjsEsm(mdxjs_esm) => {
            rsx! {
                pre {
                    code {
                        "{mdxjs_esm.value}"
                    }
                }
            }
        },
        mdast::Node::Toml(toml) => {
            rsx! {
                pre {
                    code {
                        "{toml.value}"
                    }
                }
            }
        },
        mdast::Node::Yaml(yaml) => {
            rsx! {
                pre {
                    code {
                        "{yaml.value}"
                    }
                }
            }
        },
        mdast::Node::Break(_) => {
            rsx! { br {} }
        },
        mdast::Node::InlineCode(inline_code) => {
            rsx! {
                code {
                    "{inline_code.value}"
                }
            }
        },
        mdast::Node::InlineMath(inline_math) => {
            rsx! {
                span {
                    "{inline_math.value}"
                }
            }
        },
        mdast::Node::Delete(delete) => {
            rsx! {
                del {
                    for child in delete.children {
                        ASTNodeView { node: child }
                    }
                }
            }
        },
        mdast::Node::Emphasis(emphasis) => {
            rsx! {
                em {
                    for child in emphasis.children {
                        ASTNodeView { node: child }
                    }
                }
            }
        },
        mdast::Node::MdxTextExpression(mdx_text_expression) => {
            rsx! {
                span {
                    "{mdx_text_expression.value}"
                }
            }
        },
        mdast::Node::FootnoteReference(footnote_reference) => {
            rsx! {
                sup {
                    a {
                        href: "#footnote-{footnote_reference.identifier}",
                        "[{footnote_reference.identifier}]"
                    }
                }
            }
        },
        mdast::Node::Html(html) => {
            rsx! {
                div {
                    dangerous_inner_html: "{html.value}"
                }
            }
        },
        mdast::Node::Image(image) => {
            rsx! {
                img {
                    src: "{image.url}",
                    alt: "{image.alt}",
                    title: image.title.unwrap_or_default()
                }
            }
        },
        mdast::Node::ImageReference(image_reference) => {
            rsx! {
                span {
                    "[{image_reference.identifier}]"
                }
            }
        },
        mdast::Node::MdxJsxTextElement(mdx_jsx_text_element) => {
            rsx! {
                span {
                    for child in mdx_jsx_text_element.children {
                        ASTNodeView { node: child }
                    }
                }
            }
        },
        mdast::Node::Link(link) => {
            rsx! {
                a {
                    href: "{link.url}",
                    title: link.title.unwrap_or_default(),
                    for child in link.children {
                        ASTNodeView { node: child }
                    }
                }
            }
        },
        mdast::Node::LinkReference(link_reference) => {
            rsx! {
                span {
                    "[{link_reference.identifier}]"
                }
            }
        },
        mdast::Node::Strong(strong) => {
            rsx! {
                strong {
                    for child in strong.children {
                        ASTNodeView { node: child }
                    }
                }
            }
        },
        mdast::Node::Text(text) => {
            rsx! {
                "{text.value}"
            }
        },
        mdast::Node::Code(code) => {
            rsx! {
                pre {
                    class: if let Some(lang) = &code.lang { "language-{lang}" } else { "" },
                    code {
                        "{code.value}"
                    }
                }
            }
        },
        mdast::Node::Math(math) => {
            rsx! {
                div {
                    "{math.value}"
                }
            }
        },
        mdast::Node::MdxFlowExpression(mdx_flow_expression) => {
            rsx! {
                div {
                    "{mdx_flow_expression.value}"
                }
            }
        },
        mdast::Node::Heading(heading) => {
            match heading.depth {
                1 => rsx! { h1 { for child in heading.children { ASTNodeView { node: child } } } },
                2 => rsx! { h2 { for child in heading.children { ASTNodeView { node: child } } } },
                3 => rsx! { h3 { for child in heading.children { ASTNodeView { node: child } } } },
                4 => rsx! { h4 { for child in heading.children { ASTNodeView { node: child } } } },
                5 => rsx! { h5 { for child in heading.children { ASTNodeView { node: child } } } },
                _ => rsx! { h6 { for child in heading.children { ASTNodeView { node: child } } } },
            }
        },
        mdast::Node::Table(table) => {
            rsx! {
                table {
                    for child in table.children {
                        ASTNodeView { node: child }
                    }
                }
            }
        },
        mdast::Node::ThematicBreak(_) => {
            rsx! {
                hr {}
            }
        },
        mdast::Node::TableRow(table_row) => {
            rsx! {
                tr {
                    for child in table_row.children {
                        ASTNodeView { node: child }
                    }
                }
            }
        },
        mdast::Node::TableCell(table_cell) => {
            rsx! {
                td {
                    for child in table_cell.children {
                        ASTNodeView { node: child }
                    }
                }
            }
        },
        mdast::Node::ListItem(list_item) => {
            rsx! {
                li {
                    if let Some(checked) = list_item.checked {
                        input {
                            r#type: "checkbox",
                            checked: checked,
                            disabled: true,
                        }
                    }
                    span {
                        for child in list_item.children {
                            ASTNodeView { node: child }
                        }
                    }
                }
            }
        },
        mdast::Node::Definition(_) => {
            todo!()
        },
        mdast::Node::Paragraph(paragraph) => {
            rsx! {
                p {
                    for child in paragraph.children {
                        ASTNodeView { node: child }
                    }
                }
            }
        },
    }
}
