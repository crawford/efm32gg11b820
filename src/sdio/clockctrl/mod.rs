#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CLOCKCTRL {
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
pub struct INTCLKENR {
    bits: bool,
}
impl INTCLKENR {
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
pub struct INTCLKSTABLER {
    bits: bool,
}
impl INTCLKSTABLER {
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
pub struct SDCLKENR {
    bits: bool,
}
impl SDCLKENR {
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
pub struct CLKGENSELR {
    bits: bool,
}
impl CLKGENSELR {
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
pub struct UPPSDCLKFRER {
    bits: u8,
}
impl UPPSDCLKFRER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `SDCLKFREQSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SDCLKFREQSELR {
    #[doc = "\"\""]
    NODIVISION,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SDCLKFREQSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SDCLKFREQSELR::NODIVISION => 0,
            SDCLKFREQSELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SDCLKFREQSELR {
        match value {
            0 => SDCLKFREQSELR::NODIVISION,
            i => SDCLKFREQSELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `NODIVISION`"]
    #[inline]
    pub fn is_nodivision(&self) -> bool {
        *self == SDCLKFREQSELR::NODIVISION
    }
}
#[doc = r" Value of the field"]
pub struct DATTOUTCNTVALR {
    bits: u8,
}
impl DATTOUTCNTVALR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct SFTRSTAR {
    bits: bool,
}
impl SFTRSTAR {
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
pub struct SFTRSTCMDR {
    bits: bool,
}
impl SFTRSTCMDR {
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
pub struct SFTRSTDATR {
    bits: bool,
}
impl SFTRSTDATR {
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
#[doc = r" Proxy"]
pub struct _INTCLKENW<'a> {
    w: &'a mut W,
}
impl<'a> _INTCLKENW<'a> {
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
pub struct _SDCLKENW<'a> {
    w: &'a mut W,
}
impl<'a> _SDCLKENW<'a> {
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
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CLKGENSELW<'a> {
    w: &'a mut W,
}
impl<'a> _CLKGENSELW<'a> {
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
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _UPPSDCLKFREW<'a> {
    w: &'a mut W,
}
impl<'a> _UPPSDCLKFREW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SDCLKFREQSEL`"]
pub enum SDCLKFREQSELW {
    #[doc = "\"\""]
    NODIVISION,
}
impl SDCLKFREQSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SDCLKFREQSELW::NODIVISION => 0,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SDCLKFREQSELW<'a> {
    w: &'a mut W,
}
impl<'a> _SDCLKFREQSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SDCLKFREQSELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "\"\""]
    #[inline]
    pub fn nodivision(self) -> &'a mut W {
        self.variant(SDCLKFREQSELW::NODIVISION)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DATTOUTCNTVALW<'a> {
    w: &'a mut W,
}
impl<'a> _DATTOUTCNTVALW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SFTRSTAW<'a> {
    w: &'a mut W,
}
impl<'a> _SFTRSTAW<'a> {
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
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SFTRSTCMDW<'a> {
    w: &'a mut W,
}
impl<'a> _SFTRSTCMDW<'a> {
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
        const OFFSET: u8 = 25;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SFTRSTDATW<'a> {
    w: &'a mut W,
}
impl<'a> _SFTRSTDATW<'a> {
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Internal Clock Enable"]
    #[inline]
    pub fn intclken(&self) -> INTCLKENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        INTCLKENR { bits }
    }
    #[doc = "Bit 1 - Internal Clock Stable"]
    #[inline]
    pub fn intclkstable(&self) -> INTCLKSTABLER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        INTCLKSTABLER { bits }
    }
    #[doc = "Bit 2 - SDIO_CLK Pin Clock Enable"]
    #[inline]
    pub fn sdclken(&self) -> SDCLKENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SDCLKENR { bits }
    }
    #[doc = "Bit 5 - Clock Generator Select"]
    #[inline]
    pub fn clkgensel(&self) -> CLKGENSELR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CLKGENSELR { bits }
    }
    #[doc = "Bits 6:7 - Upper Bits of SD_CLK Frequency Select"]
    #[inline]
    pub fn uppsdclkfre(&self) -> UPPSDCLKFRER {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        UPPSDCLKFRER { bits }
    }
    #[doc = "Bits 8:15 - SD_CLK Frequency Select"]
    #[inline]
    pub fn sdclkfreqsel(&self) -> SDCLKFREQSELR {
        SDCLKFREQSELR::_from({
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:19 - Data Timeout Counter Value"]
    #[inline]
    pub fn dattoutcntval(&self) -> DATTOUTCNTVALR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DATTOUTCNTVALR { bits }
    }
    #[doc = "Bit 24 - Software Reset for All"]
    #[inline]
    pub fn sftrsta(&self) -> SFTRSTAR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SFTRSTAR { bits }
    }
    #[doc = "Bit 25 - Software Reset for CMD Line"]
    #[inline]
    pub fn sftrstcmd(&self) -> SFTRSTCMDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SFTRSTCMDR { bits }
    }
    #[doc = "Bit 26 - Software Reset for DAT Line"]
    #[inline]
    pub fn sftrstdat(&self) -> SFTRSTDATR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SFTRSTDATR { bits }
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
    #[doc = "Bit 0 - Internal Clock Enable"]
    #[inline]
    pub fn intclken(&mut self) -> _INTCLKENW {
        _INTCLKENW { w: self }
    }
    #[doc = "Bit 2 - SDIO_CLK Pin Clock Enable"]
    #[inline]
    pub fn sdclken(&mut self) -> _SDCLKENW {
        _SDCLKENW { w: self }
    }
    #[doc = "Bit 5 - Clock Generator Select"]
    #[inline]
    pub fn clkgensel(&mut self) -> _CLKGENSELW {
        _CLKGENSELW { w: self }
    }
    #[doc = "Bits 6:7 - Upper Bits of SD_CLK Frequency Select"]
    #[inline]
    pub fn uppsdclkfre(&mut self) -> _UPPSDCLKFREW {
        _UPPSDCLKFREW { w: self }
    }
    #[doc = "Bits 8:15 - SD_CLK Frequency Select"]
    #[inline]
    pub fn sdclkfreqsel(&mut self) -> _SDCLKFREQSELW {
        _SDCLKFREQSELW { w: self }
    }
    #[doc = "Bits 16:19 - Data Timeout Counter Value"]
    #[inline]
    pub fn dattoutcntval(&mut self) -> _DATTOUTCNTVALW {
        _DATTOUTCNTVALW { w: self }
    }
    #[doc = "Bit 24 - Software Reset for All"]
    #[inline]
    pub fn sftrsta(&mut self) -> _SFTRSTAW {
        _SFTRSTAW { w: self }
    }
    #[doc = "Bit 25 - Software Reset for CMD Line"]
    #[inline]
    pub fn sftrstcmd(&mut self) -> _SFTRSTCMDW {
        _SFTRSTCMDW { w: self }
    }
    #[doc = "Bit 26 - Software Reset for DAT Line"]
    #[inline]
    pub fn sftrstdat(&mut self) -> _SFTRSTDATW {
        _SFTRSTDATW { w: self }
    }
}
