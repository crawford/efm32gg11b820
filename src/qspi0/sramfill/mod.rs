#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::SRAMFILL {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct SRAMFILLINDACREADR {
    bits: u16,
}
impl SRAMFILLINDACREADR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct SRAMFILLINDACWRITER {
    bits: u16,
}
impl SRAMFILLINDACWRITER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:15 - SRAM Fill Level (Indirect Read Partition)"]
    #[inline]
    pub fn sramfillindacread(&self) -> SRAMFILLINDACREADR {
        let bits = {
            const MASK: u16 = 65535;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        SRAMFILLINDACREADR { bits }
    }
    #[doc = "Bits 16:31 - SRAM Fill Level (Indirect Write Partition)"]
    #[inline]
    pub fn sramfillindacwrite(&self) -> SRAMFILLINDACWRITER {
        let bits = {
            const MASK: u16 = 65535;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        SRAMFILLINDACWRITER { bits }
    }
}
