use crate::{error::GeopropError, point::Point, tiles::Tiles};
use h3o::Resolution;
use itm::{Climate, ModeVariability, Polarization};
use pyo3::pyfunction;
use rayon::iter::{IntoParallelIterator, ParallelExtend, ParallelIterator};
use terrain::{geo::Coord, Profile};

const SQRT_3: f32 = 1.732_050_8_f32;

/// Given a transmitter at `center`, estimate its coverage taking
/// terrain into account.
///
/// # Example
///
/** ```python
from geoprop import Tiles, Point, estimate

tiles = Tiles("nasadem/3-arcsecond/srtm/")
center = Point(36.159600, -112.306877, 1000)
rx_alt_m = 1
h3_res = 10
freq_hz = 900e6
radius_km = 12

coverage = estimate(tiles, center, h3_res, freq_hz, radius_km, rx_alt_m, rx_threshold_db = None)

print("h3_id,elev,atten")
for (cell, elev, atten) in coverage:
  print("%x,%d,%f" % (cell, elev, -atten))
``` */
#[pyfunction]
pub fn estimate(
    tiles: &Tiles,
    center: Point,
    res: u8,
    freq_hz: f32,
    max_radius_km: f32,
    rx_alt_m: f32,
    rx_threshold_db: Option<f32>,
) -> Result<Vec<(u64, f32, f64)>, GeopropError> {
    let res = Resolution::try_from(res).unwrap();
    let ll = h3o::LatLng::new(center.lat as f64, center.lon as f64).unwrap();
    let cell = ll.to_cell(res);
    let mut hexes = Vec::new();
    let edge_length = res.edge_length_km() as f32;
    let start_coord = Coord {
        x: center.lon,
        y: center.lat,
    };

    for ring in (0..).take_while(|ring| *ring as f32 <= max_radius_km / (edge_length * SQRT_3)) {
        let cells = cell.grid_ring_fast(ring).collect::<Option<Vec<_>>>();

        hexes.par_extend(cells.into_par_iter().flatten().map(|cell| {
            let latlng = h3o::LatLng::from(cell);

            let profile_res = Profile::<f32>::builder()
                .start(start_coord)
                .start_alt(center.alt)
                .max_step(90.0)
                .end(Coord {
                    x: latlng.lng() as f32,
                    y: latlng.lat() as f32,
                })
                .end_alt(rx_alt_m)
                .build(tiles);

            let climate = Climate::ContinentalTemperate;
            let n0 = 301.0;
            let pol = Polarization::Vertical;
            let epsilon = 15.0;
            let sigma = 0.005;
            let mdvar = ModeVariability::Mobile;
            let time = 95.0;
            let location = 95.0;
            let situation = 95.0;
            match profile_res {
                Err(e) => Err(GeopropError::from(e)),
                Ok(profile) if profile.distances_m.len() <= 1 => Ok((
                    u64::from(cell),
                    *profile.terrain_elev_m.last().unwrap(),
                    0.0,
                )),
                Ok(profile) => {
                    let step_size_m = profile.distances_m[1];
                    let terrain = profile.terrain_elev_m;
                    itm::p2p(
                        center.alt.into(),
                        rx_alt_m.into(),
                        step_size_m.into(),
                        &terrain,
                        climate,
                        n0,
                        freq_hz.into(),
                        pol,
                        epsilon,
                        sigma,
                        mdvar,
                        time,
                        location,
                        situation,
                    )
                    .map_err(GeopropError::from)
                    .map(|loss| (u64::from(cell), *terrain.last().unwrap(), loss))
                }
            }
        }));
    }

    let pairs = hexes
        .into_iter()
        .flatten()
        .filter(|(_cell, _elev, atten)| {
            rx_threshold_db
                .map(|rxt| *atten < rxt as f64)
                .unwrap_or(true)
        })
        .collect::<Vec<_>>();

    Ok(pairs)
}
