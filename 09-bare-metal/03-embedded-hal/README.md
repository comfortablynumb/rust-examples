# Embedded HAL - Hardware Abstraction Layer

This example demonstrates how to write portable embedded code using the `embedded-hal` traits.

## What is embedded-hal?

`embedded-hal` is a collection of traits that define standard interfaces for common embedded peripherals:

- **GPIO**: Digital input/output pins
- **SPI**: Serial Peripheral Interface
- **I2C**: Inter-Integrated Circuit
- **UART**: Universal Asynchronous Receiver/Transmitter
- **Delay**: Blocking delays
- **PWM**: Pulse Width Modulation
- **ADC**: Analog to Digital Converter

## Why Use embedded-hal?

### Portability

Write your driver once, use it on ANY platform:

```rust
// This works on STM32, nRF52, ESP32, RP2040, etc.!
fn blink_led<P: OutputPin>(pin: &mut P) {
    pin.set_high().ok();
    // delay
    pin.set_low().ok();
}
```

### Ecosystem

Many drivers are already written using embedded-hal:
- **Sensors**: BME280, MPU6050, BNO055, etc.
- **Displays**: SSD1306, ST7789, ILI9341, etc.
- **Storage**: SD cards, EEPROM, Flash, etc.
- **Communication**: Radio modules, GPS, etc.

### Separation of Concerns

- **HAL crate**: Platform-specific implementation (e.g., `stm32f4xx-hal`)
- **Driver crate**: Generic driver using traits (e.g., `bme280`)
- **Application**: Uses both without platform-specific code

## embedded-hal 1.0 Traits

### GPIO - Digital I/O

```rust
use embedded_hal::digital::{OutputPin, InputPin};

fn control_led<P: OutputPin>(led: &mut P) {
    led.set_high().ok();
    led.set_low().ok();
}

fn read_button<P: InputPin>(button: &P) -> bool {
    button.is_high().unwrap_or(false)
}
```

### SPI - Serial Peripheral Interface

```rust
use embedded_hal::spi::SpiDevice;

fn read_sensor<S: SpiDevice>(spi: &mut S) -> Result<[u8; 4], S::Error> {
    let mut buffer = [0u8; 4];

    spi.transaction(&mut [
        Operation::Write(&[0x3B]), // Command
        Operation::Read(&mut buffer),
    ])?;

    Ok(buffer)
}
```

### I2C - Inter-Integrated Circuit

```rust
use embedded_hal::i2c::I2c;

fn read_temp<I: I2c>(i2c: &mut I, addr: u8) -> Result<i16, I::Error> {
    let mut buffer = [0u8; 2];
    i2c.write_read(addr, &[0x00], &mut buffer)?;
    Ok(i16::from_be_bytes(buffer))
}
```

### Delay

```rust
use embedded_hal::delay::DelayNs;

fn wait<D: DelayNs>(delay: &mut D) {
    delay.delay_ms(1000);  // 1 second
    delay.delay_us(500);   // 500 microseconds
    delay.delay_ns(100);   // 100 nanoseconds
}
```

## Writing Portable Drivers

### Basic Pattern

```rust
pub struct MyDriver<I2C> {
    i2c: I2C,
    address: u8,
}

impl<I2C> MyDriver<I2C>
where
    I2C: embedded_hal::i2c::I2c,
{
    pub fn new(i2c: I2C, address: u8) -> Self {
        Self { i2c, address }
    }

    pub fn read_register(&mut self, reg: u8) -> Result<u8, I2C::Error> {
        let mut buffer = [0u8];
        self.i2c.write_read(self.address, &[reg], &mut buffer)?;
        Ok(buffer[0])
    }
}
```

### Using the Driver

```rust
// On STM32
let i2c = stm32_hal::i2c::I2c::new(/* ... */);
let mut sensor = MyDriver::new(i2c, 0x48);

// On nRF52
let i2c = nrf52_hal::twim::Twim::new(/* ... */);
let mut sensor = MyDriver::new(i2c, 0x48);

// Same driver, different platforms!
```

## Error Handling

embedded-hal uses associated error types:

```rust
fn configure<I: I2c>(i2c: &mut I) -> Result<(), I::Error> {
    i2c.write(0x48, &[0x01, 0xFF])?;
    Ok(())
}
```

Handle errors appropriately:

```rust
match sensor.read() {
    Ok(data) => {
        // Use data
    }
    Err(e) => {
        // Handle error
        // - Retry
        // - Reset device
        // - Fall back to safe state
    }
}
```

## Common Patterns

### Initialization

