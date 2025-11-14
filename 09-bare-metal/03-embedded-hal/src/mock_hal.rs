use embedded_hal::delay::DelayNs;
/// Mock HAL implementations for demonstration purposes
///
/// In a real embedded system, these traits would be implemented by
/// the platform-specific HAL crate (e.g., stm32f4xx-hal, nrf52840-hal)
use embedded_hal::digital::OutputPin;
use embedded_hal::i2c::{ErrorType as I2cErrorType, I2c};
use embedded_hal::spi::{ErrorType as SpiErrorType, SpiDevice};

/// Mock error type for demonstrations
#[derive(Debug, Copy, Clone)]
pub struct MockError;

impl embedded_hal::spi::Error for MockError {
    fn kind(&self) -> embedded_hal::spi::ErrorKind {
        embedded_hal::spi::ErrorKind::Other
    }
}

impl embedded_hal::i2c::Error for MockError {
    fn kind(&self) -> embedded_hal::i2c::ErrorKind {
        embedded_hal::i2c::ErrorKind::Other
    }
}

/// Mock GPIO pin
///
/// In a real HAL, this would control actual hardware pins
pub struct MockGpio {
    state: bool,
}

impl MockGpio {
    pub fn new() -> Self {
        Self { state: false }
    }
}

impl OutputPin for MockGpio {
    fn set_low(&mut self) -> Result<(), Self::Error> {
        self.state = false;
        // In real hardware: write to GPIO register
        Ok(())
    }

    fn set_high(&mut self) -> Result<(), Self::Error> {
        self.state = true;
        // In real hardware: write to GPIO register
        Ok(())
    }
}

impl embedded_hal::digital::ErrorType for MockGpio {
    type Error = core::convert::Infallible;
}

/// Mock SPI device
///
/// In a real HAL, this would communicate via SPI bus
pub struct MockSpi {
    // In real hardware: register addresses, FIFO, etc.
}

impl MockSpi {
    pub fn new() -> Self {
        Self {}
    }
}

impl SpiErrorType for MockSpi {
    type Error = MockError;
}

impl SpiDevice for MockSpi {
    fn transaction(
        &mut self,
        operations: &mut [embedded_hal::spi::Operation<'_, u8>],
    ) -> Result<(), Self::Error> {
        // Simulate SPI transaction
        for op in operations {
            match op {
                embedded_hal::spi::Operation::Read(buffer) => {
                    // Simulate reading data
                    for byte in buffer.iter_mut() {
                        *byte = 0x42; // Mock data
                    }
                }
                embedded_hal::spi::Operation::Write(buffer) => {
                    // Simulate writing data
                    let _ = buffer;
                }
                embedded_hal::spi::Operation::Transfer(read, write) => {
                    // Simulate full-duplex transfer
                    for (r, w) in read.iter_mut().zip(write.iter()) {
                        *r = *w ^ 0xFF; // Mock transformation
                    }
                }
                embedded_hal::spi::Operation::TransferInPlace(buffer) => {
                    // Simulate in-place transfer
                    for byte in buffer.iter_mut() {
                        *byte ^= 0xFF; // Mock transformation
                    }
                }
                embedded_hal::spi::Operation::DelayNs(ns) => {
                    // Simulate delay
                    let _ = ns;
                }
            }
        }
        Ok(())
    }
}

/// Mock I2C device
///
/// In a real HAL, this would communicate via I2C bus
pub struct MockI2c {
    // In real hardware: SCL/SDA control, buffers, etc.
}

impl MockI2c {
    pub fn new() -> Self {
        Self {}
    }
}

impl I2cErrorType for MockI2c {
    type Error = MockError;
}

impl I2c for MockI2c {
    fn read(&mut self, _address: u8, buffer: &mut [u8]) -> Result<(), Self::Error> {
        // Simulate I2C read
        for byte in buffer.iter_mut() {
            *byte = 0x42; // Mock data
        }
        Ok(())
    }

    fn write(&mut self, _address: u8, _bytes: &[u8]) -> Result<(), Self::Error> {
        // Simulate I2C write
        Ok(())
    }

    fn write_read(
        &mut self,
        address: u8,
        bytes: &[u8],
        buffer: &mut [u8],
    ) -> Result<(), Self::Error> {
        // Simulate I2C write-read transaction
        self.write(address, bytes)?;
        self.read(address, buffer)?;
        Ok(())
    }

    fn transaction(
        &mut self,
        _address: u8,
        operations: &mut [embedded_hal::i2c::Operation<'_>],
    ) -> Result<(), Self::Error> {
        // Simulate I2C transaction
        for op in operations {
            match op {
                embedded_hal::i2c::Operation::Read(buffer) => {
                    for byte in buffer.iter_mut() {
                        *byte = 0x42;
                    }
                }
                embedded_hal::i2c::Operation::Write(buffer) => {
                    let _ = buffer;
                }
            }
        }
        Ok(())
    }
}

/// Mock delay provider
///
/// In a real HAL, this would use hardware timers
pub struct MockDelay {
    // In real hardware: timer configuration
}

impl MockDelay {
    pub fn new() -> Self {
        Self {}
    }
}

impl DelayNs for MockDelay {
    fn delay_ns(&mut self, ns: u32) {
        // Simulate delay (in real hardware: busy-wait or use timer)
        // For bare metal, this might be a busy loop based on CPU cycles
        let cycles = ns / 10; // Assume 100MHz clock
        for _ in 0..cycles {
            core::hint::spin_loop();
        }
    }

    fn delay_us(&mut self, us: u32) {
        self.delay_ns(us.saturating_mul(1000));
    }

    fn delay_ms(&mut self, ms: u32) {
        self.delay_us(ms.saturating_mul(1000));
    }
}
