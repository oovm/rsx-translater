#![allow(non_snake_case)]

use dioxus::prelude::*;
use rsx_convert::RsxBuilder;

#[derive(Props, PartialEq)]
pub struct CodeRendererData {
    is_error: bool,
    code: String,
}

pub fn CodeRenderer(cx: Scope<CodeRendererData>) -> Element {
    let class = match cx.props.is_error {
        true => "bg-warning text-neutral",
        false => "",
    };
    let code = cx.props.code.lines().enumerate().map(|(i, t)| rsx!(
        pre {
            class: "{class}",
            "data-prefix": "{i}",
            code {"{t}"}
        }
    ));
    cx.render(rsx!(code))
}

#[derive(Props, PartialEq)]
pub struct EditorSettings {
    /// fn
    is_component: bool,
    /// fn name() {}
    component_name: String,
    /// cx.render()
    is_renderer: bool,
    ///
    pre_indent: usize,
}

pub fn Editor(cx: Scope) -> Element {
    let text = use_state(&cx, || String::from("<span>content</span>"));
    let mut builder = RsxBuilder::default();
    let out = match builder.html_to_rsx(text.get()) {
        Ok(o) => rsx!(
            CodeRenderer {
                code: o,
                is_error: false,
            }
        ),
        Err(e) => rsx!(
            CodeRenderer {
                code: e.to_string(),
                is_error: true,
            }
        ),
    };

    cx.render(rsx!(
        div {
            class: "flex flex-col",
            div {
                class: "form-control",
                textarea {
                    class: "textarea h-96 textarea-bordered textarea-primary",
                    id: "editor",
                    placeholder: "<span>content</span>",
                    oninput: move |e| text.set(e.value.to_owned()),
                }
            }
            div {
                class: "mockup-code",
                out
            }
        }
        div {
            class: "form-control",
            label {
                class: "cursor-pointer label",
                span {
                    class: "label-text",
                    "Is Component"
                }
                input {
                    class: "checkbox",
                    checked: "checked",
                    r#type: "checkbox",
                }
            }
            label {
                class: "input-group input-group-sm",
                span {
                    "Component name"
                }
                input {
                    r#type: "text",
                    class: "input input-bordered input-sm",
                    value: "App",
                }
            }
            label {
                class: "cursor-pointer label",
                span {
                    class: "label-text",
                    "Is Renderer"
                }
                input {
                    r#type: "checkbox",
                    class: "checkbox",
                    checked: "checked",
                }
            }
            label {
                class: "cursor-pointer label",
                span {
                    class: "label-text",
                    "Pre Ident"
                }
                input {
                    r#type: "range",
                    class: "range",
                    max: "100",
                    value: "40",
                }
                span {
                    class: "label-text",
                    "0"
                }
            }
        }
    ))
}
