# Change Log

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

## [Unreleased]

### Added

### Changed

### Removed

## [v0.5.0] - 2021-09-04

### Added

- Add support for DMA
- Add GitHub Actions
- Add Spi::change_clock_frequency()
- Add PWM API for remapping pins [#29].
- Add ADC support

### Changed

- Use cchp.oaen bit to enable automatically all TIMER0 PWM channels
- Replaced unreachable!() with panic!()
- Enabling PWM is now explicit [#30].


### Removed

- Remove unused errifx and errifcx fields from the DMA driver code
- Remove Travis CI rules and status badge

## [v0.4.0] - 2020-11-01

### Added

- RTC driver
- Add I2C driver
- Add free watchdog
- Add ECLIC mode
- Add Fast Plus mode
- Add UPG to clear timer counter
- Add method to get gpio port and pin information
- Add EXTI support

### Changed

- Update riscv and gd32vf103-pac dependencies
- Fix PWM driver
- Rename des -> signature

### Removed

- Remove defines for unsupported ISA variants
- Remove rebase-hack.S

## [v0.3.0] - 2020-04-12

### Added

- Added Afio::disable_jtag()
- Added afio module docs

### Changed

- Binaries Regenerated
- Refactor AFIO and Serial
- Hide closed traits and make them pub(crate)
- Update dependencies, remove unused riscv-rt dependency
- Configure serial pins after enabling usart clock

## [v0.2.3] - 2020-02-26

### Changed

- Make GPIO pin activate() public

[Unreleased]: https://github.com/riscv-rust/gd32vf103xx-hal/compare/v0.5.0...HEAD
[v0.5.0]: https://github.com/riscv-rust/gd32vf103xx-hal/compare/v0.4.0...v0.5.0
[v0.4.0]: https://github.com/riscv-rust/gd32vf103xx-hal/compare/v0.3.0...v0.4.0
[v0.3.0]: https://github.com/rust-embedded/riscv/compare/v0.2.3...v0.3.0
[v0.2.3]: https://github.com/riscv-rust/gd32vf103xx-hal/compare/v0.2.2...v0.2.3