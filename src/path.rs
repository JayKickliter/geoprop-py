use crate::{error::GeopropError, profile::Profile};
use itm::{Climate, ModeVariability, Polarization};
use pyo3::pyfunction;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

/// Retuns signal loss between two points.
#[pyfunction]
#[allow(clippy::too_many_arguments)]
pub(crate) fn p2p(profile: &Profile, freq: f32) -> Result<f64, GeopropError> {
    let climate = Climate::Desert;
    let n0 = 301.;
    let f_hz = freq;
    let pol = Polarization::Vertical;
    let epsilon = 15.;
    let sigma = 0.005;
    let mdvar = ModeVariability::Accidental;
    let time = 50.0;
    let location = 50.0;
    let situation = 50.0;
    let step_size_m = profile.distances_m[1];
    let terrain = &profile.terrain_elev_m;
    let attenuation_db = itm::p2p(
        profile.start_alt.into(),
        profile.end_alt.into(),
        step_size_m.into(),
        terrain,
        climate,
        n0,
        f_hz.into(),
        pol,
        epsilon,
        sigma,
        mdvar,
        time,
        location,
        situation,
    )?;
    Ok(attenuation_db)
}

/// Returns signal losses between two points.
#[pyfunction]
#[allow(clippy::too_many_arguments)]
pub(crate) fn path(profile: &Profile, freq: f32) -> Result<Vec<f64>, GeopropError> {
    let climate = Climate::Desert;
    let n0 = 301.;
    let f_hz = freq;
    let pol = Polarization::Vertical;
    let epsilon = 15.;
    let sigma = 0.005;
    let mdvar = ModeVariability::Accidental;
    let time = 50.0;
    let location = 50.0;
    let situation = 50.0;
    let step_size_m = profile.distances_m[1];
    let terrain = &profile.terrain_elev_m;
    let loss_path_db = (1..terrain.len())
        .into_par_iter()
        .map(|end_idx| {
            let terrain = &terrain[..=end_idx];
            itm::p2p(
                profile.start_alt.into(),
                profile.end_alt.into(),
                step_size_m.into(),
                terrain,
                climate,
                n0,
                f_hz.into(),
                pol,
                epsilon,
                sigma,
                mdvar,
                time,
                location,
                situation,
            )
        })
        .collect::<Result<Vec<f64>, _>>()?;
    Ok(loss_path_db)
}
