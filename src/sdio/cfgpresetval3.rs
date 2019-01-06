#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CFGPRESETVAL3 {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = r" Value of the field"]
pub struct SDR104SDCLKFREQR {
    bits: u16,
}
impl SDR104SDCLKFREQR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct SDR104CLKGENENR {
    bits: bool,
}
impl SDR104CLKGENENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct SDR104DRVSTR {
    bits: u8,
}
impl SDR104DRVSTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DDR50SDCLKFREQR {
    bits: u16,
}
impl DDR50SDCLKFREQR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DDR50CLKGENENR {
    bits: bool,
}
impl DDR50CLKGENENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct DDR50DRVSTR {
    bits: u8,
}
impl DDR50DRVSTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _SDR104SDCLKFREQW<'a> {
    w: &'a mut W,
}
impl<'a> _SDR104SDCLKFREQW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 1023;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SDR104CLKGENENW<'a> {
    w: &'a mut W,
}
impl<'a> _SDR104CLKGENENW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SDR104DRVSTW<'a> {
    w: &'a mut W,
}
impl<'a> _SDR104DRVSTW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DDR50SDCLKFREQW<'a> {
    w: &'a mut W,
}
impl<'a> _DDR50SDCLKFREQW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 1023;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DDR50CLKGENENW<'a> {
    w: &'a mut W,
}
impl<'a> _DDR50CLKGENENW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 26;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DDR50DRVSTW<'a> {
    w: &'a mut W,
}
impl<'a> _DDR50DRVSTW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 27;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:9 - SDR104 SD_CLK Frequency"]
    #[inline]
    pub fn sdr104sdclkfreq(&self) -> SDR104SDCLKFREQR {
        let bits = {
            const MASK: u16 = 1023;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        SDR104SDCLKFREQR { bits }
    }
    #[doc = "Bit 10 - SDR104 SD_CLK Gen Enable"]
    #[inline]
    pub fn sdr104clkgenen(&self) -> SDR104CLKGENENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SDR104CLKGENENR { bits }
    }
    #[doc = "Bits 11:12 - SDR104 SD Drive Strength"]
    #[inline]
    pub fn sdr104drvst(&self) -> SDR104DRVSTR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SDR104DRVSTR { bits }
    }
    #[doc = "Bits 16:25 - Preset Value for DDR50 Speed of SD_CLK"]
    #[inline]
    pub fn ddr50sdclkfreq(&self) -> DDR50SDCLKFREQR {
        let bits = {
            const MASK: u16 = 1023;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        DDR50SDCLKFREQR { bits }
    }
    #[doc = "Bit 26 - DDR50 Speed Clock Gen Enable"]
    #[inline]
    pub fn ddr50clkgenen(&self) -> DDR50CLKGENENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DDR50CLKGENENR { bits }
    }
    #[doc = "Bits 27:28 - DDR50 Speed Drive Strength"]
    #[inline]
    pub fn ddr50drvst(&self) -> DDR50DRVSTR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DDR50DRVSTR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:9 - SDR104 SD_CLK Frequency"]
    #[inline]
    pub fn sdr104sdclkfreq(&mut self) -> _SDR104SDCLKFREQW {
        _SDR104SDCLKFREQW { w: self }
    }
    #[doc = "Bit 10 - SDR104 SD_CLK Gen Enable"]
    #[inline]
    pub fn sdr104clkgenen(&mut self) -> _SDR104CLKGENENW {
        _SDR104CLKGENENW { w: self }
    }
    #[doc = "Bits 11:12 - SDR104 SD Drive Strength"]
    #[inline]
    pub fn sdr104drvst(&mut self) -> _SDR104DRVSTW {
        _SDR104DRVSTW { w: self }
    }
    #[doc = "Bits 16:25 - Preset Value for DDR50 Speed of SD_CLK"]
    #[inline]
    pub fn ddr50sdclkfreq(&mut self) -> _DDR50SDCLKFREQW {
        _DDR50SDCLKFREQW { w: self }
    }
    #[doc = "Bit 26 - DDR50 Speed Clock Gen Enable"]
    #[inline]
    pub fn ddr50clkgenen(&mut self) -> _DDR50CLKGENENW {
        _DDR50CLKGENENW { w: self }
    }
    #[doc = "Bits 27:28 - DDR50 Speed Drive Strength"]
    #[inline]
    pub fn ddr50drvst(&mut self) -> _DDR50DRVSTW {
        _DDR50DRVSTW { w: self }
    }
}
