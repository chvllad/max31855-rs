use embedded_hal::spi::{Error, ErrorKind, SpiBusRead, SpiDevice};

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
    pub fn read_data(&mut self) -> Result<crate::MAX31855Data, MAX31855Error> {
        let mut buf = [0; 4];
        self.0
            .read(&mut buf)
            .map_err(|e| super::MAX31855Snafu { error: e.kind() }.build())?;
        Ok(crate::MAX31855Data::new(u32::from_be_bytes(buf)))
    }
}

#[cfg(test)]
mod tests {
    use more_asserts::assert_lt;

    use crate::MAX31855;

    #[test]
    fn test_spi_reads() {
        let mut spi_base = ehm::spi::Mock::new(&[
            ehm::spi::Transaction::transaction_start(),
            ehm::spi::Transaction::read_vec(vec![0xFB, 0x70, 0x14, 0x30]),
            ehm::spi::Transaction::transaction_end(),
        ]);
        let spi = spi_base.clone();
        let mut device = MAX31855::new(spi);
        let data = device.read_data().unwrap();
        assert_eq!(data.raw(), 0xFB701430);
        assert_lt!((data.get_linear_temp().unwrap() - -83.7f32).abs(), 0.01f32);
        spi_base.done();
    }
}
