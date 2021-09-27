#![allow(non_snake_case)]

use dioxus::prelude::*;
use rsx_platform_free::Editor;


fn main() {
    dioxus::desktop::launch(AppDesktop);
}

pub fn AppDesktop(cx: Scope) -> Element {
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