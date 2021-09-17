mod writer;
mod svg;

use std::{
    fmt::{Display, Formatter, Write},
};
use std::fmt::Arguments;

use anyhow::Result;

use html_parser::{Dom, Element, Node};

use crate::Result;

pub struct RsxBuilder {
    buffer: String,
    image_cache: PngCache,
    svg_cache: Vec<Element>,
}


pub struct PngCache {}

pub struct SvgCache {
    inner: Vec<Element>,
}

impl RsxBuilder {
    #[inline]
    pub fn html_to_rsx(&mut self, html: &str) -> Result<String> {
        self.reset();
        Dom::parse(html)?.as_rsx(self)
    }
    #[inline]
    pub fn html_to_rs(&mut self, html: &str) -> Result<String> {
        self.reset();
        Dom::parse(html)?.as_rs(self)
    }
    pub fn reset(&mut self) {

    }
}

impl RsxBuilder {
    pub fn render_svg(&self) {}
}

pub trait AsRsx {
    fn as_rsx(&self, ctx: &mut RsxBuilder) -> Result<String>;
    fn as_rs(&self, ctx: &mut RsxBuilder) -> Result<String>;
}

impl AsRsx for Dom {
    fn as_rsx(&self, f: &mut RsxBuilder) -> Result<String> {
        let app = r#"
        fn App() {

        }
        "#;
    }

    fn as_rs(&self, f: &mut RsxBuilder) -> Result<String> {
        todo!()
    }
}

impl AsRsx for Node {
    fn as_rsx(&self, f: &mut RsxBuilder) -> Result<String> {
        match self {
            Self::Text(t) => Ok(format!(f, "\"{}\"", t)),
            Self::Comment(c) => Ok(format!(f, "/* {} */", c)),
            Self::Element(e) if e.name.eq("svg") => e.as_rsx(ctx),
            Self::Element(e) => e.as_rsx(ctx),
        }
    }

    fn as_rs(&self, f: &mut RsxBuilder) -> Result<String> {
        match self {
            Self::Text(t) => Ok(format!(f, "\"{}\"", t)),
            Self::Element(e) => e.as_rsx(ctx),
            Self::Comment(c) => Ok(format!(f, "/* {} */", c)),
        }
    }
}

impl AsRsx for Element {
    fn as_rsx(&self, f: &mut RsxBuilder) -> Result<String> {
        todo!()
    }

    fn as_rs(&self, f: &mut RsxBuilder) -> Result<String> {
        todo!()
    }
}

pub fn convert_html_to_component(html: &str) -> Result<ComponentRenderer> {
    Ok(ComponentRenderer { dom: Dom::parse(html)?, icon_index: 0 })
}

pub struct ComponentRenderer {
    dom: Dom,
    icon_index: usize,
}

impl Display for ComponentRenderer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            r##"
fn component(cx: Scope) -> Element {{
    cx.render(rsx!("##
        )?;
        let mut svg_nodes = vec![];

        let mut svg_idx = 0;
        for child in &self.dom.children {
            render_child(f, child, 2, &mut svg_nodes, true, &mut svg_idx)?;
        }
        write!(
            f,
            r##"    ))
}}"##
        )?;

        if svg_idx == 0 {
            return Ok(());
        }

        writeln!(f, "\n\nmod icons {{")?;

        let mut id = 0;
        while let Some(svg) = svg_nodes.pop() {
            writeln!(
                f,
                r##"    pub(super) fn icon_{}(cx: Scope) -> Element {{
        cx.render(rsx!("##,
                id
            )?;
            write_tabs(f, 3)?;

            render_element(f, svg, 3, &mut svg_nodes, false, &mut 0)?;
            writeln!(f, "\t\t))\n\t}}\n")?;
            id += 1;
        }

        writeln!(f, "}}")?;

        Ok(())
    }
}

fn render_child<'a>(
    f: &mut Formatter<'_>,
    child: &'a Node,
    il: u32,
    svg_buffer: &mut Vec<&'a Element>,
    skip_svg: bool,
    svg_idx: &mut usize,
) -> std::fmt::Result {
    write_tabs(f, il)?;
    match child {
        Node::Text(t) => writeln!(f, "\"{}\"", t)?,
        Node::Comment(e) => writeln!(f, "/* {} */", e)?,
        Node::Element(el) => render_element(f, el, il, svg_buffer, skip_svg, svg_idx)?,
    };
    Ok(())
}

fn render_element<'a>(
    f: &mut Formatter<'_>,
    el: &'a Element,
    il: u32,
    svg_buffer: &mut Vec<&'a Element>,
    skip_svg: bool,
    svg_idx: &mut usize,
) -> std::fmt::Result {
    if el.name == "svg" && skip_svg {
        svg_buffer.push(el);
        // todo: attach the right icon ID
        writeln!(f, "icons::icon_{} {{}}", svg_idx)?;
        *svg_idx += 1;
        return Ok(());
    }

    // open the tag
    write!(f, "{} {{ ", &el.name)?;

    // todo: dioxus will eventually support classnames
    // for now, just write them with a space between each
    let class_iter = &mut el.classes.iter();
    if let Some(first_class) = class_iter.next() {
        write!(f, "class: \"{}", first_class)?;
        for next_class in class_iter {
            write!(f, " {}", next_class)?;
        }
        write!(f, "\",")?;
    }
    write!(f, "\n")?;

    // write the attributes
    if let Some(id) = &el.id {
        write_tabs(f, il + 1)?;
        writeln!(f, "id: \"{}\",", id)?;
    }

    for (name, value) in &el.attributes {
        write_tabs(f, il + 1)?;

        use convert_case::{Case, Casing};
        if name.chars().any(|ch| ch.is_ascii_uppercase() || ch == '-') {
            let new_name = name.to_case(Case::Snake);
            match value {
                Some(val) => writeln!(f, "{}: \"{}\",", new_name, val)?,
                None => writeln!(f, "{}: \"\",", new_name)?,
            }
        } else {
            match name.as_str() {
                "for" | "async" | "type" | "as" => write!(f, "r#")?,
                _ => {}
            }

            match value {
                Some(val) => writeln!(f, "{}: \"{}\",", name, val)?,
                None => writeln!(f, "{}: \"\",", name)?,
            }
        }
    }

    // now the children
    for child in &el.children {
        render_child(f, child, il + 1, svg_buffer, skip_svg, svg_idx)?;
    }

    // close the tag
    write_tabs(f, il)?;
    writeln!(f, "}}")?;
    Ok(())
}

fn write_tabs(f: &mut Formatter, num: u32) -> std::fmt::Result {
    for _ in 0..num {
        write!(f, "    ")?
    }
    Ok(())
}

#[test]
fn generates_svgs() {
    let st = include_str!("../../../tests/svg.html");

    let out = format!("{:}", convert_html_to_component(st).unwrap());
    dbg!(&out);

    std::fs::File::create("svg_rsx.rs").unwrap().write_all(out.as_bytes()).unwrap();
}
