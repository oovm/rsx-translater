mod writer;
mod svg;
mod config;

use std::{
    fmt::{Display, Formatter, Write},
};
use std::fmt::Arguments;

use html_parser::{Dom, Element, Node};

use crate::{AsRsx, Result};

pub struct RsxBuilder {
    config: RsxBuilderConfig,
    buffer: String,
    indent_now: usize,
    svg_cache: Vec<Element>,
}

pub struct RsxBuilderConfig {
    component_name: String,
    indent_size: usize,
}

impl Default for RsxBuilderConfig {
    fn default() -> Self {
        Self {
            component_name: "component".to_string(),
            indent_size: 4,
        }
    }
}

impl Default for RsxBuilder {
    fn default() -> Self {
        Self {
            config: Default::default(),
            buffer: String::new(),
            indent_now: 0,
            svg_cache: vec![],
        }
    }
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
        self.svg_cache.clear()
    }
}


impl AsRsx for Dom {
    fn write_rsx(&self, f: &mut RsxBuilder) -> Result<()> {
        for i in &self.children {
            i.write_rsx(f)?
        }
        f.write_svg()?;
        Ok(())
    }

    fn write_rs(&self, f: &mut RsxBuilder) -> Result<()> {
        todo!()
    }
}

impl AsRsx for Node {
    fn write_rsx(&self, f: &mut RsxBuilder) -> Result<()> {
        match self {
            Self::Text(t) => writeln!(f, "\"{}\"", t)?,
            Self::Comment(c) => writeln!(f, "/* {} */", c)?,
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

