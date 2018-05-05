#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CFG0 {
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
pub struct TUNINGCNTR {
    bits: u8,
}
impl TUNINGCNTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct TOUTCLKFREQR {
    bits: u8,
}
impl TOUTCLKFREQR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct TOUTCLKUNITR {
    bits: bool,
}
impl TOUTCLKUNITR {
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
pub struct BASECLKFREQR {
    bits: u8,
}
impl BASECLKFREQR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `MAXBLKLEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MAXBLKLENR {
    #[doc = "512 Bytes are Selected"]
    _512B,
    #[doc = "1024 Bytes are Selected"]
    _1024B,
    #[doc = "2048 Bytes are Selected"]
    _2048B,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl MAXBLKLENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MAXBLKLENR::_512B => 0,
            MAXBLKLENR::_1024B => 1,
            MAXBLKLENR::_2048B => 2,
            MAXBLKLENR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MAXBLKLENR {
        match value {
            0 => MAXBLKLENR::_512B,
            1 => MAXBLKLENR::_1024B,
            2 => MAXBLKLENR::_2048B,
            i => MAXBLKLENR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_512B`"]
    #[inline]
    pub fn is_512b(&self) -> bool {
        *self == MAXBLKLENR::_512B
    }
    #[doc = "Checks if the value of the field is `_1024B`"]
    #[inline]
    pub fn is_1024b(&self) -> bool {
        *self == MAXBLKLENR::_1024B
    }
    #[doc = "Checks if the value of the field is `_2048B`"]
    #[inline]
    pub fn is_2048b(&self) -> bool {
        *self == MAXBLKLENR::_2048B
    }
}
#[doc = r" Value of the field"]
pub struct C8BITSUPR {
    bits: bool,
}
impl C8BITSUPR {
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
pub struct CADMA2SUPR {
    bits: bool,
}
impl CADMA2SUPR {
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
pub struct CHSSUPR {
    bits: bool,
}
impl CHSSUPR {
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
pub struct CSDMASUPR {
    bits: bool,
}
impl CSDMASUPR {
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
pub struct CSUSPRESSUPR {
    bits: bool,
}
impl CSUSPRESSUPR {
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
pub struct C3P3VSUPR {
    bits: bool,
}
impl C3P3VSUPR {
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
pub struct C3P0VSUPR {
    bits: bool,
}
impl C3P0VSUPR {
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
pub struct C1P8VSUPR {
    bits: bool,
}
impl C1P8VSUPR {
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
pub struct _TUNINGCNTW<'a> {
    w: &'a mut W,
}
impl<'a> _TUNINGCNTW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TOUTCLKFREQW<'a> {
    w: &'a mut W,
}
impl<'a> _TOUTCLKFREQW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TOUTCLKUNITW<'a> {
    w: &'a mut W,
}
impl<'a> _TOUTCLKUNITW<'a> {
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
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _BASECLKFREQW<'a> {
    w: &'a mut W,
}
impl<'a> _BASECLKFREQW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 13;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MAXBLKLEN`"]
pub enum MAXBLKLENW {
    #[doc = "512 Bytes are Selected"]
    _512B,
    #[doc = "1024 Bytes are Selected"]
    _1024B,
    #[doc = "2048 Bytes are Selected"]
    _2048B,
}
impl MAXBLKLENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            MAXBLKLENW::_512B => 0,
            MAXBLKLENW::_1024B => 1,
            MAXBLKLENW::_2048B => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MAXBLKLENW<'a> {
    w: &'a mut W,
}
impl<'a> _MAXBLKLENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MAXBLKLENW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "512 Bytes are Selected"]
    #[inline]
    pub fn _512b(self) -> &'a mut W {
        self.variant(MAXBLKLENW::_512B)
    }
    #[doc = "1024 Bytes are Selected"]
    #[inline]
    pub fn _1024b(self) -> &'a mut W {
        self.variant(MAXBLKLENW::_1024B)
    }
    #[doc = "2048 Bytes are Selected"]
    #[inline]
    pub fn _2048b(self) -> &'a mut W {
        self.variant(MAXBLKLENW::_2048B)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 21;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _C8BITSUPW<'a> {
    w: &'a mut W,
}
impl<'a> _C8BITSUPW<'a> {
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
        const OFFSET: u8 = 23;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CADMA2SUPW<'a> {
    w: &'a mut W,
}
impl<'a> _CADMA2SUPW<'a> {
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
pub struct _CHSSUPW<'a> {
    w: &'a mut W,
}
impl<'a> _CHSSUPW<'a> {
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
pub struct _CSDMASUPW<'a> {
    w: &'a mut W,
}
impl<'a> _CSDMASUPW<'a> {
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
pub struct _CSUSPRESSUPW<'a> {
    w: &'a mut W,
}
impl<'a> _CSUSPRESSUPW<'a> {
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
        const OFFSET: u8 = 27;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _C3P3VSUPW<'a> {
    w: &'a mut W,
}
impl<'a> _C3P3VSUPW<'a> {
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
        const OFFSET: u8 = 28;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _C3P0VSUPW<'a> {
    w: &'a mut W,
}
impl<'a> _C3P0VSUPW<'a> {
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
        const OFFSET: u8 = 29;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _C1P8VSUPW<'a> {
    w: &'a mut W,
}
impl<'a> _C1P8VSUPW<'a> {
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
        const OFFSET: u8 = 30;
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
    #[doc = "Bits 0:5 - Tuning Counter Value"]
    #[inline]
    pub fn tuningcnt(&self) -> TUNINGCNTR {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TUNINGCNTR { bits }
    }
    #[doc = "Bits 6:11 - Timeout Clock Frequency"]
    #[inline]
    pub fn toutclkfreq(&self) -> TOUTCLKFREQR {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TOUTCLKFREQR { bits }
    }
    #[doc = "Bit 12 - Timeout Clock Unit in kHz or MHz"]
    #[inline]
    pub fn toutclkunit(&self) -> TOUTCLKUNITR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TOUTCLKUNITR { bits }
    }
    #[doc = "Bits 13:20 - Base Clock Frequency for SD_CLK"]
    #[inline]
    pub fn baseclkfreq(&self) -> BASECLKFREQR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        BASECLKFREQR { bits }
    }
    #[doc = "Bits 21:22 - MAX Block Length of Transfer"]
    #[inline]
    pub fn maxblklen(&self) -> MAXBLKLENR {
        MAXBLKLENR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 23 - 8-bit Interface Support"]
    #[inline]
    pub fn c8bitsup(&self) -> C8BITSUPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        C8BITSUPR { bits }
    }
    #[doc = "Bit 24 - ADMA2 Mode Support"]
    #[inline]
    pub fn cadma2sup(&self) -> CADMA2SUPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CADMA2SUPR { bits }
    }
    #[doc = "Bit 25 - High Speed Mode Support"]
    #[inline]
    pub fn chssup(&self) -> CHSSUPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CHSSUPR { bits }
    }
    #[doc = "Bit 26 - SDMA Mode Support"]
    #[inline]
    pub fn csdmasup(&self) -> CSDMASUPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CSDMASUPR { bits }
    }
    #[doc = "Bit 27 - Suspend/Resume Support"]
    #[inline]
    pub fn csuspressup(&self) -> CSUSPRESSUPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CSUSPRESSUPR { bits }
    }
    #[doc = "Bit 28 - Core 3P3V Support"]
    #[inline]
    pub fn c3p3vsup(&self) -> C3P3VSUPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        C3P3VSUPR { bits }
    }
    #[doc = "Bit 29 - 3P0V Support"]
    #[inline]
    pub fn c3p0vsup(&self) -> C3P0VSUPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        C3P0VSUPR { bits }
    }
    #[doc = "Bit 30 - 1P8V Support"]
    #[inline]
    pub fn c1p8vsup(&self) -> C1P8VSUPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        C1P8VSUPR { bits }
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
    #[doc = "Bits 0:5 - Tuning Counter Value"]
    #[inline]
    pub fn tuningcnt(&mut self) -> _TUNINGCNTW {
        _TUNINGCNTW { w: self }
    }
    #[doc = "Bits 6:11 - Timeout Clock Frequency"]
    #[inline]
    pub fn toutclkfreq(&mut self) -> _TOUTCLKFREQW {
        _TOUTCLKFREQW { w: self }
    }
    #[doc = "Bit 12 - Timeout Clock Unit in kHz or MHz"]
    #[inline]
    pub fn toutclkunit(&mut self) -> _TOUTCLKUNITW {
        _TOUTCLKUNITW { w: self }
    }
    #[doc = "Bits 13:20 - Base Clock Frequency for SD_CLK"]
    #[inline]
    pub fn baseclkfreq(&mut self) -> _BASECLKFREQW {
        _BASECLKFREQW { w: self }
    }
    #[doc = "Bits 21:22 - MAX Block Length of Transfer"]
    #[inline]
    pub fn maxblklen(&mut self) -> _MAXBLKLENW {
        _MAXBLKLENW { w: self }
    }
    #[doc = "Bit 23 - 8-bit Interface Support"]
    #[inline]
    pub fn c8bitsup(&mut self) -> _C8BITSUPW {
        _C8BITSUPW { w: self }
    }
    #[doc = "Bit 24 - ADMA2 Mode Support"]
    #[inline]
    pub fn cadma2sup(&mut self) -> _CADMA2SUPW {
        _CADMA2SUPW { w: self }
    }
    #[doc = "Bit 25 - High Speed Mode Support"]
    #[inline]
    pub fn chssup(&mut self) -> _CHSSUPW {
        _CHSSUPW { w: self }
    }
    #[doc = "Bit 26 - SDMA Mode Support"]
    #[inline]
    pub fn csdmasup(&mut self) -> _CSDMASUPW {
        _CSDMASUPW { w: self }
    }
    #[doc = "Bit 27 - Suspend/Resume Support"]
    #[inline]
    pub fn csuspressup(&mut self) -> _CSUSPRESSUPW {
        _CSUSPRESSUPW { w: self }
    }
    #[doc = "Bit 28 - Core 3P3V Support"]
    #[inline]
    pub fn c3p3vsup(&mut self) -> _C3P3VSUPW {
        _C3P3VSUPW { w: self }
    }
    #[doc = "Bit 29 - 3P0V Support"]
    #[inline]
    pub fn c3p0vsup(&mut self) -> _C3P0VSUPW {
        _C3P0VSUPW { w: self }
    }
    #[doc = "Bit 30 - 1P8V Support"]
    #[inline]
    pub fn c1p8vsup(&mut self) -> _C1P8VSUPW {
        _C1P8VSUPW { w: self }
    }
}
