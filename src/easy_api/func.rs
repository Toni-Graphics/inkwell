use crate::values::FunctionValue;

pub struct EasyFunction<'ctx> {
    func: FunctionValue<'ctx>   
}

impl<'ctx> EasyFunction<'ctx> {
    pub fn new(func: FunctionValue<'ctx>) -> Self {
        Self {
            func: func
        }
    } 
}