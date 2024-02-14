# geoprop python bindings

This crate provides a Python API to select geoprop functionality.

## Example

### Point to Point

```python
import geoprop

tiles = geoprop.Tiles("/path/to/nasadem/srtm/hgt-files")

start = geoprop.Point(36.00413897612008, -112.2797569088778, 3)
end = geoprop.Point(36.20334730019485, -112.1230717397408, 3)

profile = tiles.profile(start, end)
great_circle = profile.great_circle()
distances = profile.distances()
elevation = profile.elevation()
los = profile.los()

freq = 900e6
p2p_atten = geoprop.p2p(profile, freq)
path_atten = geoprop.path(profile, freq)
```


### Coverage

![Screenshot 2024-02-14 at 10 45 53 AM](https://github.com/JayKickliter/geoprop-py/assets/2551201/0dd53033-eaf7-4560-bb5c-d05cbc3be660)

```python
from geoprop import Tiles, Point, Coverage

tiles = Tiles("nasadem/3-arcsecond/srtm/")
center = Point(36.159600, -112.306877, 1000)
rx_alt_m = 1
h3_res = 10
freq_hz = 900e6
radius_km = 12

coverage = Coverage(tiles)
estimated_coverage = coverage.estimate(center, h3_res, freq_hz, radius_km, rx_alt_m, rx_threshold_db = None)

print("h3_id,elev,atten")
for (cell, elev, atten) in estimated_coverage:
    print("%x,%d,%f" % (cell, elev, -atten))
```

