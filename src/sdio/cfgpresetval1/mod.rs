#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CFGPRESETVAL1 {
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
pub struct HSPSDCLKFREQR {
    bits: u16,
}
impl HSPSDCLKFREQR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct HSPCLKGENENR {
    bits: bool,
}
impl HSPCLKGENENR {
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
pub struct HSPDRVSTR {
    bits: u8,
}
impl HSPDRVSTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct SDR12SDCLKFREQR {
    bits: u16,
}
impl SDR12SDCLKFREQR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct SDR12CLKGENENR {
    bits: bool,
}
impl SDR12CLKGENENR {
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
pub struct SDR12DRVSTR {
    bits: u8,
}
impl SDR12DRVSTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _HSPSDCLKFREQW<'a> {
    w: &'a mut W,
}
impl<'a> _HSPSDCLKFREQW<'a> {
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
pub struct _HSPCLKGENENW<'a> {
    w: &'a mut W,
}
impl<'a> _HSPCLKGENENW<'a> {
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
pub struct _HSPDRVSTW<'a> {
    w: &'a mut W,
}
impl<'a> _HSPDRVSTW<'a> {
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
pub struct _SDR12SDCLKFREQW<'a> {
    w: &'a mut W,
}
impl<'a> _SDR12SDCLKFREQW<'a> {
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
pub struct _SDR12CLKGENENW<'a> {
    w: &'a mut W,
}
impl<'a> _SDR12CLKGENENW<'a> {
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
pub struct _SDR12DRVSTW<'a> {
    w: &'a mut W,
}
impl<'a> _SDR12DRVSTW<'a> {
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
    #[doc = "Bits 0:9 - High Speed SD_CLK Frequency"]
    #[inline]
    pub fn hspsdclkfreq(&self) -> HSPSDCLKFREQR {
        let bits = {
            const MASK: u16 = 1023;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        HSPSDCLKFREQR { bits }
    }
    #[doc = "Bit 10 - High Speed SD_CLK Gen Enable"]
    #[inline]
    pub fn hspclkgenen(&self) -> HSPCLKGENENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        HSPCLKGENENR { bits }
    }
    #[doc = "Bits 11:12 - High Speed SD Drive Strength"]
    #[inline]
    pub fn hspdrvst(&self) -> HSPDRVSTR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        HSPDRVSTR { bits }
    }
    #[doc = "Bits 16:25 - Preset Value for SDR12 Speed of SD_CLK"]
    #[inline]
    pub fn sdr12sdclkfreq(&self) -> SDR12SDCLKFREQR {
        let bits = {
            const MASK: u16 = 1023;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        SDR12SDCLKFREQR { bits }
    }
    #[doc = "Bit 26 - SDR12 Speed Clock Gen Enable"]
    #[inline]
    pub fn sdr12clkgenen(&self) -> SDR12CLKGENENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SDR12CLKGENENR { bits }
    }
    #[doc = "Bits 27:28 - SDR12 Speed Drive Strength"]
    #[inline]
    pub fn sdr12drvst(&self) -> SDR12DRVSTR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SDR12DRVSTR { bits }
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
    #[doc = "Bits 0:9 - High Speed SD_CLK Frequency"]
    #[inline]
    pub fn hspsdclkfreq(&mut self) -> _HSPSDCLKFREQW {
        _HSPSDCLKFREQW { w: self }
    }
    #[doc = "Bit 10 - High Speed SD_CLK Gen Enable"]
    #[inline]
    pub fn hspclkgenen(&mut self) -> _HSPCLKGENENW {
        _HSPCLKGENENW { w: self }
    }
    #[doc = "Bits 11:12 - High Speed SD Drive Strength"]
    #[inline]
    pub fn hspdrvst(&mut self) -> _HSPDRVSTW {
        _HSPDRVSTW { w: self }
    }
    #[doc = "Bits 16:25 - Preset Value for SDR12 Speed of SD_CLK"]
    #[inline]
    pub fn sdr12sdclkfreq(&mut self) -> _SDR12SDCLKFREQW {
        _SDR12SDCLKFREQW { w: self }
    }
    #[doc = "Bit 26 - SDR12 Speed Clock Gen Enable"]
    #[inline]
    pub fn sdr12clkgenen(&mut self) -> _SDR12CLKGENENW {
        _SDR12CLKGENENW { w: self }
    }
    #[doc = "Bits 27:28 - SDR12 Speed Drive Strength"]
    #[inline]
    pub fn sdr12drvst(&mut self) -> _SDR12DRVSTW {
        _SDR12DRVSTW { w: self }
    }
}
