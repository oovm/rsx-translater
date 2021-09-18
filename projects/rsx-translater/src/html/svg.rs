use super::*;


impl RsxBuilder {
    pub fn collect_svg(&mut self, e: &Element) -> Result<()> {
        self.svg_cache.push(e.to_owned());
        // todo: attach the right icon ID
        writeln!(self.buffer, "icons::icon_{} {{}}", self.svg_cache.len())?;
        return Ok(())
    }
    pub fn write_svg(&self) -> Result<()> {
        Ok(())
    }
}
