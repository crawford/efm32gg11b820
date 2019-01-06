#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CTRL {
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
pub struct ITAPDLYENR {
    bits: bool,
}
impl ITAPDLYENR {
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
pub struct ITAPDLYSELR {
    bits: u8,
}
impl ITAPDLYSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct ITAPCHGWINR {
    bits: bool,
}
impl ITAPCHGWINR {
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
pub struct OTAPDLYENR {
    bits: bool,
}
impl OTAPDLYENR {
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
pub struct OTAPDLYSELR {
    bits: u8,
}
impl OTAPDLYSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct TXDLYMUXSELR {
    bits: u8,
}
impl TXDLYMUXSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _ITAPDLYENW<'a> {
    w: &'a mut W,
}
impl<'a> _ITAPDLYENW<'a> {
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
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ITAPDLYSELW<'a> {
    w: &'a mut W,
}
impl<'a> _ITAPDLYSELW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ITAPCHGWINW<'a> {
    w: &'a mut W,
}
impl<'a> _ITAPCHGWINW<'a> {
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
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _OTAPDLYENW<'a> {
    w: &'a mut W,
}
impl<'a> _OTAPDLYENW<'a> {
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
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _OTAPDLYSELW<'a> {
    w: &'a mut W,
}
impl<'a> _OTAPDLYSELW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TXDLYMUXSELW<'a> {
    w: &'a mut W,
}
impl<'a> _TXDLYMUXSELW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 16;
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
    #[doc = "Bit 0 - Selective Tap Delay Line Enable on Rxclk_in"]
    #[inline]
    pub fn itapdlyen(&self) -> ITAPDLYENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ITAPDLYENR { bits }
    }
    #[doc = "Bits 1:5 - Selects One of 32 Taps on the Rxclk_in Line"]
    #[inline]
    pub fn itapdlysel(&self) -> ITAPDLYSELR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        ITAPDLYSELR { bits }
    }
    #[doc = "Bit 6 - Gating Signal for Tap Delay Change"]
    #[inline]
    pub fn itapchgwin(&self) -> ITAPCHGWINR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ITAPCHGWINR { bits }
    }
    #[doc = "Bit 7 - Selective Tap Delay Line Enable on SDIO_CLK Pin"]
    #[inline]
    pub fn otapdlyen(&self) -> OTAPDLYENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        OTAPDLYENR { bits }
    }
    #[doc = "Bits 8:11 - Selects One of 32 Taps on the SDIO_CLK Pin"]
    #[inline]
    pub fn otapdlysel(&self) -> OTAPDLYSELR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        OTAPDLYSELR { bits }
    }
    #[doc = "Bits 16:17 - TX Delay Mux Selection"]
    #[inline]
    pub fn txdlymuxsel(&self) -> TXDLYMUXSELR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TXDLYMUXSELR { bits }
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
    #[doc = "Bit 0 - Selective Tap Delay Line Enable on Rxclk_in"]
    #[inline]
    pub fn itapdlyen(&mut self) -> _ITAPDLYENW {
        _ITAPDLYENW { w: self }
    }
    #[doc = "Bits 1:5 - Selects One of 32 Taps on the Rxclk_in Line"]
    #[inline]
    pub fn itapdlysel(&mut self) -> _ITAPDLYSELW {
        _ITAPDLYSELW { w: self }
    }
    #[doc = "Bit 6 - Gating Signal for Tap Delay Change"]
    #[inline]
    pub fn itapchgwin(&mut self) -> _ITAPCHGWINW {
        _ITAPCHGWINW { w: self }
    }
    #[doc = "Bit 7 - Selective Tap Delay Line Enable on SDIO_CLK Pin"]
    #[inline]
    pub fn otapdlyen(&mut self) -> _OTAPDLYENW {
        _OTAPDLYENW { w: self }
    }
    #[doc = "Bits 8:11 - Selects One of 32 Taps on the SDIO_CLK Pin"]
    #[inline]
    pub fn otapdlysel(&mut self) -> _OTAPDLYSELW {
        _OTAPDLYSELW { w: self }
    }
    #[doc = "Bits 16:17 - TX Delay Mux Selection"]
    #[inline]
    pub fn txdlymuxsel(&mut self) -> _TXDLYMUXSELW {
        _TXDLYMUXSELW { w: self }
    }
}
