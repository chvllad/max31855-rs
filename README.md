[![CI](https://github.com/chvllad/max31855-rs/workflows/CI/badge.svg)](https://github.com/chvllad/max31855-rs/actions)

# max31855-rs

MAX31855 driver with linearization and async embedded_hal support
## Example
```rust
use max31855_rs::MAX31855;
let mut device = MAX31855::new(spi);

// Sync
let data = device.read_data()?;
// Async
let data = device.read_data().await?;

let data_raw = data.raw();
let temp = data.get_linear_temp()?;
```
## Features
* `linearization` - enables linearization for K-type thermocouples as described in <https://learn.adafruit.com/calibrating-sensors/maxim-31855-linearization>. Requires either `libm` or `micromath` feature to work.
* `hal-async` - enables async spi reading using `embedded-hal-async` interfaces (currently requires nightly compiler)
* `hal-block` - enables blocking spi reading using `embedded-hal` interfaces

License: MIT OR Apache-2.0
