use posit::*;
use typenum::{Cmp, Less, Unsigned, U0, U1, U16, U17, U2, U3, U32, U4, U5, U6, U7, U8, U9};
use typenum::{
    U10, U11, U12, U13, U14, U15, U18, U19, U20, U21, U22, U23, U24, U25, U26, U27, U28, U29, U30,
    U31,
};

use posit::cast;

pub trait PositTrait {}
impl PositTrait for P8E0 {}

const VERSION: &str = env!("CARGO_PKG_VERSION");

type rust__f64 = f64;

#[no_mangle]
pub extern "C" fn from_bits(bits: u64, n: u32, es: u32) -> Result<Box<rust__f64>, ()> {
    match (n, es) {
        (8, 0) => Ok(Box::new(cast::f64(P8E0::new(bits as u8)))),
        (8, 1) => Ok(Box::new(cast::f64(P8E1::new(bits as u8)))),
        (8, 2) => Ok(Box::new(cast::f64(P8E2::new(bits as u8)))),

        (16, 0) => Ok(Box::new(cast::f64(P16E0::new(bits as u16)))),
        (16, 1) => Ok(Box::new(cast::f64(P16E1::new(bits as u16)))),
        (16, 2) => Ok(Box::new(cast::f64(P16E2::new(bits as u16)))),

        _ => {
            println!(
                "*** from_bits::<{}, {}> has no bindings attached. ***",
                n, es
            );
            Ok(Box::new(f64::NAN))
        }
    }
}

use std::os::raw::c_char;
use std::ffi::CString;

#[no_mangle]
pub extern "C" fn get_str() -> *const c_char {
    let rust_str = "Hello from Rust!";
    let c_string = CString::new(rust_str).expect("CString::new failed");
    c_string.into_raw()
}