use crate::RsxBuilder;
use crate::Result;

pub trait AsRsx {
    fn write_rsx(&self, ctx: &mut RsxBuilder) -> Result<String>;
    fn write_rs(&self, ctx: &mut RsxBuilder) -> Result<String>;
}