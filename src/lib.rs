use dioxus::core::exports::bumpalo;
use dioxus::core::*;
use pulldown_cmark::escape::{escape_href, escape_html, StrWrite, WriteWrapper};
use pulldown_cmark::{Alignment, CodeBlockKind, Event, LinkType, Parser, Tag};
use pulldown_cmark::{CowStr, Event::*};

fn render_markdown<'a>(cx: Context<'a>, text: &str) -> DomTree<'a> {
    cx.render(LazyNodes::new(|f| {
        let bump = f.bump();

        // While parsing, we keep a list of nodes that need to be
        let mut nodestack = Vec::<VNode>::new();
        let mut nodes = bumpalo::collections::Vec::new_in(bump);
        let parser = Parser::new(text);
        for line in parser {
            match line {
                Start(tag) => {
                    //
                    nodestack.push(match tag {
                        Tag::Paragraph => DelBuilder::new(&node_ctx, "p"),
                        Tag::Heading(i) => {
                            //
                            let tagnmae = match i {
                                1 => "h1",
                                2 => "h2",
                                3 => "h3",
                                4 => "h4",
                                5 => "h5",
                                _ => "h6",
                            };
                        }
                        // table stuff TODO
                        Tag::Table(_) => todo!(),
                        Tag::TableHead => todo!(),
                        Tag::TableRow => todo!(),
                        Tag::TableCell => todo!(),
                        Tag::BlockQuote => todo!(),

                        // TODO: integrate Codeblocks (syntect) for highlighting
                        Tag::CodeBlock(_) => todo!(),

                        // list stuff
                        Tag::List(Some(1)) => DelBuilder::new(&node_ctx, "ol"),
                        Tag::List(Some(start)) => DelBuilder::new(&node_ctx, "ol")
                            .attr("start", format_args!("{}", start)),
                        Tag::List(None) => DelBuilder::new(&node_ctx, "ul"),
                        Tag::Item => DelBuilder::new(&node_ctx, "li"),

                        // text formatting
                        Tag::Emphasis => DelBuilder::new(&node_ctx, "em"),
                        Tag::Strong => DelBuilder::new(&node_ctx, "strong"),
                        Tag::Strikethrough => DelBuilder::new(&node_ctx, "del"),

                        // links
                        Tag::Link(LinkType::Email, dest, title) => {
                            let b = DelBuilder::new(&node_ctx, "a")
                                .attr("href", format_args!("mailto:{}", dest));

                            if !title.is_empty() {
                                b.attr("title", format_args!("{}", title))
                            } else {
                                b
                            }
                        }

                        Tag::Link(_link_type, dest, title) => {
                            let b = DelBuilder::new(&node_ctx, "a")
                                .attr("href", format_args!("{}", dest));
                            if !title.is_empty() {
                                b.attr("title", format_args!("{}", title))
                            } else {
                                b
                            }
                        }

                        Tag::Image(_link_type, src, title) => {
                            let b = DelBuilder::new(&node_ctx, "img")
                                .attr("src", format_args!("{}", src));
                            if !title.is_empty() {
                                b.attr("title", format_args!("{}", title))
                            } else {
                                b
                            }
                        }
                        Tag::FootnoteDefinition(_) => todo!(),
                    });
                }
                End(tag) => {
                    //

                    match tag {
                        // close the top element
                        Tag::BlockQuote
                        | Tag::Paragraph
                        | Tag::Heading(_)
                        | Tag::List(Some(_))
                        | Tag::List(None)
                        | Tag::Emphasis
                        | Tag::Strong
                        | Tag::Strikethrough
                        | Tag::Link(_, _, _)
                        | Tag::Image(_, _, _)
                        | Tag::FootnoteDefinition(_)
                        | Tag::Item => {
                            let top = nodestack.pop().expect("and end must always follow a start");
                            nodes.push(top.finish())
                        }

                        // close the top two elements
                        Tag::CodeBlock(_) => {
                            // pop the top
                            // then make its wrapper
                        }

                        // close the top three elements
                        Tag::Table(_) | Tag::TableHead | Tag::TableRow | Tag::TableCell => todo!(),
                    }
                }
                Text(text) => {
                    let node = dioxus::nodebuilder::text3(bump, format_args_f!("{}", text));
                    match nodestack.pop() {
                        // Currently inside a tag, push a text node to the element
                        Some(top) => nodestack.push(top.iter_child(Some(node))),

                        // Outside of a tag, push the text in a just text
                        None => nodes.push(node),
                    }
                }
                Code(code) => {
                    todo!("WIP: integrate the syntect grammar");
                    // let node = DelBuilder::new(&n, "code")
                    //
                    todo!("what is a code block??");
                }
                Html(htm) => {
                    //
                    todo!("What is an html block?");
                }

                // not sure yet how to add these
                // see the html implementation
                // https://github.com/raphlinus/pulldown-cmark/blob/8db4423ce78a1dad8a8377e0f25ebd48e0e534fc/src/html.rs#L120-L127
                FootnoteReference(tf) => todo!("tf are footnotes?"),
                SoftBreak => todo!("tf are SoftBreak?"),
                HardBreak => todo!("tf are HardBreak?"),
                Rule => todo!("tf are Rule?"),
                TaskListMarker(_) => todo!("tf are TaskListMarker?"),
            }
        }
        nodes.into_bump_slice()
    }))
}

// #[test]
// fn markdown_parses() {
//     use dioxus_ssr::TextRenderer;

//     TextRenderer::<()>::new(|ctx| {
//         let nds = ctx.render2(LazyNodeList::new(markdown_vnode_builder(include_str!(
//             "./sample.md"
//         ))));

//         ctx.render(rsx! {
//             div {
//                 {nds}
//             }
//         })
//     });
// }
