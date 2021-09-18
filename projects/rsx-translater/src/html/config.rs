use super::*;


impl RsxBuilder {
    #[inline]
    pub fn set_name(&mut self, name: String) {
        self.config.component_name = name
    }
}