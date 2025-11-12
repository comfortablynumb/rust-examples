/// Portable driver implementations using embedded-hal traits
///
/// These drivers work with ANY platform that implements the embedded-hal traits

pub mod led_controller {
    use embedded_hal::digital::OutputPin;
    use embedded_hal::delay::DelayNs;

    /// Blink an LED using any GPIO pin and delay implementation
    ///
    /// This function is completely portable - it works on:
    /// - STM32, nRF, ESP32, RP2040, RISC-V, etc.
    /// - Any platform with embedded-hal support
    pub fn blink_led<P, D>(pin: &mut P, delay: &mut D)
    where
        P: OutputPin,
        D: DelayNs,
    {
        for _ in 0..5 {
            let _ = pin.set_high();
            delay.delay_ms(500);

            let _ = pin.set_low();
            delay.delay_ms(500);
        }
    }

    /// More complex LED pattern
    pub fn morse_code<P, D>(pin: &mut P, delay: &mut D, message: &str)
    where
        P: OutputPin,
        D: DelayNs,
    {
        const DOT_DURATION: u32 = 200;
        const DASH_DURATION: u32 = 600;
        const SYMBOL_GAP: u32 = 200;

        for ch in message.chars() {
            match ch {
                '.' => {
                    let _ = pin.set_high();
                    delay.delay_ms(DOT_DURATION);
                    let _ = pin.set_low();
                    delay.delay_ms(SYMBOL_GAP);
                }
                '-' => {
                    let _ = pin.set_high();
                    delay.delay_ms(DASH_DURATION);
                    let _ = pin.set_low();
                    delay.delay_ms(SYMBOL_GAP);
                }
                ' ' => {
                    delay.delay_ms(DASH_DURATION);
                }
                _ => {}
            }
        }
    }
}

pub mod spi_sensor {
    use embedded_hal::spi::SpiDevice;

    /// Read data from an SPI sensor
    ///
    /// This driver works with any SPI implementation
    pub fn read_sensor<S>(spi: &mut S) -> Result<[u8; 4], S::Error>
    where
        S: SpiDevice,
    {
        let mut buffer = [0u8; 4];

        // Perform SPI transaction
        spi.transaction(&mut [
            embedded_hal::spi::Operation::Write(&[0x3B]), // Read register command
            embedded_hal::spi::Operation::Read(&mut buffer),
        ])?;

        Ok(buffer)
    }

    /// Write configuration to SPI sensor
    pub fn configure_sensor<S>(spi: &mut S, config: u8) -> Result<(), S::Error>
    where
        S: SpiDevice,
    {
        spi.transaction(&mut [
            embedded_hal::spi::Operation::Write(&[0x1A, config]),
        ])?;

        Ok(())
    }

    /// Advanced: Read multiple registers
    pub fn read_registers<S>(spi: &mut S, start_reg: u8, buffer: &mut [u8])
        -> Result<(), S::Error>
    where
        S: SpiDevice,
    {
        spi.transaction(&mut [
            embedded_hal::spi::Operation::Write(&[start_reg | 0x80]), // Set read bit
            embedded_hal::spi::Operation::Read(buffer),
        ])?;

        Ok(())
    }
}

pub mod i2c_sensor {
    use embedded_hal::i2c::I2c;

    const TEMP_SENSOR_ADDR: u8 = 0x48;
    const TEMP_REG: u8 = 0x00;
    const CONFIG_REG: u8 = 0x01;

    /// Read temperature from I2C sensor
    ///
    /// This driver works with any I2C implementation
    pub fn read_temperature<I>(i2c: &mut I) -> Result<i16, I::Error>
    where
        I: I2c,
    {
        let mut buffer = [0u8; 2];

        // Read temperature register
        i2c.write_read(TEMP_SENSOR_ADDR, &[TEMP_REG], &mut buffer)?;

        // Convert raw bytes to temperature
        // This is a typical 12-bit temperature sensor format
        let raw = i16::from_be_bytes(buffer);
        let temp = raw >> 4; // 12-bit value in upper bits

        Ok(temp)
    }

    /// Configure I2C sensor
    pub fn configure<I>(i2c: &mut I, config: u8) -> Result<(), I::Error>
    where
        I: I2c,
    {
        i2c.write(TEMP_SENSOR_ADDR, &[CONFIG_REG, config])?;
        Ok(())
    }

    /// Read multiple bytes from sensor
    pub fn read_data<I>(i2c: &mut I, reg: u8, buffer: &mut [u8]) -> Result<(), I::Error>
    where
        I: I2c,
    {
        i2c.write_read(TEMP_SENSOR_ADDR, &[reg], buffer)?;
        Ok(())
    }
}

/// Example of a complete driver for a specific sensor
pub mod bme280 {
    use embedded_hal::i2c::I2c;
    use embedded_hal::delay::DelayNs;

    const BME280_ADDR: u8 = 0x76;
    const CHIP_ID_REG: u8 = 0xD0;
    const CTRL_MEAS_REG: u8 = 0xF4;

    /// BME280 sensor driver
    pub struct Bme280<I> {
        i2c: I,
        address: u8,
    }

    impl<I> Bme280<I>
    where
        I: I2c,
    {
        pub fn new(i2c: I) -> Self {
            Self {
                i2c,
                address: BME280_ADDR,
            }
        }

        /// Initialize the sensor
        pub fn init<D>(&mut self, delay: &mut D) -> Result<(), I::Error>
        where
            D: DelayNs,
        {
            // Verify chip ID
            let mut id = [0u8];
            self.i2c.write_read(self.address, &[CHIP_ID_REG], &mut id)?;

            // Configure sensor
            self.i2c.write(self.address, &[CTRL_MEAS_REG, 0x27])?;

            delay.delay_ms(10);

            Ok(())
        }

        /// Read temperature, pressure, and humidity
        pub fn read_all(&mut self) -> Result<(i32, u32, u32), I::Error> {
            let mut data = [0u8; 8];
            self.i2c.write_read(self.address, &[0xF7], &mut data)?;

            // Parse the data (simplified)
            let temp = i32::from_be_bytes([0, data[3], data[4], data[5]]) >> 4;
            let pressure = u32::from_be_bytes([data[0], data[1], data[2], 0]) >> 4;
            let humidity = u32::from_be_bytes([0, 0, data[6], data[7]]);

            Ok((temp, pressure, humidity))
        }
    }
}
