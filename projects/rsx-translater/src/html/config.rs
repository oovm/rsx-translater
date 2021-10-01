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

impl Default for RsxBuilderConfig {
    fn default() -> Self {
        Self {
            component_name: "Component".to_string(),
            indent_size: 4,
            indent_pre: 0,
            is_renderer: false,
            is_component: false
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