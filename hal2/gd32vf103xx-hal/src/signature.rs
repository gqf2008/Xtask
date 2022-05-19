//! Device electronic signature
//!
//! Section 1.5, GD32VF103 User Manual

const FLASH_SIZE_PTR: *const u16 = 0x1FFF_F7E0 as *const _;
const SRAM_SIZE_PTR: *const u16 = 0x1FFF_F7E2 as *const _;
const DEVICE_ID_PTR: *const [u32; 3] = 0x1FFF_F7E8 as *const _;

/// Flash memory size in KBytes.
#[inline]
pub fn flash_size_kb() -> u16 {
    unsafe { *FLASH_SIZE_PTR }
}

/// On-chip SRAM size in KBytes.
#[inline]
pub fn sram_size_kb() -> u16 {
    unsafe { *SRAM_SIZE_PTR }
}

/// Factory programed unique device id.
#[inline]
pub fn device_id() -> &'static [u32; 3] {
    unsafe { &*DEVICE_ID_PTR }
}
