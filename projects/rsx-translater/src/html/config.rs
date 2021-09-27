use super::*;


impl RsxBuilder {
    #[inline]
    pub fn set_name(&mut self, name: String) {
        self.config.component_name = name
    }
    #[inline]
    pub fn preset_indent(&mut self, indent: usize) {
        self.config.indent_pre = indent
    }
}