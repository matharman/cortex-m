//! Process Stack Pointer

/// Reads the CPU register
#[inline]
pub fn read() -> u32 {
    call_asm!(__psp_r() -> u32)
}

/// Writes `bits` to the CPU register
#[inline]
pub unsafe fn write(bits: u32) {
    call_asm!(__psp_w(bits: u32))
}

/// Reads the CPU register
#[cfg(armv8m)]
#[inline]
pub fn read_ns() -> u32 {
    call_asm!(__psp_ns_r() -> u32)
}

/// Writes `bits` to the CPU register
#[cfg(armv8m)]
#[inline]
pub unsafe fn write_ns(bits: u32) {
    call_asm!(__psp_ns_w(bits: u32))
}
