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
    let text = use_state(&cx, || String::new());
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
            textarea {
                id: "editor",
                oninput: move |e| text.set(e.value.to_owned()),
            }
            out
        }
    ))
}
