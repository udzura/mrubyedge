use std::rc::Rc;

use crate::{yamrb::{helpers::mrb_define_cmethod, value::RObject, vm::VM}, Error};

use super::array::mrb_array_push;

pub(crate) fn initialize_string(vm: &mut VM) {
    let string_class = vm.define_standard_class("String");

    mrb_define_cmethod(vm, string_class.clone(), "unpack", Box::new(mrb_string_unpack));
}

fn bytes_of<const N: usize>(value: &[u8], cursor: usize) -> Result<[u8; N], Error> {
    if value.len() < cursor + N {
        return Err(Error::RuntimeError("Not enough bytes".to_string()));
    }
    value[cursor..cursor + N].try_into().map_err(|_| Error::RuntimeError(format!("Bit size mismatch: {}", N)))
}

fn mrb_string_unpack(vm: &mut VM, args: &[Rc<RObject>]) -> Result<Rc<RObject>, Error> {
    let this = vm.getself()?;
    let value: Vec<u8> = this.as_ref().try_into()?;
    let format: Vec<u8> = args[0].as_ref().try_into()?;
    let mut cursor: usize = 0;
    let result = Rc::new(RObject::array(Vec::new()));

    for c in format.iter() {
        // We just support Ruby#pack's format of:
        //   - Q: 64-bit unsigned (unsigned long long)
        //   - q: 64-bit signed (signed long long)
        //   - L: 32-bit unsigned (unsigned long)
        //   - l: 32-bit signed (signed long)
        //   - I: 32-bit unsigned (unsigned int)
        //   - i: 32-bit signed (signed int)
        //   - S: 16-bit unsigned (unsigned short)
        //   - s: 16-bit signed (signed short)
        //   - C: 8-bit unsigned (unsigned char)
        //   - c: 8-bit signed (signed char)
        let value = match c {
            b'Q' => {
                let value = u64::from_le_bytes(bytes_of::<8>(&value, cursor)?);
                cursor += 8;
                value as i64
            }
            b'q' => {
                let value = i64::from_le_bytes(bytes_of::<8>(&value, cursor)?);
                cursor += 8;
                value as i64
            }
            b'L' | b'I' => {
                let value = u32::from_le_bytes(bytes_of::<4>(&value, cursor)?);
                cursor += 4;
                value as i64
            }
            b'l' | b'i' => {
                let value = i32::from_le_bytes(bytes_of::<4>(&value, cursor)?);
                cursor += 4;
                value as i64
            }
            b'S' => {
                let value = u16::from_le_bytes(bytes_of::<2>(&value, cursor)?);
                cursor += 2;
                value as i64
            }
            b's' => {
                let value = i16::from_le_bytes(bytes_of::<2>(&value, cursor)?);
                cursor += 2;
                value as i64
            }
            b'C' => {
                let value = i8::from_le_bytes(bytes_of::<1>(&value, cursor)?);
                cursor += 1;
                value as i64
            }
            b'c' => {
                let value = u8::from_le_bytes(bytes_of::<1>(&value, cursor)?);
                cursor += 1;
                value as i64
            }
            b' ' => {
                // ignore space
                continue;
            }
            _ => {
                return Err(Error::RuntimeError("Unsupported format".to_string()));
            }
        };
        mrb_array_push(result.clone(), &[Rc::new(RObject::integer(value as i64))])?;
    }

    Ok(result)
}

#[test]
fn test_mrb_string_unpack() {
    use crate::yamrb::*;

    let mut vm = VM::empty();
    prelude::prelude(&mut vm);

    let data = Rc::new(RObject::string_from_vec(
        vec![
            0x01,
            0x02, 0x03,
            0x04, 0x05, 0x06, 0x07,
            0x04, 0x04, 0x03, 0x03, 0x02, 0x02, 0x00, 0x00,
        ],
    ));
    let format = Rc::new(RObject::string(
        "c s l q".to_string(),
    ));
    let arg = vec![format];

    let ret = helpers::mrb_funcall(&mut vm, Some(data), "unpack", &arg).expect("unpack failed");
    
    let answers = vec![
        0x01,
        0x02 | 0x03 << 8,
        0x04 | 0x05 << 8 | 0x06 << 16 | 0x07 << 24,
        0x04 | 0x04 << 8 | 0x03 << 16 | 0x03 << 24 | 0x02 << 32 | 0x02 << 40 | 0x00 << 48 | 0x00 << 56,
    ];

    for (i, expected) in answers.iter().enumerate() {
        let args = vec![Rc::new(RObject::integer(i as i64))];
        let value = prelude::array::mrb_array_get_index(ret.clone(), &args).expect("getting index failed");
        let value: i64 = value.as_ref().try_into().expect("value is not integer");
        assert_eq!(value, *expected);
    }
}