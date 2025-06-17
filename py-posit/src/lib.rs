use colored::Colorize;
use posit::cast;
use posit::*;
use std::ffi::CString;
use std::os::raw::c_char;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[allow(non_camel_case_types)]
type rust__f64 = f64;

macro_rules! match_bits {
    ($n:expr, $es:expr, $bits:expr => { $($n_lit:literal, $es_lit:literal => $typ:ty),* $(,)? }) => {
        match ($n, $es) {
            $(
                ($n_lit, $es_lit) => cast::f64(<$typ>::new($bits as _)),
            )*
            _ => {
                eprintln!(
                    "{}",
                    format!("*** from_bits::<{}, {}> has no bindings attached. ***", $n, $es).red()
                );
                return Ok(Box::new(f64::NAN));
            }
        }
    };
}

macro_rules! match_double {
    ($x:expr, $n:expr, $es:expr => { $($n_lit:literal, $es_lit:literal => $ctor:expr),* $(,)? }) => {
        match ($n, $es) {
            $(
                ($n_lit, $es_lit) => $ctor($x).map(cast::f64),
            )*
            _ => {
                eprintln!(
                    "{}",
                    format!("*** from_double::<{}, {}> has no bindings attached. ***", $n, $es).red()
                );
                return Ok(Box::new(f64::NAN));
            }
        }
    };
}

#[unsafe(no_mangle)]
pub extern "C" fn from_bits(bits: u64, n: u32, es: u32) -> Result<Box<rust__f64>, ()> {
    let result = match_bits!(n, es, bits => {
        8, 0 => P8E0,
        8, 1 => P8E1,
        8, 2 => P8E2,
        8, 5 => P8E5,
        16, 0 => P16E0,
        16, 1 => P16E1,
        16, 2 => P16E2
    });
    Ok(Box::new(result))
}

#[unsafe(no_mangle)]
pub extern "C" fn from_double(x: f64, n: u32, es: u32) -> Result<Box<rust__f64>, ()> {
    let result = match_double!(x, n, es => {
        8, 0 => posit::P8E0,
        8, 1 => posit::P8E1,
        8, 2 => posit::P8E2,
        8, 5 => posit::P8E5,
        16, 0 => posit::P16E0,
        16, 1 => posit::P16E1,
        16, 2 => posit::P16E2
    });

    Ok(Box::new(result.unwrap_or_else(|_| f64::NAN)))
}

#[unsafe(no_mangle)]
pub extern "C" fn get_version() -> *const c_char {
    let c_str = CString::new(VERSION).expect("CString creation failed");
    c_str.into_raw()
}
