use colored::Colorize;
use posit::cast;
use posit::*;
use pyo3::prelude::*;

const VERSION: &str = env!("CARGO_PKG_VERSION");

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
                return f64::NAN;
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
                return f64::NAN;
            }
        }
    };
}

#[pyfunction]
fn from_bits(bits: u64, n: u32, es: u32) -> f64 {
    match_bits!(n, es, bits => {
        8, 0 => P8E0,
        8, 1 => P8E1,
        8, 2 => P8E2,
        8, 5 => P8E5,
        16, 0 => P16E0,
        16, 1 => P16E1,
        16, 2 => P16E2
    })
}

#[pyfunction]
fn from_double(x: f64, n: u32, es: u32) -> f64 {
    let result = match_double!(x, n, es => {
        8, 0 => posit::P8E0,
        8, 1 => posit::P8E1,
        8, 2 => posit::P8E2,
        8, 5 => posit::P8E5,
        16, 0 => posit::P16E0,
        16, 1 => posit::P16E1,
        16, 2 => posit::P16E2
    });

    result.unwrap_or(f64::NAN)
}

#[pyfunction]
fn __get_version() -> String {
    VERSION.to_string()
}

#[pymodule]
fn jposit(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(from_bits, py)?)?;
    m.add_function(wrap_pyfunction!(from_double, py)?)?;
    m.add_function(wrap_pyfunction!(__get_version, py)?)?;
    Ok(())
}
