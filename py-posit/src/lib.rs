use colored::Colorize;
use posit::cast;
use posit::*;
use std::ffi::CString;
use std::os::raw::c_char;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[no_mangle]
pub extern "C" fn from_bits(bits: u64, n: u32, es: u32) -> Result<Box<Option<f64>>, ()> {
    let result = match (n, es) {
        (8, 0) => cast::f64(P8E0::new(bits as u8)),
        (8, 1) => cast::f64(P8E1::new(bits as u8)),
        (8, 2) => cast::f64(P8E2::new(bits as u8)),
        (16, 0) => cast::f64(P16E0::new(bits as u16)),
        (16, 1) => cast::f64(P16E1::new(bits as u16)),
        (16, 2) => cast::f64(P16E2::new(bits as u16)),
        _ => {
            eprintln!(
                "{}",
                format!(
                    "*** from_bits::<{}, {}> has no bindings attached. ***",
                    n, es
                )
                .red()
            );
            return Ok(Box::new(None));
        }
    };
    Ok(Box::new(Some(result)))
}

#[no_mangle]
pub extern "C" fn from_double(x: f64, n: u32, es: u32) -> Result<Box<Option<f64>>, ()> {
    let result = match (n, es) {
        (8, 0) => posit::P8E0(x).map(cast::f64),
        (8, 1) => posit::P8E1(x).map(cast::f64),
        (8, 2) => posit::P8E2(x).map(cast::f64),
        (16, 0) => posit::P16E0(x).map(cast::f64),
        (16, 1) => posit::P16E1(x).map(cast::f64),
        (16, 2) => posit::P16E2(x).map(cast::f64),
        _ => {
            eprintln!(
                "{}",
                format!(
                    "*** from_bits::<{}, {}> has no bindings attached. ***",
                    n, es
                )
                .red()
            );
            // return Err(());
            return Ok(Box::new(None));
        }
    };
    match result {
        Ok(val) => Ok(Box::new(Some(val))),
        Err(_) => Ok(Box::new(None)), // Conversion failed, but not a fatal error
    }

}

#[no_mangle]
pub extern "C" fn get_version() -> *const c_char {
    let c_str = CString::new(VERSION).expect("CString creation failed");
    c_str.into_raw()
}
