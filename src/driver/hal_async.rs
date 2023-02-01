use embedded_hal_async::spi::{Error, ErrorKind, SpiBusRead, SpiDevice};

pub type MAX31855Error = super::error::MAX31855Error<ErrorKind>;

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
    pub async fn read_data(&mut self) -> Result<crate::MAX31855Data, MAX31855Error> {
        let mut buf = [0; 4];
        self.0
            .read(&mut buf)
            .await
            .map_err(|e| super::MAX31855Snafu { error: e.kind() }.build())?;
        Ok(crate::MAX31855Data::new(u32::from_be_bytes(buf)))
    }
}
