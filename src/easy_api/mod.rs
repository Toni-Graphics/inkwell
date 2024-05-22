//! Easy api

use crate::{context::Context, module::Linkage, types::{BasicMetadataTypeEnum, BasicTypeEnum, FunctionType}};

pub mod context;
pub mod func;

#[allow(non_camel_case_types)]
pub enum EasyType {
    u8, i8,
    u16, i16,
    u32, i32,
    u64, i64,

    f16, 
    f32,
    f64,
    f128,
}

impl EasyType {
    pub fn to_llvm<'a>(&'a self, context: &'a Context) -> BasicTypeEnum {
        match self {
            EasyType::u8  | EasyType::i8 => BasicTypeEnum::IntType( context.i8_type() ),
            EasyType::u16 | EasyType::i16 => BasicTypeEnum::IntType( context.i16_type() ),
            EasyType::u32 | EasyType::i32 => BasicTypeEnum::IntType( context.i32_type() ),
            EasyType::u64 | EasyType::i64 => BasicTypeEnum::IntType( context.i64_type() ),
            EasyType::f16   => BasicTypeEnum::FloatType( context.f16_type()),
            EasyType::f32   => BasicTypeEnum::FloatType( context.f32_type()),
            EasyType::f64   => BasicTypeEnum::FloatType( context.f64_type()),
            EasyType::f128  => BasicTypeEnum::FloatType( context.f128_type()),
        }
    }
}

pub struct EasyFnType<'ctx> {
    pub ret: EasyType,
    
    pub args: &'ctx Vec<EasyType>,
}

impl<'ctx> EasyFnType<'ctx> {
    pub fn to_llvm(&self, context: &Context) -> FunctionType {
        let mut llvm_args: Vec<BasicMetadataTypeEnum> = vec![];

        for arg in self.args {
            llvm_args.push( arg.to_llvm(&context).into() );
        }

        let ret = self.ret.to_llvm(&context);

        match ret {
            BasicTypeEnum::ArrayType(x) => x.fn_type(&llvm_args, false),
            BasicTypeEnum::FloatType(x) => x.fn_type(&llvm_args, false),
            BasicTypeEnum::IntType(x) => x.fn_type(&llvm_args, false),
            BasicTypeEnum::PointerType(x) => x.fn_type(&llvm_args, false),
            BasicTypeEnum::StructType(x) => x.fn_type(&llvm_args, false),
            BasicTypeEnum::VectorType(x) => x.fn_type(&llvm_args, false),
        }
    }
}

pub enum EasyLink {
    Export,
    Private,
    DLLImport,
    DLLExport,
}

impl EasyLink {
    pub fn to_llvm(&self) -> Linkage {
        match self {
            EasyLink::Export => Linkage::External,
            EasyLink::Private => Linkage::Private,
            EasyLink::DLLImport => Linkage::DLLImport,
            EasyLink::DLLExport => Linkage::DLLExport,
        }
    }
}