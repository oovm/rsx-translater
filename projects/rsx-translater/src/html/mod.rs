mod writer;
mod svg;
mod config;

use std::{
    fmt::{Write},
};
use std::collections::BTreeMap;
use std::fmt::Arguments;
use std::iter::FromIterator;
use html_parser::{Dom, Element, Node};
use crate::{AsRsx, Result};
use std::fmt::{Debug, Formatter};

#[derive(Clone)]
pub struct RsxBuilder {
    pub config: RsxBuilderConfig,
    buffer: String,
    indent_now: usize,
    svg_cache: Vec<Element>,
}

#[derive(Clone, Debug)]
pub struct RsxBuilderConfig {
    pub component_name: String,
    pub indent_size: usize,
    pub indent_pre: usize,
    pub is_renderer: bool,
    pub is_component: bool,
}

impl RsxBuilder {
    #[inline]
    pub fn html_to_rsx(&mut self, html: &str) -> Result<String> {
        self.reset();
        Dom::parse(html)?.write_rsx(self)?;
        Ok(self.buffer.to_owned())
    }
    #[inline]
    pub fn html_to_rs(&mut self, html: &str) -> Result<String> {
        self.reset();
        Dom::parse(html)?.write_rs(self)?;
        Ok(self.buffer.to_owned())
    }
    #[inline]
    pub fn reset(&mut self) {
        self.buffer.clear();
        self.svg_cache.clear();
        self.indent_now = self.config.indent_pre * self.config.indent_size;
    }
}


impl AsRsx for Dom {
    fn write_rsx(&self, f: &mut RsxBuilder) -> Result<()> {
        f.write_indent()?;
        f.write_component_l()?;
        f.write_render_l()?;
        for i in &self.children {
            i.write_rsx(f)?;
            f.write_newline()?;
        }
        f.write_render_r()?;
        f.write_component_r()?;
        f.write_svg()?;
        Ok(())
    }

    fn write_rs(&self, _: &mut RsxBuilder) -> Result<()> {
        todo!()
    }
}

impl AsRsx for Node {
    fn write_rsx(&self, f: &mut RsxBuilder) -> Result<()> {
        match self {
            Self::Text(t) => { write!(f, "{:?}", t)? }
            Self::Comment(c) => write!(f, "/* {} */", c)?,
            Self::Element(e) if e.name.eq("svg") => f.collect_svg(e)?,
            Self::Element(e) => e.write_rsx(f)?,
        }
        Ok(())
    }

    fn write_rs(&self, f: &mut RsxBuilder) -> Result<()> {
        match self {
            Self::Text(t) => writeln!(f, "\"{}\"", t)?,
            Self::Comment(c) => writeln!(f, "/* {} */", c)?,
            Self::Element(e) if e.name.eq("svg") => f.collect_svg(e)?,
            Self::Element(e) => e.write_rs(f)?,
        }
        Ok(())
    }
}


impl AsRsx for Element {
    fn write_rsx(&self, f: &mut RsxBuilder) -> Result<()> {
        // ?????????
        f.write_str(&self.name)?;
        f.write_char(' ')?;
        f.write_brace_l()?;
        // html ??????
        // todo: dioxus will eventually support classnames
        // for now, just write them with a space between each
        if !self.classes.is_empty() {
            write!(f, "class: ")?;
            write!(f, "{:?}", self.classes.join(" "))?;
            f.write_char(',')?;
            f.write_newline()?;
        }
        // id ??????
        if let Some(id) = &self.id {
            write!(f, "id: {:?},", id)?;
            f.write_newline()?;
        }
        // ????????????
        let ordered = BTreeMap::from_iter(&self.attributes);
        let max_index = ordered.len();
        for (index, (name, value)) in ordered.iter().enumerate() {
            match name.as_str() {
                s if s.contains('-') => {
                    write!(f, "{:?}", name)?
                }
                "for" | "async" | "type" | "as" => {
                    write!(f, "r#{}", name)?;
                }
                _ => {
                    write!(f, "{}", name)?;
                }
            }
            match value {
                Some(v) => write!(f, ": {:?},", v)?,
                None => write!(f, ": \"\",")?,
            }
            // println!("{} < {}", index, max_index);
            if index + 1 != max_index {
                f.write_newline()?
            }
        }
        // ?????????
        // now the children
        let max_index = self.children.len();

        for (index, child) in self.children.iter().enumerate() {
            child.write_rsx(f)?;
            if index + 1 != max_index {
                f.write_newline()?
            }
        }
        // ??????
        f.write_brace_r()?;
        Ok(())
    }

    fn write_rs(&self, _: &mut RsxBuilder) -> Result<()> {
        todo!()
    }
}