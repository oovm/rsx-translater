use std::collections::BTreeMap;
use std::iter::FromIterator;
use super::*;
use convert_case::{Case, Casing};

impl RsxBuilder {
    #[inline]
    pub(crate) fn indent(&mut self) {
        self.indent_now += self.config.indent_size
    }
    #[inline]
    pub(crate) fn dedent(&mut self) {
        self.indent_now -= self.config.indent_size
    }
    #[inline]
    pub(crate) fn write_indent(&mut self) -> Result<()> {
        write!(self.buffer, "{}", &" ".repeat(self.indent_now))?;
        Ok(())
    }
    #[inline]
    pub(crate) fn write_newline(&mut self) -> Result<()> {
        write!(self.buffer, "\n")?;
        self.write_indent()?;
        Ok(())
    }
    #[inline]
    pub(crate) fn write_brace_l(&mut self) -> Result<()> {
        write!(self.buffer, "{{\n")?;
        self.indent();
        self.write_newline()?;
        Ok(())
    }
    #[inline]
    pub(crate) fn write_brace_r(&mut self) -> Result<()> {
        self.dedent();
        self.write_newline()?;
        write!(self.buffer, "}}")?;
        Ok(())
    }
}


impl Write for RsxBuilder {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        self.buffer.write_str(s)
    }

    fn write_char(&mut self, c: char) -> std::fmt::Result {
        self.buffer.write_char(c)
    }

    fn write_fmt(self: &mut Self, args: Arguments<'_>) -> std::fmt::Result {
        self.buffer.write_fmt(args)
    }
}


impl AsRsx for Element {
    fn write_rsx(&self, f: &mut RsxBuilder) -> Result<()> {
        // 命名头
        f.write_indent()?;
        f.write_str(&self.name)?;
        f.write_char(' ')?;
        f.write_brace_l()?;
        // html 类名
        // todo: dioxus will eventually support classnames
        // for now, just write them with a space between each
        write!(f, "class: ");
        write!(f, "{:?}", self.classes.join(" "));
        f.write_newline()?;
        // id 属性
        if let Some(id) = &self.id {
            writeln!(f, "id: \"{}\",", id)?;
        }
        f.write_newline()?;
        // 其他属性
        let ordered = BTreeMap::from_iter(&self.attributes);

        for (name, value) in &ordered {
            if name.chars().any(|ch| ch.is_ascii_uppercase() || ch == '-') {
                let new_name = name.to_case(Case::Snake);
                match value {
                    Some(val) => writeln!(f, "{}: \"{}\",", new_name, val)?,
                    None => writeln!(f, "{}: \"\",", new_name)?,
                }
            }
            else {
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
        // 子节点
        // now the children
        for child in &self.children {
            child.write_rsx(f)?
        }
        // 结束
        f.write_brace_r()?;
        Ok(())
    }

    fn write_rs(&self, f: &mut RsxBuilder) -> Result<()> {
        todo!()
    }
}