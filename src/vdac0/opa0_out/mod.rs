#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::OPA0_OUT {
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
pub struct MAINOUTENR {
    bits: bool,
}
impl MAINOUTENR {
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
pub struct ALTOUTENR {
    bits: bool,
}
impl ALTOUTENR {
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
pub struct APORTOUTENR {
    bits: bool,
}
impl APORTOUTENR {
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
pub struct SHORTR {
    bits: bool,
}
impl SHORTR {
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
#[doc = "Possible values of the field `ALTOUTPADEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ALTOUTPADENR {
    #[doc = "Alternate Output 0"]
    OUT0,
    #[doc = "Alternate Output 1"]
    OUT1,
    #[doc = "Alternate Output 2"]
    OUT2,
    #[doc = "Alternate Output 3"]
    OUT3,
    #[doc = "Alternate Output 4"]
    OUT4,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl ALTOUTPADENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ALTOUTPADENR::OUT0 => 1,
            ALTOUTPADENR::OUT1 => 2,
            ALTOUTPADENR::OUT2 => 4,
            ALTOUTPADENR::OUT3 => 8,
            ALTOUTPADENR::OUT4 => 16,
            ALTOUTPADENR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ALTOUTPADENR {
        match value {
            1 => ALTOUTPADENR::OUT0,
            2 => ALTOUTPADENR::OUT1,
            4 => ALTOUTPADENR::OUT2,
            8 => ALTOUTPADENR::OUT3,
            16 => ALTOUTPADENR::OUT4,
            i => ALTOUTPADENR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `OUT0`"]
    #[inline]
    pub fn is_out0(&self) -> bool {
        *self == ALTOUTPADENR::OUT0
    }
    #[doc = "Checks if the value of the field is `OUT1`"]
    #[inline]
    pub fn is_out1(&self) -> bool {
        *self == ALTOUTPADENR::OUT1
    }
    #[doc = "Checks if the value of the field is `OUT2`"]
    #[inline]
    pub fn is_out2(&self) -> bool {
        *self == ALTOUTPADENR::OUT2
    }
    #[doc = "Checks if the value of the field is `OUT3`"]
    #[inline]
    pub fn is_out3(&self) -> bool {
        *self == ALTOUTPADENR::OUT3
    }
    #[doc = "Checks if the value of the field is `OUT4`"]
    #[inline]
    pub fn is_out4(&self) -> bool {
        *self == ALTOUTPADENR::OUT4
    }
}
#[doc = r" Value of the field"]
pub struct APORTOUTSELR {
    bits: u8,
}
impl APORTOUTSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _MAINOUTENW<'a> {
    w: &'a mut W,
}
impl<'a> _MAINOUTENW<'a> {
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
pub struct _ALTOUTENW<'a> {
    w: &'a mut W,
}
impl<'a> _ALTOUTENW<'a> {
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
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _APORTOUTENW<'a> {
    w: &'a mut W,
}
impl<'a> _APORTOUTENW<'a> {
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
pub struct _SHORTW<'a> {
    w: &'a mut W,
}
impl<'a> _SHORTW<'a> {
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
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ALTOUTPADEN`"]
pub enum ALTOUTPADENW {
    #[doc = "Alternate Output 0"]
    OUT0,
    #[doc = "Alternate Output 1"]
    OUT1,
    #[doc = "Alternate Output 2"]
    OUT2,
    #[doc = "Alternate Output 3"]
    OUT3,
    #[doc = "Alternate Output 4"]
    OUT4,
}
impl ALTOUTPADENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ALTOUTPADENW::OUT0 => 1,
            ALTOUTPADENW::OUT1 => 2,
            ALTOUTPADENW::OUT2 => 4,
            ALTOUTPADENW::OUT3 => 8,
            ALTOUTPADENW::OUT4 => 16,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ALTOUTPADENW<'a> {
    w: &'a mut W,
}
impl<'a> _ALTOUTPADENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ALTOUTPADENW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Alternate Output 0"]
    #[inline]
    pub fn out0(self) -> &'a mut W {
        self.variant(ALTOUTPADENW::OUT0)
    }
    #[doc = "Alternate Output 1"]
    #[inline]
    pub fn out1(self) -> &'a mut W {
        self.variant(ALTOUTPADENW::OUT1)
    }
    #[doc = "Alternate Output 2"]
    #[inline]
    pub fn out2(self) -> &'a mut W {
        self.variant(ALTOUTPADENW::OUT2)
    }
    #[doc = "Alternate Output 3"]
    #[inline]
    pub fn out3(self) -> &'a mut W {
        self.variant(ALTOUTPADENW::OUT3)
    }
    #[doc = "Alternate Output 4"]
    #[inline]
    pub fn out4(self) -> &'a mut W {
        self.variant(ALTOUTPADENW::OUT4)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _APORTOUTSELW<'a> {
    w: &'a mut W,
}
impl<'a> _APORTOUTSELW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
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
    #[doc = "Bit 0 - OPAx Main Output Enable"]
    #[inline]
    pub fn mainouten(&self) -> MAINOUTENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        MAINOUTENR { bits }
    }
    #[doc = "Bit 1 - OPAx Alternative Output Enable"]
    #[inline]
    pub fn altouten(&self) -> ALTOUTENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ALTOUTENR { bits }
    }
    #[doc = "Bit 2 - OPAx Aport Output Enable"]
    #[inline]
    pub fn aportouten(&self) -> APORTOUTENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        APORTOUTENR { bits }
    }
    #[doc = "Bit 3 - OPAx Main and Alternative Output Short"]
    #[inline]
    pub fn short(&self) -> SHORTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SHORTR { bits }
    }
    #[doc = "Bits 4:8 - OPAx Output Enable Value"]
    #[inline]
    pub fn altoutpaden(&self) -> ALTOUTPADENR {
        ALTOUTPADENR::_from({
            const MASK: u8 = 31;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:23 - OPAx APORT Output"]
    #[inline]
    pub fn aportoutsel(&self) -> APORTOUTSELR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        APORTOUTSELR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 1 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - OPAx Main Output Enable"]
    #[inline]
    pub fn mainouten(&mut self) -> _MAINOUTENW {
        _MAINOUTENW { w: self }
    }
    #[doc = "Bit 1 - OPAx Alternative Output Enable"]
    #[inline]
    pub fn altouten(&mut self) -> _ALTOUTENW {
        _ALTOUTENW { w: self }
    }
    #[doc = "Bit 2 - OPAx Aport Output Enable"]
    #[inline]
    pub fn aportouten(&mut self) -> _APORTOUTENW {
        _APORTOUTENW { w: self }
    }
    #[doc = "Bit 3 - OPAx Main and Alternative Output Short"]
    #[inline]
    pub fn short(&mut self) -> _SHORTW {
        _SHORTW { w: self }
    }
    #[doc = "Bits 4:8 - OPAx Output Enable Value"]
    #[inline]
    pub fn altoutpaden(&mut self) -> _ALTOUTPADENW {
        _ALTOUTPADENW { w: self }
    }
    #[doc = "Bits 16:23 - OPAx APORT Output"]
    #[inline]
    pub fn aportoutsel(&mut self) -> _APORTOUTSELW {
        _APORTOUTSELW { w: self }
    }
}
