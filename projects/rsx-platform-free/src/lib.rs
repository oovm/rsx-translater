#![allow(non_snake_case)]
#![feature(arbitrary_self_types)]

use std::borrow::BorrowMut;
use std::cell::RefMut;
use std::ops::DerefMut;
use std::str::ParseBoolError;
use dioxus::events::{FormEvent};
use dioxus::prelude::*;
use rsx_convert::RsxBuilder;

#[derive(Props, PartialEq)]
pub struct CodeRendererP {
    is_error: bool,
    code: String,
}

pub fn CodeRenderer(cx: Scope<CodeRendererP>) -> Element {
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

impl Default for EditorSettings {
    fn default() -> Self {
        Self {
            is_component: false,
            component_name: "App".to_string(),
            is_renderer: false,
            pre_indent: 0
        }
    }
}

impl EditorSettings {
    pub fn set_as_component(self: UseState<Self>, e: FormEvent) {
        let is_component = match e.value.parse::<bool>() {
            Ok(o) => {o}
            Err(_) => {return }
        };
        // let new = Self {
        //     is_component,
        //     component_name: "".to_string(),
        //     is_renderer: false,
        //     pre_indent: 0
        // };
        match self.get_wip_mut().deref_mut() {
            None => {}
            Some(s) => {s.is_component = is_component}
        }
        self.needs_update();
        // self.set(new)
    }
}

pub trait RsxBuilderEffect {
    fn render(self: UseState<Self>, input: &str) -> LazyNodes;
}

impl RsxBuilderEffect for RsxBuilder {
    fn render(self: UseState<Self>, input: &str) -> LazyNodes {
        let mut new = self.get().clone();
        let out = match new.html_to_rsx(input) {
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
        self.set(new);
        out
    }
}

pub fn Editor(cx: Scope) -> Element {
    let text = use_state(&cx, || String::new());
    let cfg = use_state(&cx, || EditorSettings::default());

    cx.render(rsx!(
        div {
            class: "flex flex-row",
            div {
                class: "form-control flex-1",
                textarea {
                    class: "textarea h-96 textarea-bordered textarea-primary",
                    id: "editor",
                    placeholder: "<span>content</span>",
                    oninput: move |e| text.set(e.value.to_owned()),
                }
            }
            div {
                class: "mockup-code flex-1",
                cfg.get().render(text.get())
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
                    oninput: move |e| cfg.set_as_component(e)
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
                    // oninput: set_component
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
                    // oninput: move |e| cfg.set(cfg.set_is_r(e))
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
                    // oninput: set_component
                }
                span {
                    class: "label-text",
                    "0"
                }
            }
        }
    ))
}
