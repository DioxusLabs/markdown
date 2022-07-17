#![allow(non_snake_case)]

use comrak::{
    markdown_to_html,
    nodes::{AstNode, NodeCode, NodeValue},
    parse_document, Arena, ComrakOptions,
};
use dioxus::prelude::{dioxus_elements::blockquote, *};

#[derive(Props)]
pub struct MarkdownProps<'a> {
    #[props(default)]
    style: &'a str,
    #[props(default)]
    id: &'a str,
    #[props(default)]
    class: &'a str,

    content: &'a str,
}

pub fn use_markdown<'a>(cx: &'a ScopeState, content: &'a str) -> Element<'a> {
    let factory = NodeFactory::new(cx);
    let mut eles: Vec<VNode> = vec![];
    cx.use_hook(|_| {
        let arena = Arena::new();
        let options = ComrakOptions::default();
        let root = parse_document(&arena, content, &options);
        for node in root.children() {
            println!("{:?}", node.data.borrow().value);
            println!("{:#?}", node.children());
            println!("");
            let r: Element = match node.data.borrow().value.clone() {
                NodeValue::Heading(h) => {
                    let text = collect_text(node);
                    match h.level {
                        1 => cx.render(rsx! {
                            h1 {
                                "{text}"
                            }
                        }),
                        2 => cx.render(rsx! {
                            h2 {
                                "{text}"
                            }
                        }),
                        3 => cx.render(rsx! {
                            h3 {
                                "{text}"
                            }
                        }),
                        4 => cx.render(rsx! {
                            h4 {
                                "{text}"
                            }
                        }),
                        5 => cx.render(rsx! {
                            h5 {
                                "{text}"
                            }
                        }),
                        6 => cx.render(rsx! {
                            h6 {
                                "{text}"
                            }
                        }),
                        _ => None,
                    }
                }
                NodeValue::BlockQuote => {
                    let mut buffer = vec![];
                    collect_children(node, &mut buffer, false);
                    let child_node = buffer.iter().map(|i| {
                        rsx! {
                            p { "{i}" }
                        }
                    });
                    cx.render(rsx! {
                        blockquote {
                            child_node
                        }
                    })
                }
                _ => None,
            };
            if let Some(r) = r {
                eles.push(r);
            }
        }
    });
    Some(factory.fragment_root(eles))
}

fn collect_children<'a>(node: &'a AstNode<'a>, output: &mut Vec<String>, space_break: bool) {
    match node.data.borrow().value {
        NodeValue::Text(ref literal) | NodeValue::Code(NodeCode { ref literal, .. }) => {
            let mut buffer: Vec<u8> = vec![];
            buffer.extend_from_slice(literal);
            let buffer = String::from_utf8(buffer).unwrap();
            if space_break && output.len() > 0 {
                let index = output.len() - 1;
                output[index] += &buffer;
            } else {
                output.push(buffer);
            }
        }
        NodeValue::SoftBreak | NodeValue::LineBreak => {
            if space_break {
                let index = output.len() - 1;
                output[index] += " ";
            }
        },
        _ => {
            for child in node.children() {
                collect_children(child, output, space_break);
            }
        }
    }
}

fn collect_paragraph<'a>(node: &'a AstNode<'a>) -> Vec<LazyNodes> {
    let mut buffer = vec![];
    let mut result = vec![];
    collect_children(node, &mut buffer, false);
    for text in buffer {
        result.push(rsx! {
            p { "{text}" }
        })
    }
    result
}

fn collect_text<'a>(node: &'a AstNode<'a>) -> String {
    let mut buffer = vec![];
    collect_children(node, &mut buffer, true);
    buffer[0].clone()
}

/// Render some text as markdown.
pub fn Markdown<'a>(cx: Scope<'a, MarkdownProps<'a>>) -> Element {
    let raw_html = markdown_to_html(cx.props.content, &ComrakOptions::default());
    cx.render(rsx! {
        div {
            id: "{cx.props.id}",
            class: "{cx.props.class}",
            style: "{cx.props.style}",
            dangerous_inner_html: "{raw_html}"
        }
    })
}
