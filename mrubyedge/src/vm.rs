use core::ffi::c_void;

pub struct VM<'a> {
    pub vm_id: u32,
    pub top_irep: Box<VMIrep>,
    pub cur_irep: Box<VMIrep>,
    pub insns: &'a [u8],
    pub pc: usize,
    pub target_class: Box<RClass>,
    pub callinfo_vec: Box<CallInfo>,
    pub exception: Box<RObject>,
    pub regs: [RObject; 256],
}

pub struct VMIrep {
    // https://github.com/mrubyc/mrubyc/blob/5fab2b85dce8fc0780293235df6c0daa5fd57dce/src/vm.h#L41-L62
}

pub enum RObject {
    Class(RClass),
    RInstance(*mut c_void),
    RString(String),
    // ...
}

pub struct RClass {
    pub sym_id: u32,
    pub num_builtin_method: usize,
    pub super_klass: Box<RClass>,
    pub methods: Vec<RMethod>,
}

pub struct RMethod {
    pub sym_id: u32,
    pub body: Method,
}

pub enum Method {
    RubyMethod(Box<VMIrep>),
    CMethod(fn() -> ()),
}

pub struct CallInfo {
    // https://github.com/mrubyc/mrubyc/blob/5fab2b85dce8fc0780293235df6c0daa5fd57dce/src/vm.h#L111-L126
}
