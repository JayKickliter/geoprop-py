mod coverage;
mod error;
mod path;
mod point;
mod profile;
mod tiles;

use point::Point;
use profile::Profile;
use pyo3::{pymodule, types::PyModule, wrap_pyfunction, PyResult, Python};
use tiles::Tiles;

/// A Python module implemented in Rust.
#[pymodule]
#[pyo3(name = "geoprop")]
fn python(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Point>()?;
    m.add_class::<Profile>()?;
    m.add_class::<Tiles>()?;
    m.add_function(wrap_pyfunction!(coverage::estimate, m)?)?;
    m.add_function(wrap_pyfunction!(path::p2p, m)?)?;
    m.add_function(wrap_pyfunction!(path::path, m)?)?;
    Ok(())
}
