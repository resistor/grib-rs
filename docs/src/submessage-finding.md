# Finding submessages that match some conditions inside a GRIB message

wgrib2:

```shell
wgrib2 datafile.grib -match ':3 hour fcst:'
```

pygrib:

```python
import pygrib

grib = pygrib.open("datafile.grib")
for submessage in grib.select(forecastTime=3):
    print(submessage)
```

grib-rs:

```rust
use grib::codetables::grib2::*;
use grib::codetables::*;
use grib::datatypes::*;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

fn find_submessages(path: &Path, forecast_time_hours: u32) {
    let f = File::open(&path).unwrap();
    let f = BufReader::new(f);

    let grib2 = grib::from_reader(f).unwrap();

    for (index, submessage) in grib2.iter().enumerate() {
        let ft = submessage.prod_def().forecast_time();
        match ft {
            Some(ForecastTime {
                unit: Name(Table4_4::Hour),
                value: hours,
            }) => {
                if hours == forecast_time_hours {
                    println!("{}: {}", index, hours);
                }
            }
            _ => {}
        }
    }
}

fn main() {
    let path = Path::new("datafile.grib");
    find_submessages(path, 3);
}
```

gribber:

```shell
gribber list datafile.grib | grep '3 Hour'
```

(gribber's API for finding submessages is still in the conceptual stage and is not yet available.)
