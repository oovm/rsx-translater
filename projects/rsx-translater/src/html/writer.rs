use super::*;

impl RsxBuilder {
    #[inline]
    pub(crate) fn indent(&mut self) {
        self.indent += self.config.indent_size
    }
    #[inline]
    pub(crate) fn dedent(&mut self) {
        self.indent -= self.config.indent_size
    }
    #[inline]
    pub(crate) fn write_indent(&mut self) -> Result<()> {
        write!(self.buffer, "{}", &" ".repeat(4))?;
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
