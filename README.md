# units

Convert to and from and do arithmetic on various units.

## examples

### Conversion
```rs
use units::{
    units::{Meter, Mph, Second},
    Measurement,
};

// Craft a measurement
let m = Meter::new(42.0);

// Do some math
let speed = m / Second::new(1.3616521739130436);

// Check the result
assert_eq![speed.to_unit(Mph), Some(69.0)];
```
### Inversion
```rs
use units::{units::{Hertz, Second}, Measurement, Units};

let freq = Hertz::new(60.0);

// Number of cycles / freq
let time = Measurement::from_unit(Units::Scalar, 60.0) / freq;

assert_eq![time, Some(1.0)];
```
