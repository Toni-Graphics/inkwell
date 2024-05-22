use crate::{builder::Builder, context::Context, module::Module};
use super::EasyFnType;
use super::EasyLink;
use super::func::EasyFunction;

pub struct EasyContext<'ctx> { 
    context: Context,
    module: Module<'ctx>,
    builder: Builder<'ctx>,
}

impl<'ctx> EasyContext<'ctx> {
    pub fn new(name: &str) -> Self {
        let context = Context::create();
        let module = context.create_module(name);

        Self {
            context: context,
            module: module,
            builder: context.create_builder()
        }
    }

    pub fn create_func(&self, name: &str, fn_type: EasyFnType, link: EasyLink) -> EasyFunction {

        let fn_type = fn_type.to_llvm(&self.context);

        let func = self.module.add_function(
            name, 
            fn_type, 
            Some(link.to_llvm())
        );

        EasyFunction::new(func)
    } 
}