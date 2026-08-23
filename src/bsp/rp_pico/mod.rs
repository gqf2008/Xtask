pub use rp2040_hal as hal;
pub mod led;
pub mod stdout;

hal::bsp_pins!(
    /// GPIO 0 supports following functions:
    ///
    /// | Function     | Alias with applied function |
    /// |--------------|-----------------------------|
    /// | `SPI0 RX`    | [crate::Gp0Spi0Rx]          |
    /// | `UART0 TX`   | [crate::Gp0Uart0Tx]         |
    /// | `I2C0 SDA`   | [crate::Gp0I2C0Sda]         |
    /// | `PWM0 A`     | [crate::Gp0Pwm0A]           |
    /// | `PIO0`       | [crate::Gp0Pio0]            |
    /// | `PIO1`       | [crate::Gp0Pio1]            |
    Gpio0 {
        name: gpio0,
        aliases: {
            /// UART Function alias for pin [crate::Pins::gpio0].
            FunctionUart, PullDown: Gp0Uart0Tx,
            /// SPI Function alias for pin [crate::Pins::gpio0].
            FunctionSpi, PullDown: Gp0Spi0Rx,
            /// I2C Function alias for pin [crate::Pins::gpio0].
            FunctionI2C, PullDown: Gp0I2C0Sda,
            /// PWM Function alias for pin [crate::Pins::gpio0].
            FunctionPwm, PullDown: Gp0Pwm0A,
            /// PIO0 Function alias for pin [crate::Pins::gpio0].
            FunctionPio0, PullDown: Gp0Pio0,
            /// PIO1 Function alias for pin [crate::Pins::gpio0].
            FunctionPio1, PullDown: Gp0Pio1
        }
    },

    /// GPIO 1 supports following functions:
    ///
    /// | Function     | Alias with applied function |
    /// |--------------|-----------------------------|
    /// | `SPI0 CSn`   | [crate::Gp1Spi0Csn]         |
    /// | `UART0 RX`   | [crate::Gp1Uart0Rx]         |
    /// | `I2C0 SCL`   | [crate::Gp1I2C0Scl]         |
    /// | `PWM0 B`     | [crate::Gp1Pwm0B]           |
    /// | `PIO0`       | [crate::Gp1Pio0]            |
    /// | `PIO1`       | [crate::Gp1Pio1]            |
    Gpio1 {
        name: gpio1,
        aliases: {
            /// UART Function alias for pin [crate::Pins::gpio1].
            FunctionUart, PullDown: Gp1Uart0Rx,
            /// SPI Function alias for pin [crate::Pins::gpio1].
            FunctionSpi, PullDown: Gp1Spi0Csn,
            /// I2C Function alias for pin [crate::Pins::gpio1].
            FunctionI2C, PullDown: Gp1I2C0Scl,
            /// PWM Function alias for pin [crate::Pins::gpio1].
            FunctionPwm, PullDown: Gp1Pwm0B,
            /// PIO0 Function alias for pin [crate::Pins::gpio1].
            FunctionPio0, PullDown: Gp1Pio0,
            /// PIO1 Function alias for pin [crate::Pins::gpio1].
            FunctionPio1, PullDown: Gp1Pio1
        }
    },

    /// GPIO 2 supports following functions:
    ///
    /// | Function     | Alias with applied function |
    /// |--------------|-----------------------------|
    /// | `SPI0 SCK`   | [crate::Gp2Spi0Sck]         |
    /// | `UART0 CTS`  | [crate::Gp2Uart0Cts]        |
    /// | `I2C1 SDA`   | [crate::Gp2I2C1Sda]         |
    /// | `PWM1 A`     | [crate::Gp2Pwm1A]           |
    /// | `PIO0`       | [crate::Gp2Pio0]            |
    /// | `PIO1`       | [crate::Gp2Pio1]            |
    Gpio2 {
        name: gpio2,
        aliases: {
            /// UART Function alias for pin [crate::Pins::gpio2].
            FunctionUart, PullDown: Gp2Uart0Cts,
            /// SPI Function alias for pin [crate::Pins::gpio2].
            FunctionSpi, PullDown: Gp2Spi0Sck,
            /// I2C Function alias for pin [crate::Pins::gpio2].
            FunctionI2C, PullDown: Gp2I2C1Sda,
            /// PWM Function alias for pin [crate::Pins::gpio2].
            FunctionPwm, PullDown: Gp2Pwm1A,
            /// PIO0 Function alias for pin [crate::Pins::gpio2].
            FunctionPio0, PullDown: Gp2Pio0,
            /// PIO1 Function alias for pin [crate::Pins::gpio2].
            FunctionPio1, PullDown: Gp2Pio1
        }
    },

    /// GPIO 3 supports following functions:
    ///
    /// | Function     | Alias with applied function |
    /// |--------------|-----------------------------|
    /// | `SPI0 TX`    | [crate::Gp3Spi0Tx]          |
    /// | `UART0 RTS`  | [crate::Gp3Uart0Rts]        |
    /// | `I2C1 SCL`   | [crate::Gp3I2C1Scl]         |
    /// | `PWM1 B`     | [crate::Gp3Pwm1B]           |
    /// | `PIO0`       | [crate::Gp3Pio0]            |
    /// | `PIO1`       | [crate::Gp3Pio1]            |
    Gpio3 {
        name: gpio3,
        aliases: {
            /// UART Function alias for pin [crate::Pins::gpio3].
            FunctionUart, PullDown: Gp3Uart0Rts,
            /// SPI Function alias for pin [crate::Pins::gpio3].
            FunctionSpi, PullDown: Gp3Spi0Tx,
            /// I2C Function alias for pin [crate::Pins::gpio3].
            FunctionI2C, PullDown: Gp3I2C1Scl,
            /// PWM Function alias for pin [crate::Pins::gpio3].
            FunctionPwm, PullDown: Gp3Pwm1B,
            /// PIO0 Function alias for pin [crate::Pins::gpio3].
            FunctionPio0, PullDown: Gp3Pio0,
            /// PIO1 Function alias for pin [crate::Pins::gpio3].
            FunctionPio1, PullDown: Gp3Pio1
        }
    },

    /// GPIO 4 supports following functions:
    ///
    /// | Function     | Alias with applied function |
    /// |--------------|-----------------------------|
    /// | `SPI0 RX`    | [crate::Gp4Spi0Rx]          |
    /// | `UART1 TX`   | [crate::Gp4Uart1Tx]         |
    /// | `I2C0 SDA`   | [crate::Gp4I2C0Sda]         |
    /// | `PWM2 A`     | [crate::Gp4Pwm2A]           |
    /// | `PIO0`       | [crate::Gp4Pio0]            |
    /// | `PIO1`       | [crate::Gp4Pio1]            |
    Gpio4 {
        name: gpio4,
        aliases: {
            /// UART Function alias for pin [crate::Pins::gpio4].
            FunctionUart, PullDown: Gp4Uart1Tx,
            /// SPI Function alias for pin [crate::Pins::gpio4].
            FunctionSpi, PullDown: Gp4Spi0Rx,
            /// I2C Function alias for pin [crate::Pins::gpio4].
            FunctionI2C, PullDown: Gp4I2C0Sda,
            /// PWM Function alias for pin [crate::Pins::gpio4].
            FunctionPwm, PullDown: Gp4Pwm2A,
            /// PIO0 Function alias for pin [crate::Pins::gpio4].
            FunctionPio0, PullDown: Gp4Pio0,
            /// PIO1 Function alias for pin [crate::Pins::gpio4].
            FunctionPio1, PullDown: Gp4Pio1
        }
    },

    /// GPIO 5 supports following functions:
    ///
    /// | Function     | Alias with applied function |
    /// |--------------|-----------------------------|
    /// | `SPI0 CSn`   | [crate::Gp5Spi0Csn]         |
    /// | `UART1 RX`   | [crate::Gp5Uart1Rx]         |
    /// | `I2C0 SCL`   | [crate::Gp5I2C0Scl]         |
    /// | `PWM2 B`     | [crate::Gp5Pwm2B]           |
    /// | `PIO0`       | [crate::Gp5Pio0]            |
    /// | `PIO1`       | [crate::Gp5Pio1]            |
    Gpio5 {
        name: gpio5,
        aliases: {
            /// UART Function alias for pin [crate::Pins::gpio5].
            FunctionUart, PullDown: Gp5Uart1Rx,
            /// SPI Function alias for pin [crate::Pins::gpio5].
            FunctionSpi, PullDown: Gp5Spi0Csn,
            /// I2C Function alias for pin [crate::Pins::gpio5].
            FunctionI2C, PullDown: Gp5I2C0Scl,
            /// PWM Function alias for pin [crate::Pins::gpio5].
            FunctionPwm, PullDown: Gp5Pwm2B,
            /// PIO0 Function alias for pin [crate::Pins::gpio5].
            FunctionPio0, PullDown: Gp5Pio0,
            /// PIO1 Function alias for pin [crate::Pins::gpio5].
            FunctionPio1, PullDown: Gp5Pio1
        }
    },

    /// GPIO 6 supports following functions:
    ///
    /// | Function     | Alias with applied function |
    /// |--------------|-----------------------------|
    /// | `SPI0 SCK`   | [crate::Gp6Spi0Sck]         |
    /// | `UART1 CTS`  | [crate::Gp6Uart1Cts]        |
    /// | `I2C1 SDA`   | [crate::Gp6I2C1Sda]         |
    /// | `PWM3 A`     | [crate::Gp6Pwm3A]           |
    /// | `PIO0`       | [crate::Gp6Pio0]            |
    /// | `PIO1`       | [crate::Gp6Pio1]            |
    Gpio6 {
        name: gpio6,
        aliases: {
            /// UART Function alias for pin [crate::Pins::gpio6].
            FunctionUart, PullDown: Gp6Uart1Cts,
            /// SPI Function alias for pin [crate::Pins::gpio6].
            FunctionSpi, PullDown: Gp6Spi0Sck,
            /// I2C Function alias for pin [crate::Pins::gpio6].
            FunctionI2C, PullDown: Gp6I2C1Sda,
            /// PWM Function alias for pin [crate::Pins::gpio6].
            FunctionPwm, PullDown: Gp6Pwm3A,
            /// PIO0 Function alias for pin [crate::Pins::gpio6].
            FunctionPio0, PullDown: Gp6Pio0,
            /// PIO1 Function alias for pin [crate::Pins::gpio6].
            FunctionPio1, PullDown: Gp6Pio1
        }
    },

    /// GPIO 7 supports following functions:
    ///
    /// | Function     | Alias with applied function |
    /// |--------------|-----------------------------|
    /// | `SPI0 TX`    | [crate::Gp7Spi0Tx]          |
    /// | `UART1 RTS`  | [crate::Gp7Uart1Rts]        |
    /// | `I2C1 SCL`   | [crate::Gp7I2C1Scl]         |
    /// | `PWM3 B`     | [crate::Gp7Pwm3B]           |
    /// | `PIO0`       | [crate::Gp7Pio0]            |
    /// | `PIO1`       | [crate::Gp7Pio1]            |
    Gpio7 {
        name: gpio7,
        aliases: {
            /// UART Function alias for pin [crate::Pins::gpio7].
            FunctionUart, PullDown: Gp7Uart1Rts,
            /// SPI Function alias for pin [crate::Pins::gpio7].
            FunctionSpi, PullDown: Gp7Spi0Tx,
            /// I2C Function alias for pin [crate::Pins::gpio7].
            FunctionI2C, PullDown: Gp7I2C1Scl,
            /// PWM Function alias for pin [crate::Pins::gpio7].
            FunctionPwm, PullDown: Gp7Pwm3B,
            /// PIO0 Function alias for pin [crate::Pins::gpio7].
            FunctionPio0, PullDown: Gp7Pio0,
            /// PIO1 Function alias for pin [crate::Pins::gpio7].
            FunctionPio1, PullDown: Gp7Pio1
        }
    },

    /// GPIO 8 supports following functions:
    ///
    /// | Function     | Alias with applied function |
    /// |--------------|-----------------------------|
    /// | `SPI1 RX`    | [crate::Gp8Spi1Rx]          |
    /// | `UART1 TX`   | [crate::Gp8Uart1Tx]         |
    /// | `I2C0 SDA`   | [crate::Gp8I2C0Sda]         |
    /// | `PWM4 A`     | [crate::Gp8Pwm4A]           |
    /// | `PIO0`       | [crate::Gp8Pio0]            |
    /// | `PIO1`       | [crate::Gp8Pio1]            |
    Gpio8 {
        name: gpio8,
        aliases: {
            /// UART Function alias for pin [crate::Pins::gpio8].
            FunctionUart, PullDown: Gp8Uart1Tx,
            /// SPI Function alias for pin [crate::Pins::gpio8].
            FunctionSpi, PullDown: Gp8Spi1Rx,
            /// I2C Function alias for pin [crate::Pins::gpio8].
            FunctionI2C, PullDown: Gp8I2C0Sda,
            /// PWM Function alias for pin [crate::Pins::gpio8].
            FunctionPwm, PullDown: Gp8Pwm4A,
            /// PIO0 Function alias for pin [crate::Pins::gpio8].
            FunctionPio0, PullDown: Gp8Pio0,
            /// PIO1 Function alias for pin [crate::Pins::gpio8].
            FunctionPio1, PullDown: Gp8Pio1
        }
    },

    /// GPIO 9 supports following functions:
    ///
    /// | Function     | Alias with applied function |
    /// |--------------|-----------------------------|
    /// | `SPI1 CSn`   | [crate::Gp9Spi1Csn]         |
    /// | `UART1 RX`   | [crate::Gp9Uart1Rx]         |
    /// | `I2C0 SCL`   | [crate::Gp9I2C0Scl]         |
    /// | `PWM4 B`     | [crate::Gp9Pwm4B]           |
    /// | `PIO0`       | [crate::Gp9Pio0]            |
    /// | `PIO1`       | [crate::Gp9Pio1]            |
    Gpio9 {
        name: gpio9,
        aliases: {
            /// UART Function alias for pin [crate::Pins::gpio9].
            FunctionUart, PullDown: Gp9Uart1Rx,
            /// SPI Function alias for pin [crate::Pins::gpio9].
            FunctionSpi, PullDown: Gp9Spi1Csn,
            /// I2C Function alias for pin [crate::Pins::gpio9].
            FunctionI2C, PullDown: Gp9I2C0Scl,
            /// PWM Function alias for pin [crate::Pins::gpio9].
            FunctionPwm, PullDown: Gp9Pwm4B,
            /// PIO0 Function alias for pin [crate::Pins::gpio9].
            FunctionPio0, PullDown: Gp9Pio0,
            /// PIO1 Function alias for pin [crate::Pins::gpio9].
            FunctionPio1, PullDown: Gp9Pio1
        }
    },

    /// GPIO 10 supports following functions:
    ///
    /// | Function     | Alias with applied function |
    /// |--------------|-----------------------------|
    /// | `SPI1 SCK`   | [crate::Gp10Spi1Sck]        |
    /// | `UART1 CTS`  | [crate::Gp10Uart1Cts]       |
    /// | `I2C1 SDA`   | [crate::Gp10I2C1Sda]        |
    /// | `PWM5 A`     | [crate::Gp10Pwm5A]          |
    /// | `PIO0`       | [crate::Gp10Pio0]           |
    /// | `PIO1`       | [crate::Gp10Pio1]           |
    Gpio10 {
        name: gpio10,
        aliases: {
            /// UART Function alias for pin [crate::Pins::gpio10].
            FunctionUart, PullDown: Gp10Uart1Cts,
            /// SPI Function alias for pin [crate::Pins::gpio10].
            FunctionSpi, PullDown: Gp10Spi1Sck,
            /// I2C Function alias for pin [crate::Pins::gpio10].
            FunctionI2C, PullDown: Gp10I2C1Sda,
            /// PWM Function alias for pin [crate::Pins::gpio10].
            FunctionPwm, PullDown: Gp10Pwm5A,
            /// PIO0 Function alias for pin [crate::Pins::gpio10].
            FunctionPio0, PullDown: Gp10Pio0,
            /// PIO1 Function alias for pin [crate::Pins::gpio10].
            FunctionPio1, PullDown: Gp10Pio1
        }
    },

    /// GPIO 11 supports following functions:
    ///
    /// | Function     | Alias with applied function |
    /// |--------------|-----------------------------|
    /// | `SPI1 TX`    | [crate::Gp11Spi1Tx]         |
    /// | `UART1 RTS`  | [crate::Gp11Uart1Rts]       |
    /// | `I2C1 SCL`   | [crate::Gp11I2C1Scl]        |
    /// | `PWM5 B`     | [crate::Gp11Pwm5B]          |
    /// | `PIO0`       | [crate::Gp11Pio0]           |
    /// | `PIO1`       | [crate::Gp11Pio1]           |
    Gpio11 {
        name: gpio11,
        aliases: {
            /// UART Function alias for pin [crate::Pins::gpio11].
            FunctionUart, PullDown: Gp11Uart1Rts,
            /// SPI Function alias for pin [crate::Pins::gpio11].
            FunctionSpi, PullDown: Gp11Spi1Tx,
            /// I2C Function alias for pin [crate::Pins::gpio11].
            FunctionI2C, PullDown: Gp11I2C1Scl,
            /// PWM Function alias for pin [crate::Pins::gpio11].
            FunctionPwm, PullDown: Gp11Pwm5B,
            /// PIO0 Function alias for pin [crate::Pins::gpio11].
            FunctionPio0, PullDown: Gp11Pio0,
            /// PIO1 Function alias for pin [crate::Pins::gpio11].
            FunctionPio1, PullDown: Gp11Pio1
        }
    },

    /// GPIO 12 supports following functions:
    ///
    /// | Function     | Alias with applied function |
    /// |--------------|-----------------------------|
    /// | `SPI1 RX`    | [crate::Gp12Spi1Rx]         |
    /// | `UART0 TX`   | [crate::Gp12Uart0Tx]        |
    /// | `I2C0 SDA`   | [crate::Gp12I2C0Sda]        |
    /// | `PWM6 A`     | [crate::Gp12Pwm6A]          |
    /// | `PIO0`       | [crate::Gp12Pio0]           |
    /// | `PIO1`       | [crate::Gp12Pio1]           |
    Gpio12 {
        name: gpio12,
        aliases: {
            /// UART Function alias for pin [crate::Pins::gpio12].
            FunctionUart, PullDown: Gp12Uart0Tx,
            /// SPI Function alias for pin [crate::Pins::gpio12].
            FunctionSpi, PullDown: Gp12Spi1Rx,
            /// I2C Function alias for pin [crate::Pins::gpio12].
            FunctionI2C, PullDown: Gp12I2C0Sda,
            /// PWM Function alias for pin [crate::Pins::gpio12].
            FunctionPwm, PullDown: Gp12Pwm6A,
            /// PIO0 Function alias for pin [crate::Pins::gpio12].
            FunctionPio0, PullDown: Gp12Pio0,
            /// PIO1 Function alias for pin [crate::Pins::gpio12].
            FunctionPio1, PullDown: Gp12Pio1
        }
    },

    /// GPIO 13 supports following functions:
    ///
    /// | Function     | Alias with applied function |
    /// |--------------|-----------------------------|
    /// | `SPI1 CSn`   | [crate::Gp13Spi1Csn]        |
    /// | `UART0 RX`   | [crate::Gp13Uart0Rx]        |
    /// | `I2C0 SCL`   | [crate::Gp13I2C0Scl]        |
    /// | `PWM6 B`     | [crate::Gp13Pwm6B]          |
    /// | `PIO0`       | [crate::Gp13Pio0]           |
    /// | `PIO1`       | [crate::Gp13Pio1]           |
    Gpio13 {
        name: gpio13,
        aliases: {
            /// UART Function alias for pin [crate::Pins::gpio13].
            FunctionUart, PullDown: Gp13Uart0Rx,
            /// SPI Function alias for pin [crate::Pins::gpio13].
            FunctionSpi, PullDown: Gp13Spi1Csn,
            /// I2C Function alias for pin [crate::Pins::gpio13].
            FunctionI2C, PullDown: Gp13I2C0Scl,
            /// PWM Function alias for pin [crate::Pins::gpio13].
            FunctionPwm, PullDown: Gp13Pwm6B,
            /// PIO0 Function alias for pin [crate::Pins::gpio13].
            FunctionPio0, PullDown: Gp13Pio0,
            /// PIO1 Function alias for pin [crate::Pins::gpio13].
            FunctionPio1, PullDown: Gp13Pio1
        }
    },

    /// GPIO 14 supports following functions:
    ///
    /// | Function     | Alias with applied function |
    /// |--------------|-----------------------------|
    /// | `SPI1 SCK`   | [crate::Gp14Spi1Sck]        |
    /// | `UART0 CTS`  | [crate::Gp14Uart0Cts]       |
    /// | `I2C1 SDA`   | [crate::Gp14I2C1Sda]        |
    /// | `PWM7 A`     | [crate::Gp14Pwm7A]          |
    /// | `PIO0`       | [crate::Gp14Pio0]           |
    /// | `PIO1`       | [crate::Gp14Pio1]           |
    Gpio14 {
        name: gpio14,
        aliases: {
            /// UART Function alias for pin [crate::Pins::gpio14].
            FunctionUart, PullDown: Gp14Uart0Cts,
            /// SPI Function alias for pin [crate::Pins::gpio14].
            FunctionSpi, PullDown: Gp14Spi1Sck,
            /// I2C Function alias for pin [crate::Pins::gpio14].
            FunctionI2C, PullDown: Gp14I2C1Sda,
            /// PWM Function alias for pin [crate::Pins::gpio14].
            FunctionPwm, PullDown: Gp14Pwm7A,
            /// PIO0 Function alias for pin [crate::Pins::gpio14].
            FunctionPio0, PullDown: Gp14Pio0,
            /// PIO1 Function alias for pin [crate::Pins::gpio14].
            FunctionPio1, PullDown: Gp14Pio1
        }
    },

    /// GPIO 15 supports following functions:
    ///
    /// | Function     | Alias with applied function |
    /// |--------------|-----------------------------|
    /// | `SPI1 TX`    | [crate::Gp15Spi1Tx]         |
    /// | `UART0 RTS`  | [crate::Gp15Uart0Rts]       |
    /// | `I2C1 SCL`   | [crate::Gp15I2C1Scl]        |
    /// | `PWM7 B`     | [crate::Gp15Pwm7B]          |
    /// | `PIO0`       | [crate::Gp15Pio0]           |
    /// | `PIO1`       | [crate::Gp15Pio1]           |
    Gpio15 {
        name: gpio15,
        aliases: {
            /// UART Function alias for pin [crate::Pins::gpio15].
            FunctionUart, PullDown: Gp15Uart0Rts,
            /// SPI Function alias for pin [crate::Pins::gpio15].
            FunctionSpi, PullDown: Gp15Spi1Tx,
            /// I2C Function alias for pin [crate::Pins::gpio15].
            FunctionI2C, PullDown: Gp15I2C1Scl,
            /// PWM Function alias for pin [crate::Pins::gpio15].
            FunctionPwm, PullDown: Gp15Pwm7B,
            /// PIO0 Function alias for pin [crate::Pins::gpio15].
            FunctionPio0, PullDown: Gp15Pio0,
            /// PIO1 Function alias for pin [crate::Pins::gpio15].
            FunctionPio1, PullDown: Gp15Pio1
        }
    },

    /// GPIO 16 supports following functions:
    ///
    /// | Function     | Alias with applied function |
    /// |--------------|-----------------------------|
    /// | `SPI0 RX`    | [crate::Gp16Spi0Rx]         |
    /// | `UART0 TX`   | [crate::Gp16Uart0Tx]        |
    /// | `I2C0 SDA`   | [crate::Gp16I2C0Sda]        |
    /// | `PWM0 A`     | [crate::Gp16Pwm0A]          |
    /// | `PIO0`       | [crate::Gp16Pio0]           |
    /// | `PIO1`       | [crate::Gp16Pio1]           |
    Gpio16 {
        name: gpio16,
        aliases: {
            /// UART Function alias for pin [crate::Pins::gpio16].
            FunctionUart, PullDown: Gp16Uart0Tx,
            /// SPI Function alias for pin [crate::Pins::gpio16].
            FunctionSpi, PullDown: Gp16Spi0Rx,
            /// I2C Function alias for pin [crate::Pins::gpio16].
            FunctionI2C, PullDown: Gp16I2C0Sda,
            /// PWM Function alias for pin [crate::Pins::gpio16].
            FunctionPwm, PullDown: Gp16Pwm0A,
            /// PIO0 Function alias for pin [crate::Pins::gpio16].
            FunctionPio0, PullDown: Gp16Pio0,
            /// PIO1 Function alias for pin [crate::Pins::gpio16].
            FunctionPio1, PullDown: Gp16Pio1
        }
    },

    /// GPIO 17 supports following functions:
    ///
    /// | Function     | Alias with applied function |
    /// |--------------|-----------------------------|
    /// | `SPI0 CSn`   | [crate::Gp17Spi0Csn]        |
    /// | `UART0 RX`   | [crate::Gp17Uart0Rx]        |
    /// | `I2C0 SCL`   | [crate::Gp17I2C0Scl]        |
    /// | `PWM0 B`     | [crate::Gp17Pwm0B]          |
    /// | `PIO0`       | [crate::Gp17Pio0]           |
    /// | `PIO1`       | [crate::Gp17Pio1]           |
    Gpio17 {
        name: gpio17,
        aliases: {
            /// UART Function alias for pin [crate::Pins::gpio17].
            FunctionUart, PullDown: Gp17Uart0Rx,
            /// SPI Function alias for pin [crate::Pins::gpio17].
            FunctionSpi, PullDown: Gp17Spi0Csn,
            /// I2C Function alias for pin [crate::Pins::gpio17].
            FunctionI2C, PullDown: Gp17I2C0Scl,
            /// PWM Function alias for pin [crate::Pins::gpio17].
            FunctionPwm, PullDown: Gp17Pwm0B,
            /// PIO0 Function alias for pin [crate::Pins::gpio17].
            FunctionPio0, PullDown: Gp17Pio0,
            /// PIO1 Function alias for pin [crate::Pins::gpio17].
            FunctionPio1, PullDown: Gp17Pio1
        }
    },

    /// GPIO 18 supports following functions:
    ///
    /// | Function     | Alias with applied function |
    /// |--------------|-----------------------------|
    /// | `SPI0 SCK`   | [crate::Gp18Spi0Sck]        |
    /// | `UART0 CTS`  | [crate::Gp18Uart0Cts]       |
    /// | `I2C1 SDA`   | [crate::Gp18I2C1Sda]        |
    /// | `PWM1 A`     | [crate::Gp18Pwm1A]          |
    /// | `PIO0`       | [crate::Gp18Pio0]           |
    /// | `PIO1`       | [crate::Gp18Pio1]           |
    Gpio18 {
        name: gpio18,
        aliases: {
            /// UART Function alias for pin [crate::Pins::gpio18].
            FunctionUart, PullDown: Gp18Uart0Cts,
            /// SPI Function alias for pin [crate::Pins::gpio18].
            FunctionSpi, PullDown: Gp18Spi0Sck,
            /// I2C Function alias for pin [crate::Pins::gpio18].
            FunctionI2C, PullDown: Gp18I2C1Sda,
            /// PWM Function alias for pin [crate::Pins::gpio18].
            FunctionPwm, PullDown: Gp18Pwm1A,
            /// PIO0 Function alias for pin [crate::Pins::gpio18].
            FunctionPio0, PullDown: Gp18Pio0,
            /// PIO1 Function alias for pin [crate::Pins::gpio18].
            FunctionPio1, PullDown: Gp18Pio1
        }
    },

    /// GPIO 19 supports following functions:
    ///
    /// | Function     | Alias with applied function |
    /// |--------------|-----------------------------|
    /// | `SPI0 TX`    | [crate::Gp19Spi0Tx]         |
    /// | `UART0 RTS`  | [crate::Gp19Uart0Rts]       |
    /// | `I2C1 SCL`   | [crate::Gp19I2C1Scl]        |
    /// | `PWM1 B`     | [crate::Gp19Pwm1B]          |
    /// | `PIO0`       | [crate::Gp19Pio0]           |
    /// | `PIO1`       | [crate::Gp19Pio1]           |
    Gpio19 {
        name: gpio19,
        aliases: {
            /// UART Function alias for pin [crate::Pins::gpio19].
            FunctionUart, PullDown: Gp19Uart0Rts,
            /// SPI Function alias for pin [crate::Pins::gpio19].
            FunctionSpi, PullDown: Gp19Spi0Tx,
            /// I2C Function alias for pin [crate::Pins::gpio19].
            FunctionI2C, PullDown: Gp19I2C1Scl,
            /// PWM Function alias for pin [crate::Pins::gpio19].
            FunctionPwm, PullDown: Gp19Pwm1B,
            /// PIO0 Function alias for pin [crate::Pins::gpio19].
            FunctionPio0, PullDown: Gp19Pio0,
            /// PIO1 Function alias for pin [crate::Pins::gpio19].
            FunctionPio1, PullDown: Gp19Pio1
        }
    },

    /// GPIO 20 supports following functions:
    ///
    /// | Function     | Alias with applied function |
    /// |--------------|-----------------------------|
    /// | `SPI0 RX`    | [crate::Gp20Spi0Rx]         |
    /// | `UART1 TX`   | [crate::Gp20Uart1Tx]        |
    /// | `I2C0 SDA`   | [crate::Gp20I2C0Sda]        |
    /// | `PWM2 A`     | [crate::Gp20Pwm2A]          |
    /// | `PIO0`       | [crate::Gp20Pio0]           |
    /// | `PIO1`       | [crate::Gp20Pio1]           |
    Gpio20 {
        name: gpio20,
        aliases: {
            /// UART Function alias for pin [crate::Pins::gpio20].
            FunctionUart, PullDown: Gp20Uart1Tx,
            /// SPI Function alias for pin [crate::Pins::gpio20].
            FunctionSpi, PullDown: Gp20Spi0Rx,
            /// I2C Function alias for pin [crate::Pins::gpio20].
            FunctionI2C, PullDown: Gp20I2C0Sda,
            /// PWM Function alias for pin [crate::Pins::gpio20].
            FunctionPwm, PullDown: Gp20Pwm2A,
            /// PIO0 Function alias for pin [crate::Pins::gpio20].
            FunctionPio0, PullDown: Gp20Pio0,
            /// PIO1 Function alias for pin [crate::Pins::gpio20].
            FunctionPio1, PullDown: Gp20Pio1
        }
    },

    /// GPIO 21 supports following functions:
    ///
    /// | Function     | Alias with applied function |
    /// |--------------|-----------------------------|
    /// | `SPI0 CSn`   | [crate::Gp21Spi0Csn]        |
    /// | `UART1 RX`   | [crate::Gp21Uart1Rx]        |
    /// | `I2C0 SCL`   | [crate::Gp21I2C0Scl]        |
    /// | `PWM2 B`     | [crate::Gp21Pwm2B]          |
    /// | `PIO0`       | [crate::Gp21Pio0]           |
    /// | `PIO1`       | [crate::Gp21Pio1]           |
    Gpio21 {
        name: gpio21,
        aliases: {
            /// UART Function alias for pin [crate::Pins::gpio21].
            FunctionUart, PullDown: Gp21Uart1Rx,
            /// SPI Function alias for pin [crate::Pins::gpio21].
            FunctionSpi, PullDown: Gp21Spi0Csn,
            /// I2C Function alias for pin [crate::Pins::gpio21].
            FunctionI2C, PullDown: Gp21I2C0Scl,
            /// PWM Function alias for pin [crate::Pins::gpio21].
            FunctionPwm, PullDown: Gp21Pwm2B,
            /// PIO0 Function alias for pin [crate::Pins::gpio21].
            FunctionPio0, PullDown: Gp21Pio0,
            /// PIO1 Function alias for pin [crate::Pins::gpio21].
            FunctionPio1, PullDown: Gp21Pio1
        }
    },

    /// GPIO 22 supports following functions:
    ///
    /// | Function     | Alias with applied function |
    /// |--------------|-----------------------------|
    /// | `SPI0 SCK`   | [crate::Gp22Spi0Sck]        |
    /// | `UART1 CTS`  | [crate::Gp22Uart1Cts]       |
    /// | `I2C1 SDA`   | [crate::Gp22I2C1Sda]        |
    /// | `PWM3 A`     | [crate::Gp22Pwm3A]          |
    /// | `PIO0`       | [crate::Gp22Pio0]           |
    /// | `PIO1`       | [crate::Gp22Pio1]           |
    Gpio22 {
        name: gpio22,
        aliases: {
            /// UART Function alias for pin [crate::Pins::gpio22].
            FunctionUart, PullDown: Gp22Uart1Cts,
            /// SPI Function alias for pin [crate::Pins::gpio22].
            FunctionSpi, PullDown: Gp22Spi0Sck,
            /// I2C Function alias for pin [crate::Pins::gpio22].
            FunctionI2C, PullDown: Gp22I2C1Sda,
            /// PWM Function alias for pin [crate::Pins::gpio22].
            FunctionPwm, PullDown: Gp22Pwm3A,
            /// PIO0 Function alias for pin [crate::Pins::gpio22].
            FunctionPio0, PullDown: Gp22Pio0,
            /// PIO1 Function alias for pin [crate::Pins::gpio22].
            FunctionPio1, PullDown: Gp22Pio1
        }
    },

    /// GPIO 23 is connected to b_power_save of the Raspberry Pi Pico board.
    Gpio23 {
        name: b_power_save,
    },

    /// GPIO 24 is connected to vbus_detect of the Raspberry Pi Pico board.
    Gpio24 {
        name: vbus_detect,
    },

    /// GPIO 25 is connected to led of the Raspberry Pi Pico board.
    Gpio25 {
        name: led,
    },

    /// GPIO 26 supports following functions:
    ///
    /// | Function     | Alias with applied function |
    /// |--------------|-----------------------------|
    /// | `SPI1 SCK`   | [crate::Gp26Spi1Sck]        |
    /// | `UART1 CTS`  | [crate::Gp26Uart1Cts]       |
    /// | `I2C1 SDA`   | [crate::Gp26I2C1Sda]        |
    /// | `PWM5 A`     | [crate::Gp26Pwm5A]          |
    /// | `PIO0`       | [crate::Gp26Pio0]           |
    /// | `PIO1`       | [crate::Gp26Pio1]           |
    Gpio26 {
        name: gpio26,
        aliases: {
            /// UART Function alias for pin [crate::Pins::gpio26].
            FunctionUart, PullDown: Gp26Uart1Cts,
            /// SPI Function alias for pin [crate::Pins::gpio26].
            FunctionSpi, PullDown: Gp26Spi1Sck,
            /// I2C Function alias for pin [crate::Pins::gpio26].
            FunctionI2C, PullDown: Gp26I2C1Sda,
            /// PWM Function alias for pin [crate::Pins::gpio26].
            FunctionPwm, PullDown: Gp26Pwm5A,
            /// PIO0 Function alias for pin [crate::Pins::gpio26].
            FunctionPio0, PullDown: Gp26Pio0,
            /// PIO1 Function alias for pin [crate::Pins::gpio26].
            FunctionPio1, PullDown: Gp26Pio1
        }
    },

    /// GPIO 27 supports following functions:
    ///
    /// | Function     | Alias with applied function |
    /// |--------------|-----------------------------|
    /// | `SPI1 TX`    | [crate::Gp27Spi1Tx]         |
    /// | `UART1 RTS`  | [crate::Gp27Uart1Rts]       |
    /// | `I2C1 SCL`   | [crate::Gp27I2C1Scl]        |
    /// | `PWM5 B`     | [crate::Gp27Pwm5B]          |
    /// | `PIO0`       | [crate::Gp27Pio0]           |
    /// | `PIO1`       | [crate::Gp27Pio1]           |
    Gpio27 {
        name: gpio27,
        aliases: {
            /// UART Function alias for pin [crate::Pins::gpio27].
            FunctionUart, PullDown: Gp27Uart1Rts,
            /// SPI Function alias for pin [crate::Pins::gpio27].
            FunctionSpi, PullDown: Gp27Spi1Tx,
            /// I2C Function alias for pin [crate::Pins::gpio27].
            FunctionI2C, PullDown: Gp27I2C1Scl,
            /// PWM Function alias for pin [crate::Pins::gpio27].
            FunctionPwm, PullDown: Gp27Pwm5B,
            /// PIO0 Function alias for pin [crate::Pins::gpio27].
            FunctionPio0, PullDown: Gp27Pio0,
            /// PIO1 Function alias for pin [crate::Pins::gpio27].
            FunctionPio1, PullDown: Gp27Pio1
        }
    },

    /// GPIO 28 supports following functions:
    ///
    /// | Function     | Alias with applied function |
    /// |--------------|-----------------------------|
    /// | `SPI1 RX`    | [crate::Gp28Spi1Rx]         |
    /// | `UART0 TX`   | [crate::Gp28Uart0Tx]        |
    /// | `I2C0 SDA`   | [crate::Gp28I2C0Sda]        |
    /// | `PWM6 A`     | [crate::Gp28Pwm6A]          |
    /// | `PIO0`       | [crate::Gp28Pio0]           |
    /// | `PIO1`       | [crate::Gp28Pio1]           |
    Gpio28 {
        name: gpio28,
        aliases: {
            /// UART Function alias for pin [crate::Pins::gpio28].
            FunctionUart, PullDown: Gp28Uart0Tx,
            /// SPI Function alias for pin [crate::Pins::gpio28].
            FunctionSpi, PullDown: Gp28Spi1Rx,
            /// I2C Function alias for pin [crate::Pins::gpio28].
            FunctionI2C, PullDown: Gp28I2C0Sda,
            /// PWM Function alias for pin [crate::Pins::gpio28].
            FunctionPwm, PullDown: Gp28Pwm6A,
            /// PIO0 Function alias for pin [crate::Pins::gpio28].
            FunctionPio0, PullDown: Gp28Pio0,
            /// PIO1 Function alias for pin [crate::Pins::gpio28].
            FunctionPio1, PullDown: Gp28Pio1
        }
    },

    /// GPIO 29 is connected to voltage_monitor of the Raspberry Pi Pico board.
    Gpio29 {
        name: voltage_monitor,
    },
);

pub const XOSC_CRYSTAL_FREQ: u32 = 12_000_000;
