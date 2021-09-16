//! Example: Basic Tailwind usage
//!
//! This example shows how an app might be styled with TailwindCSS.
//!
//! To minify your tailwind bundle, currently you need to use npm. Follow these instructions:
//!
//!     https://dev.to/arctic_hen7/how-to-set-up-tailwind-css-with-yew-and-trunk-il9
#![allow(non_snake_case)]

use dioxus::prelude::*;


fn main() {
    dioxus::desktop::launch(app);
}

pub fn app(cx: Scope) -> Element {
    cx.render(rsx!(
        link { href:"https://cdn.jsdelivr.net/npm/daisyui@1.24.3/dist/full.css", rel:"stylesheet" }
        link { href:"https://cdn.jsdelivr.net/npm/tailwindcss@2.2/dist/tailwind.min.css", rel:"stylesheet" }
    ))
}

pub fn StacksIcon(cx: Scope) -> Element {
    cx.render(rsx!(
        svg {
            fill: "none",
            stroke: "currentColor",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            stroke_width: "2",
            class: "w-10 h-10 text-white p-2 bg-indigo-500 rounded-full",
            view_box: "0 0 24 24",
            path { d: "M12 2L2 7l10 5 10-5-10-5zM2 17l10 5 10-5M2 12l10 5 10-5"}
        }
    ))
}

pub fn RightArrowIcon(cx: Scope) -> Element {
    cx.render(rsx!(
        svg {
            fill: "none",
            stroke: "currentColor",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            stroke_width: "2",
            class: "w-4 h-4 ml-1",
            view_box: "0 0 24 24",
            path { d: "M5 12h14M12 5l7 7-7 7"}
        }
    ))
}