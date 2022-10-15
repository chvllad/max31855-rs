use embedded_hal_async::spi::{SpiBusRead, SpiDevice};
use error_stack::{IntoReport, Result};

impl<SPI> super::MAX31855<SPI>
where
    SPI: SpiDevice,
    SPI::Bus: SpiBusRead,
{
    /// Initializes MAX31855 struct with SPI device
    pub fn new(spi: SPI) -> Self {
        Self(spi)
    }

    /// Reads data from device
    pub async fn read_data(&mut self) -> Result<crate::MAX31855Data, super::MAX31855Error> {
        let mut buf = [0; 4];
        self.0
            .read(&mut buf)
            .await
            .map_err(|_| super::MAX31855Error)
            .into_report()?;
        Ok(crate::MAX31855Data::new(u32::from_be_bytes(buf)))
    }
}
