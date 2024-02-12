use crate::{error::GeopropError, point::Point, profile::Profile};
use pyo3::{pyclass, pymethods};
use std::path::PathBuf;
use terrain::{TileMode, Tiles as TerrainTiles};

#[pyclass]
pub(crate) struct Tiles(TerrainTiles);

impl std::ops::Deref for Tiles {
    type Target = TerrainTiles;
    fn deref(&self) -> &TerrainTiles {
        &self.0
    }
}

#[pymethods]
impl Tiles {
    #[new]
    pub(crate) fn new(tile_dir: PathBuf) -> Result<Tiles, GeopropError> {
        Ok(Tiles(TerrainTiles::new(tile_dir, TileMode::MemMap)?))
    }

    pub(crate) fn profile(
        &self,
        start: Point,
        end: Point,
        earth_curve: Option<bool>,
        earth_radius_m: Option<f32>,
    ) -> Result<crate::profile::Profile, GeopropError> {
        Profile::new(self, start, end, earth_curve, earth_radius_m)
    }
}
