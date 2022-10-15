[![CI](https://github.com/chvllad/max31855-rs/workflows/CI/badge.svg)](https://github.com/chvllad/max31855-rs/actions)

# max31855-rs

MAX31855 driver with linearization and async embedded_hal support
## Example
```rust
use max31855::MAX31855;
let mut device = MAX31855::new(spi);
let data = device.read_data()?;
assert_eq!(data.raw(), 0xFB701430);
assert_eq!(data.get_linear_temp()?, -83.7001f32);
```
## Features
* `linearization` - enables linearization for K-type thermocouples as described in <https://learn.adafruit.com/calibrating-sensors/maxim-31855-linearization>
* `hal-async` - enables async spi reading using `embedded-hal-async` interfaces (currently requires nightly compiler)
* `hal-block` - enables blocking spi reading using `embedded-hal` interfaces

License: MIT
