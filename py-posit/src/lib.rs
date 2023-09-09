use posit::cast;
use posit::*;
use std::ffi::CString;
use std::os::raw::c_char;
use typenum::{Cmp, Less, Unsigned, U0, U1, U16, U17, U2, U3, U32, U4, U5, U6, U7, U8, U9};
use typenum::{
    U10, U11, U12, U13, U14, U15, U18, U19, U20, U21, U22, U23, U24, U25, U26, U27, U28, U29, U30,
    U31,
};

const VERSION: &str = env!("CARGO_PKG_VERSION");

type rust__f64 = f64;

#[no_mangle]
pub extern "C" fn from_bits(bits: u64, n: u32, es: u32) -> Result<Box<rust__f64>, ()> {
    let result = match (n, es) {
        (8, 0) => cast::f64(P8E0::new(bits as u8)),
        (8, 1) => cast::f64(P8E1::new(bits as u8)),
        (8, 2) => cast::f64(P8E2::new(bits as u8)),
        (16, 0) => cast::f64(P16E0::new(bits as u16)),
        (16, 1) => cast::f64(P16E1::new(bits as u16)),
        (16, 2) => cast::f64(P16E2::new(bits as u16)),
        _ => {
            println!(
                "*** from_bits::<{}, {}> has no bindings attached. ***",
                n, es
            );
            return Ok(Box::new(f64::NAN));
        }
    };
    Ok(Box::new(result))
}


#[no_mangle]
pub extern "C" fn from_double(x: f64, n: u32, es: u32) -> Result<Box<rust__f64>, ()> {
    let result = match (n, es) {
        (8, 0) => posit::P8E0(x).map(cast::f64),
        (8, 1) => posit::P8E1(x).map(cast::f64),
        (8, 2) => posit::P8E2(x).map(cast::f64),
        (16, 0) => posit::P16E0(x).map(cast::f64),
        (16, 1) => posit::P16E1(x).map(cast::f64),
        (16, 2) => posit::P16E2(x).map(cast::f64),
        _ => {
            println!(
                "*** from_double::<{}, {}> has no bindings attached. ***",
                n, es
            );
            // return Err(());
            return Ok(Box::new(f64::NAN));
        }
    };
    Ok(result.map(Box::new).unwrap_or_else(|Error| Box::new(f64::NAN)))
}

#[no_mangle]
pub extern "C" fn get_version() -> *const c_char {
    let c_str = CString::new(VERSION).expect("CString creation failed");
    c_str.into_raw()
}
