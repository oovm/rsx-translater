use super::*;


impl RsxBuilder {
    #[inline]
    pub fn set_config(mut self, config: RsxBuilderConfig)-> Self {
        self.config = config;
        self
    }
    #[inline]
    pub fn set_name(mut self, name: String)->Self {
        self.config.component_name = name;
        self
    }
    #[inline]
    pub fn preset_indent(mut self, indent: usize) -> Self  {
        self.config.indent_pre = indent;
        self
    }
    #[inline]
    pub fn config_component(mut self, config: bool) -> Self {
        self.config.is_component = config;
        self
    }
    #[inline]
    pub fn config_renderer(mut self, config: bool) -> Self  {
        self.config.is_renderer = config;
        self
    }
}

impl Default for RsxBuilderConfig {
    fn default() -> Self {
        Self {
            component_name: "Component".to_string(),
            indent_size: 4,
            indent_pre: 0,
            is_renderer: false,
            is_component: false,
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

impl Debug for RsxBuilder {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.config, f)
    }
}