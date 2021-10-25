#![allow(non_snake_case)]
#![feature(arbitrary_self_types)]

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

pub struct RsxBuilderComponent {
    inner: RsxBuilder,
}

impl Default for RsxBuilderComponent {
    fn default() -> Self {
        Self {
            inner: Default::default()
        }
    }
}

impl RsxBuilderComponent {
    fn render(self: UseState<Self>, input: &str) -> LazyNodes {
        let rendered = self.inner.clone().html_to_rsx(input);
        match rendered {
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
        }
    }
    fn set_component_name(self: UseState<Self>, e: FormEvent) {
        let new = self.inner.clone().set_name(e.value.to_owned());
        self.set(Self { inner: new })
    }
    fn set_as_component(self: UseState<Self>, e: FormEvent) {
        let new = match e.value.parse::<bool>() {
            Ok(o) => { self.inner.clone().config_component(o) }
            Err(_) => { return; }
        };
        self.set(Self { inner: new })
    }
    fn set_as_renderer(self: UseState<Self>, e: FormEvent) {
        let new = match e.value.parse::<bool>() {
            Ok(o) => { self.inner.clone().config_renderer(o) }
            Err(_) => { return; }
        };
        self.set(Self { inner: new })
    }
    fn set_pre_indent(self: UseState<Self>, e: FormEvent) {
        let new = match e.value.parse::<usize>() {
            Ok(o) => { self.inner.clone().preset_indent(o) }
            Err(_) => { return; }
        };
        self.set(Self { inner: new })
    }
}

pub fn Editor(cx: Scope) -> Element {
    let text = use_state(&cx, || String::new());
    let cfg = use_state(&cx, || RsxBuilderComponent::default());
    let component_name = cfg.inner.config.component_name.to_owned();
    let is_component = cfg.inner.config.is_component;
    let is_renderer = cfg.inner.config.is_renderer;
    let pre_indent = cfg.inner.config.indent_pre;

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
                cfg.render(text.get())
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
                    r#type: "checkbox",
                    class: "checkbox",
                    checked: "{is_component}",
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
                    value: "{component_name}",
                    oninput: move |e| cfg.set_component_name(e)
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
                    checked: "{is_renderer}",
                    oninput: move |e| cfg.set_as_renderer(e)
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
                    min: "0",
                    max: "10",
                    step: "1",
                    value: "{pre_indent}",
                    oninput: move |e| cfg.set_pre_indent(e)
                }
                span {
                    class: "label-text",
                    "0"
                }
            }
            a {
                href: "https://github.com/oovm/rsx-translater/issues",
                target: "_blank",
                button {
                    class: "py-2 px-4 mr-2 mb-2 text-sm font-medium text-gray-900 bg-white rounded-lg border border-gray-200 hover:bg-gray-100 hover:text-blue-700 focus:z-10 focus:ring-2 focus:ring-blue-700 focus:text-blue-700 dark:bg-gray-800 dark:text-gray-400 dark:border-gray-600 dark:hover:text-white dark:hover:bg-gray-700",
                    r#type: "button",
                    "Report bug on github"
                }
            }
        }
    ))
}
