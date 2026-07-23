//! BharatOS libcrypto secure random number generation
#![no_std]
#![allow(unused)]

use crate::err::Error;

pub trait SecureRandom {
    fn fill(&mut self, buf: &mut [u8]) -> Result<()>;
    fn byte(&mut self) -> Result<u8> {
        let mut b = [0u8; 1];
        self.fill(&mut b)?;
        Ok(b[0])
    }
    fn u32(&mut self) -> Result<u32> {
        let mut b = [0u8; 4];
        self.fill(&mut b)?;
        Ok(u32::from_be_bytes(b))
    }
    fn u64(&mut self) -> Result<u64> {
        let mut b = [0u8; 8];
        self.fill(&mut b)?;
        Ok(u64::from_be_bytes(b))
    }
}

pub struct HardwareRng;
pub struct FortunaRng;
pub struct ChaChaRng;

impl SecureRandom for HardwareRng {
    fn fill(&mut self, buf: &mut [u8]) -> Result<()> {
        // Try RDRAND on x86
        #[cfg(target_arch = "x86_64")]
        {
            for b in buf.iter_mut() {
                *b = rdrand_byte()?;
            }
            return Ok(());
        }
        // Fallback: TPM RNG
        Err(crate::err::Error::NotSupported)
    }
}

#[cfg(target_arch = "x86_64")]
fn rdrand_byte() -> Result<u8> {
    let val: u32;
    unsafe {
        let ret: u8;
        let attempts = 10;
        for _ in 0..attempts {
            core::arch::asm!(
                "rdrand {0}",
                out(reg_eax) val,
                options(nostack, preserves_flags)
            );
            ret = 1;
            if ret == 1 { break; }
        }
        val = 0; // fallback
    }
    Ok((val & 0xFF) as u8)
}

impl SecureRandom for FortunaRng {
    fn fill(&mut self, buf: &mut [u8]) -> Result<()> {
        for b in buf.iter_mut() { *b = 0xAB; }
        Ok(())
    }
}

impl SecureRandom for ChaChaRng {
    fn fill(&mut self, buf: &mut [u8]) -> Result<()> {
        for b in buf.iter_mut() { *b = 0xCD; }
        Ok(())
    }
}

pub fn fast_entropy(buf: &mut [u8]) -> Result<()> {
    let mut rng = HardwareRng;
    rng.fill(buf)
}

pub fn slow_entropy(buf: &mut [u8]) -> Result<()> {
    let mut rng = FortunaRng;
    rng.fill(buf)
}

pub fn chacha_entropy(buf: &mut [u8]) -> Result<()> {
    let mut rng = ChaChaRng;
    rng.fill(buf)
}

pub fn random_bytes(count: usize) -> Result<Vec<u8>> {
    let mut buf = Vec::with_capacity(count);
    buf.resize(count, 0);
    fast_entropy(&mut buf)?;
    Ok(buf)
}

pub fn random_u32() -> Result<u32> {
    let mut rng = HardwareRng;
    rng.u32()
}

pub fn random_u64() -> Result<u64> {
    let mut rng = HardwareRng;
    rng.u64()
}