```rust
impl<I: I2c> Sensor<I> {
    pub fn init<D: DelayNs>(&mut self, delay: &mut D) -> Result<(), I::Error> {
        // Reset
        self.write_register(RESET_REG, 0x01)?;
        delay.delay_ms(10);

        // Configure
        self.write_register(CONFIG_REG, 0x27)?;
        delay.delay_ms(1);

        // Verify
        let id = self.read_register(ID_REG)?;
        if id != EXPECTED_ID {
            return Err(/* error */);
        }

        Ok(())
    }
}
```

### Transactions

```rust
// SPI transaction (atomic operation)
spi.transaction(&mut [
    Operation::Write(&[0x20, 0x8F]), // Write config
    Operation::DelayNs(1000),         // Wait
    Operation::Write(&[0x3B]),        // Read command
    Operation::Read(&mut buffer),      // Read data
])?;
```

### Shared Bus

```rust
use embedded_hal_bus::spi::ExclusiveDevice;

let spi_bus = /* SPI peripheral */;
let cs1 = /* CS pin 1 */;
let cs2 = /* CS pin 2 */;

let device1 = ExclusiveDevice::new(spi_bus, cs1, delay);
let device2 = ExclusiveDevice::new(spi_bus, cs2, delay);
```

## Testing Drivers

### Mock Implementations

```rust
struct MockI2c {
    written: Vec<u8>,
}

impl I2c for MockI2c {
    type Error = ();

    fn write(&mut self, _addr: u8, bytes: &[u8]) -> Result<(), ()> {
        self.written.extend_from_slice(bytes);
        Ok(())
    }

    // ... other methods
}

#[test]
fn test_driver() {
    let mock_i2c = MockI2c::new();
    let mut driver = MyDriver::new(mock_i2c, 0x48);

    driver.configure().unwrap();
    assert_eq!(mock_i2c.written, &[0x01, 0xFF]);
}
```

## Platform HAL Crates

### STM32
```toml
stm32f4xx-hal = "0.21"
```

### nRF52
```toml
nrf52840-hal = "0.17"
```

### ESP32
```toml
esp32-hal = "0.18"
```

### RP2040
```toml
rp2040-hal = "0.10"
```

### RISC-V
```toml
riscv-hal = "0.1"
```

## Building

This example uses mock implementations for demonstration:

```bash
# Build for any target
cargo build --target thumbv7em-none-eabihf
cargo build --target riscv32imac-unknown-none-elf
```

For real hardware, replace mock implementations with platform HAL:

```rust
use stm32f4xx_hal as hal;

let dp = hal::pac::Peripherals::take().unwrap();
let rcc = dp.RCC.constrain();
let clocks = rcc.cfgr.freeze();

let gpioa = dp.GPIOA.split();
let led = gpioa.pa5.into_push_pull_output();

// Now use with portable driver!
blink_led(&mut led, &mut delay);
```

## Best Practices

1. **Use traits, not concrete types**
   ```rust
   // Good
   fn init<I: I2c>(i2c: &mut I) { }

   // Bad
   fn init(i2c: &mut Stm32I2c) { }
   ```

2. **Return trait errors**
   ```rust
   fn read<I: I2c>(&mut self) -> Result<u8, I::Error>
   ```

3. **Don't assume timing**
   - Always use `DelayNs` trait
   - Don't rely on CPU speed

4. **Handle all errors**
   - Don't unwrap in library code
   - Let caller decide error handling

5. **Document requirements**
   ```rust
   /// Reads sensor data
   ///
   /// # Requirements
   /// - Sensor must be initialized first
   /// - I2C bus must run at 100kHz or 400kHz
   pub fn read(&mut self) -> Result<Data, Error>
   ```

## Real-World Example

Complete sensor driver:

```rust
pub struct Bme280<I> {
    i2c: I,
    address: u8,
    cal_data: CalibrationData,
}

impl<I: I2c> Bme280<I> {
    pub fn new(i2c: I, address: u8) -> Self {
        Self {
            i2c,
            address,
            cal_data: CalibrationData::default(),
        }
    }

    pub fn init<D: DelayNs>(&mut self, delay: &mut D) -> Result<(), I::Error> {
        // Read calibration data
        self.read_calibration()?;

        // Configure sensor
        self.write_register(CTRL_MEAS, 0x27)?;
        delay.delay_ms(10);

        Ok(())
    }

    pub fn read_temperature(&mut self) -> Result<f32, I::Error> {
        let raw = self.read_raw_temp()?;
        Ok(self.compensate_temp(raw))
    }
}
```

## References

- [embedded-hal documentation](https://docs.rs/embedded-hal/)
- [embedded-hal book](https://github.com/rust-embedded/embedded-hal)
- [awesome-embedded-rust drivers](https://github.com/rust-embedded/awesome-embedded-rust#driver-crates)
- [The Embedded Rust Book](https://rust-embedded.github.io/book/)
