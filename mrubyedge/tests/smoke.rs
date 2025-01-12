extern crate mec_mrbc_sys;
extern crate mrubyedge;

macro_rules! mrbc_compile {
    ($src:expr, $dest:expr) => {
        {
            let src = $src;
            let dest = $dest;
            let args = [
                CStr::from_bytes_with_nul(b"mrbc\0").unwrap().as_ptr(),
                CStr::from_bytes_with_nul(b"-o\0").unwrap().as_ptr(),
                CStr::from_bytes_with_nul(dest.as_bytes()).unwrap().as_ptr(),
                CStr::from_bytes_with_nul(src.as_bytes()).unwrap().as_ptr(),
            ];
            unsafe {
                mec_mrbc_sys::mrbc_main(args.len() as i32, args.as_ptr() as *mut *mut i8);
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use mrubyedge::yamrb::{helpers::mrb_funcall, value::RObject};
    use std::{ffi::CStr, rc::Rc};

    #[test]
    fn smoke_test() {
        // Arrange
        let src = "tests/mrblib/add.rb\0";
        let dest = "tests/mrblib/add.mrb\0";
        mrbc_compile!(src, dest);

        let binary = std::fs::read("tests/mrblib/add.mrb").unwrap();
        // Act
        let mut rite = mrubyedge::rite::load(&binary).unwrap();
        let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);
        vm.run().unwrap();

        // Assert
        let args = vec![Rc::new(RObject::integer(1)), Rc::new(RObject::integer(2))];
        let result: i32 = mrb_funcall(&mut vm, None, "add", &args).unwrap().as_ref().try_into().unwrap();
        assert_eq!(result, 3);
    }
}