#![allow(non_snake_case)]

use dioxus::prelude::*;
use rsx_convert::RsxBuilder;

fn main() {
    dioxus::web::launch(AppWeb)
}

pub fn main_ssr() {
    let mut vdom = VirtualDom::new(AppWeb);
    let _ = vdom.rebuild();
    println!("{}", dioxus::ssr::render_vdom(&vdom));
}

pub fn AppWeb(cx: Scope) -> Element {
    cx.render(rsx! {
        Editor {

        }
    })
}