use super::*;


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
        self.write_char('\n')?;
        self.write_indent()?;
        Ok(())
    }
    #[inline]
    pub(crate) fn write_brace_l(&mut self) -> Result<()> {
        self.write_char('{')?;
        self.indent();
        self.write_newline()?;
        Ok(())
    }
    #[inline]
    pub(crate) fn write_brace_r(&mut self) -> Result<()> {
        self.dedent();
        self.write_newline()?;
        self.write_char('}')?;
        Ok(())
    }
    #[inline]
    pub(crate) fn write_render_l(&mut self) -> Result<()> {
        // 一个组件必须有 render
        if !(self.config.is_component || self.config.is_renderer) {
            return Ok(());
        }
        self.write_str("cx.render(rsx! ")?;
        self.write_brace_l()
    }
    #[inline]
    pub(crate) fn write_render_r(&mut self) -> Result<()> {
        if !(self.config.is_component || self.config.is_renderer) {
            return Ok(());
        }
        self.dedent();
        self.write_newline()?;
        self.write_str("})")?;
        Ok(())
    }
    #[inline]
    pub(crate) fn write_component_l(&mut self) -> Result<()> {
        // 一个组件必须有 render
        if !(self.config.is_component) {
            return Ok(());
        }
        write!(self.buffer, "pub fn {}(cx: Scope) -> Element ", self.config.component_name.as_str())?;
        self.write_brace_l()
    }
    #[inline]
    pub(crate) fn write_component_r(&mut self) -> Result<()> {
        if !(self.config.is_component) {
            return Ok(());
        }
        self.write_brace_r()
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

