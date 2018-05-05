#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CFGPRESETVAL0 {
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
        R {
            bits: self.register.get(),
        }
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
pub struct INITSDCLKFREQR {
    bits: u16,
}
impl INITSDCLKFREQR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct INITCLKGENENR {
    bits: bool,
}
impl INITCLKGENENR {
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
pub struct INITDRVSTR {
    bits: u8,
}
impl INITDRVSTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DSPSDCLKFREQR {
    bits: u16,
}
impl DSPSDCLKFREQR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DSPCLKGENENR {
    bits: bool,
}
impl DSPCLKGENENR {
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
pub struct DSPDRVSTR {
    bits: u8,
}
impl DSPDRVSTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _INITSDCLKFREQW<'a> {
    w: &'a mut W,
}
impl<'a> _INITSDCLKFREQW<'a> {
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
pub struct _INITCLKGENENW<'a> {
    w: &'a mut W,
}
impl<'a> _INITCLKGENENW<'a> {
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
pub struct _INITDRVSTW<'a> {
    w: &'a mut W,
}
impl<'a> _INITDRVSTW<'a> {
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
pub struct _DSPSDCLKFREQW<'a> {
    w: &'a mut W,
}
impl<'a> _DSPSDCLKFREQW<'a> {
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
pub struct _DSPCLKGENENW<'a> {
    w: &'a mut W,
}
impl<'a> _DSPCLKGENENW<'a> {
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
pub struct _DSPDRVSTW<'a> {
    w: &'a mut W,
}
impl<'a> _DSPDRVSTW<'a> {
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
    #[doc = "Bits 0:9 - Initial SD_CLK Frequency"]
    #[inline]
    pub fn initsdclkfreq(&self) -> INITSDCLKFREQR {
        let bits = {
            const MASK: u16 = 1023;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        INITSDCLKFREQR { bits }
    }
    #[doc = "Bit 10 - Initial Clock Gen Enable"]
    #[inline]
    pub fn initclkgenen(&self) -> INITCLKGENENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        INITCLKGENENR { bits }
    }
    #[doc = "Bits 11:12 - Initial Drive Strength"]
    #[inline]
    pub fn initdrvst(&self) -> INITDRVSTR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        INITDRVSTR { bits }
    }
    #[doc = "Bits 16:25 - Preset Value for Default Speed of SD_CLK"]
    #[inline]
    pub fn dspsdclkfreq(&self) -> DSPSDCLKFREQR {
        let bits = {
            const MASK: u16 = 1023;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        DSPSDCLKFREQR { bits }
    }
    #[doc = "Bit 26 - Default Speed Clock Gen Enable"]
    #[inline]
    pub fn dspclkgenen(&self) -> DSPCLKGENENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DSPCLKGENENR { bits }
    }
    #[doc = "Bits 27:28 - Default Speed Drive Strength"]
    #[inline]
    pub fn dspdrvst(&self) -> DSPDRVSTR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DSPDRVSTR { bits }
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
    #[doc = "Bits 0:9 - Initial SD_CLK Frequency"]
    #[inline]
    pub fn initsdclkfreq(&mut self) -> _INITSDCLKFREQW {
        _INITSDCLKFREQW { w: self }
    }
    #[doc = "Bit 10 - Initial Clock Gen Enable"]
    #[inline]
    pub fn initclkgenen(&mut self) -> _INITCLKGENENW {
        _INITCLKGENENW { w: self }
    }
    #[doc = "Bits 11:12 - Initial Drive Strength"]
    #[inline]
    pub fn initdrvst(&mut self) -> _INITDRVSTW {
        _INITDRVSTW { w: self }
    }
    #[doc = "Bits 16:25 - Preset Value for Default Speed of SD_CLK"]
    #[inline]
    pub fn dspsdclkfreq(&mut self) -> _DSPSDCLKFREQW {
        _DSPSDCLKFREQW { w: self }
    }
    #[doc = "Bit 26 - Default Speed Clock Gen Enable"]
    #[inline]
    pub fn dspclkgenen(&mut self) -> _DSPCLKGENENW {
        _DSPCLKGENENW { w: self }
    }
    #[doc = "Bits 27:28 - Default Speed Drive Strength"]
    #[inline]
    pub fn dspdrvst(&mut self) -> _DSPDRVSTW {
        _DSPDRVSTW { w: self }
    }
}
