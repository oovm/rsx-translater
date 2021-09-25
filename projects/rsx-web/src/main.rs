//! Example: Basic Tailwind usage
//!
//! This example shows how an app might be styled with TailwindCSS.
//!
//! To minify your tailwind bundle, currently you need to use npm. Follow these instructions:
//!
//!     https://dev.to/arctic_hen7/how-to-set-up-tailwind-css-with-yew-and-trunk-il9
#![allow(non_snake_case)]

use dioxus::prelude::*;
use rsx_convert::RsxBuilder;
use crate::dioxus_elements::span;

fn main() {
    dioxus::desktop::launch(App);
}

pub fn App(cx: Scope) -> Element {
    cx.render(rsx! {
        link {
            href: "https://cdn.jsdelivr.net/npm/daisyui@1.24.3/dist/full.css",
            rel: "stylesheet",

        }
        link {
            href: "https://cdn.jsdelivr.net/npm/tailwindcss@2.2/dist/tailwind.min.css",
            rel: "stylesheet",

        }
        Editor {

        }
    })
}

pub fn Editor(cx: Scope) -> Element {
    let text = use_state(&cx, || String::from("<span>content</span>"));
    let mut builder = RsxBuilder::default();
    let out = match builder.html_to_rsx(text.get()) {
        Ok(o) => rsx!(
            pre {code {"{o}"}}
        ),
        Err(e) => rsx!(
            pre {
                code {
                    class: "text-red-400",
                    "{e}"
                }
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
                pre {
                    data_prefix: "1",
                    code {
                        "npm i daisyui"
                    }
                }
                pre {
                    data_prefix: "2",
                    code {
                        "installing..."
                    }
                }
                pre {
                    class: "bg-warning text-neutral",
                    data_prefix: "3",
                    code {
                        "Error!"
                    }
                }
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
                    value: "20.99",
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
                    "4"
                }
            }
        }
    ))
}
