#[doc = "Reader of register CFG0"]
pub type R = crate::R<u32, super::CFG0>;
#[doc = "Writer for register CFG0"]
pub type W = crate::W<u32, super::CFG0>;
#[doc = "Register CFG0 `reset()`'s with value 0"]
impl crate::ResetValue for super::CFG0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TUNINGCNT`"]
pub type TUNINGCNT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TUNINGCNT`"]
pub struct TUNINGCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> TUNINGCNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
#[doc = "Reader of field `TOUTCLKFREQ`"]
pub type TOUTCLKFREQ_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TOUTCLKFREQ`"]
pub struct TOUTCLKFREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> TOUTCLKFREQ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 6)) | (((value as u32) & 0x3f) << 6);
        self.w
    }
}
#[doc = "Reader of field `TOUTCLKUNIT`"]
pub type TOUTCLKUNIT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TOUTCLKUNIT`"]
pub struct TOUTCLKUNIT_W<'a> {
    w: &'a mut W,
}
impl<'a> TOUTCLKUNIT_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `BASECLKFREQ`"]
pub type BASECLKFREQ_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `BASECLKFREQ`"]
pub struct BASECLKFREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> BASECLKFREQ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 13)) | (((value as u32) & 0xff) << 13);
        self.w
    }
}
#[doc = "MAX Block Length of Transfer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MAXBLKLEN_A {
    #[doc = "0: 512 Bytes are Selected"]
    _512B,
    #[doc = "1: 1024 Bytes are Selected"]
    _1024B,
    #[doc = "2: 2048 Bytes are Selected"]
    _2048B,
}
impl From<MAXBLKLEN_A> for u8 {
    #[inline(always)]
    fn from(variant: MAXBLKLEN_A) -> Self {
        match variant {
            MAXBLKLEN_A::_512B => 0,
            MAXBLKLEN_A::_1024B => 1,
            MAXBLKLEN_A::_2048B => 2,
        }
    }
}
#[doc = "Reader of field `MAXBLKLEN`"]
pub type MAXBLKLEN_R = crate::R<u8, MAXBLKLEN_A>;
impl MAXBLKLEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, MAXBLKLEN_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(MAXBLKLEN_A::_512B),
            1 => Val(MAXBLKLEN_A::_1024B),
            2 => Val(MAXBLKLEN_A::_2048B),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_512B`"]
    #[inline(always)]
    pub fn is_512b(&self) -> bool {
        *self == MAXBLKLEN_A::_512B
    }
    #[doc = "Checks if the value of the field is `_1024B`"]
    #[inline(always)]
    pub fn is_1024b(&self) -> bool {
        *self == MAXBLKLEN_A::_1024B
    }
    #[doc = "Checks if the value of the field is `_2048B`"]
    #[inline(always)]
    pub fn is_2048b(&self) -> bool {
        *self == MAXBLKLEN_A::_2048B
    }
}
#[doc = "Write proxy for field `MAXBLKLEN`"]
pub struct MAXBLKLEN_W<'a> {
    w: &'a mut W,
}
impl<'a> MAXBLKLEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MAXBLKLEN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "512 Bytes are Selected"]
    #[inline(always)]
    pub fn _512b(self) -> &'a mut W {
        self.variant(MAXBLKLEN_A::_512B)
    }
    #[doc = "1024 Bytes are Selected"]
    #[inline(always)]
    pub fn _1024b(self) -> &'a mut W {
        self.variant(MAXBLKLEN_A::_1024B)
    }
    #[doc = "2048 Bytes are Selected"]
    #[inline(always)]
    pub fn _2048b(self) -> &'a mut W {
        self.variant(MAXBLKLEN_A::_2048B)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 21)) | (((value as u32) & 0x03) << 21);
        self.w
    }
}
#[doc = "Reader of field `C8BITSUP`"]
pub type C8BITSUP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `C8BITSUP`"]
pub struct C8BITSUP_W<'a> {
    w: &'a mut W,
}
impl<'a> C8BITSUP_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "Reader of field `CADMA2SUP`"]
pub type CADMA2SUP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CADMA2SUP`"]
pub struct CADMA2SUP_W<'a> {
    w: &'a mut W,
}
impl<'a> CADMA2SUP_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Reader of field `CHSSUP`"]
pub type CHSSUP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CHSSUP`"]
pub struct CHSSUP_W<'a> {
    w: &'a mut W,
}
impl<'a> CHSSUP_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Reader of field `CSDMASUP`"]
pub type CSDMASUP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CSDMASUP`"]
pub struct CSDMASUP_W<'a> {
    w: &'a mut W,
}
impl<'a> CSDMASUP_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Reader of field `CSUSPRESSUP`"]
pub type CSUSPRESSUP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CSUSPRESSUP`"]
pub struct CSUSPRESSUP_W<'a> {
    w: &'a mut W,
}
impl<'a> CSUSPRESSUP_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = "Reader of field `C3P3VSUP`"]
pub type C3P3VSUP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `C3P3VSUP`"]
pub struct C3P3VSUP_W<'a> {
    w: &'a mut W,
}
impl<'a> C3P3VSUP_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Reader of field `C3P0VSUP`"]
pub type C3P0VSUP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `C3P0VSUP`"]
pub struct C3P0VSUP_W<'a> {
    w: &'a mut W,
}
impl<'a> C3P0VSUP_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "Reader of field `C1P8VSUP`"]
pub type C1P8VSUP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `C1P8VSUP`"]
pub struct C1P8VSUP_W<'a> {
    w: &'a mut W,
}
impl<'a> C1P8VSUP_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - Tuning Counter Value"]
    #[inline(always)]
    pub fn tuningcnt(&self) -> TUNINGCNT_R {
        TUNINGCNT_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:11 - Timeout Clock Frequency"]
    #[inline(always)]
    pub fn toutclkfreq(&self) -> TOUTCLKFREQ_R {
        TOUTCLKFREQ_R::new(((self.bits >> 6) & 0x3f) as u8)
    }
    #[doc = "Bit 12 - Timeout Clock Unit in kHz or MHz"]
    #[inline(always)]
    pub fn toutclkunit(&self) -> TOUTCLKUNIT_R {
        TOUTCLKUNIT_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 13:20 - Base Clock Frequency for SD_CLK"]
    #[inline(always)]
    pub fn baseclkfreq(&self) -> BASECLKFREQ_R {
        BASECLKFREQ_R::new(((self.bits >> 13) & 0xff) as u8)
    }
    #[doc = "Bits 21:22 - MAX Block Length of Transfer"]
    #[inline(always)]
    pub fn maxblklen(&self) -> MAXBLKLEN_R {
        MAXBLKLEN_R::new(((self.bits >> 21) & 0x03) as u8)
    }
    #[doc = "Bit 23 - 8-bit Interface Support"]
    #[inline(always)]
    pub fn c8bitsup(&self) -> C8BITSUP_R {
        C8BITSUP_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - ADMA2 Mode Support"]
    #[inline(always)]
    pub fn cadma2sup(&self) -> CADMA2SUP_R {
        CADMA2SUP_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - High Speed Mode Support"]
    #[inline(always)]
    pub fn chssup(&self) -> CHSSUP_R {
        CHSSUP_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - SDMA Mode Support"]
    #[inline(always)]
    pub fn csdmasup(&self) -> CSDMASUP_R {
        CSDMASUP_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Suspend/Resume Support"]
    #[inline(always)]
    pub fn csuspressup(&self) -> CSUSPRESSUP_R {
        CSUSPRESSUP_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Core 3P3V Support"]
    #[inline(always)]
    pub fn c3p3vsup(&self) -> C3P3VSUP_R {
        C3P3VSUP_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - 3P0V Support"]
    #[inline(always)]
    pub fn c3p0vsup(&self) -> C3P0VSUP_R {
        C3P0VSUP_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - 1P8V Support"]
    #[inline(always)]
    pub fn c1p8vsup(&self) -> C1P8VSUP_R {
        C1P8VSUP_R::new(((self.bits >> 30) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - Tuning Counter Value"]
    #[inline(always)]
    pub fn tuningcnt(&mut self) -> TUNINGCNT_W {
        TUNINGCNT_W { w: self }
    }
    #[doc = "Bits 6:11 - Timeout Clock Frequency"]
    #[inline(always)]
    pub fn toutclkfreq(&mut self) -> TOUTCLKFREQ_W {
        TOUTCLKFREQ_W { w: self }
    }
    #[doc = "Bit 12 - Timeout Clock Unit in kHz or MHz"]
    #[inline(always)]
    pub fn toutclkunit(&mut self) -> TOUTCLKUNIT_W {
        TOUTCLKUNIT_W { w: self }
    }
    #[doc = "Bits 13:20 - Base Clock Frequency for SD_CLK"]
    #[inline(always)]
    pub fn baseclkfreq(&mut self) -> BASECLKFREQ_W {
        BASECLKFREQ_W { w: self }
    }
    #[doc = "Bits 21:22 - MAX Block Length of Transfer"]
    #[inline(always)]
    pub fn maxblklen(&mut self) -> MAXBLKLEN_W {
        MAXBLKLEN_W { w: self }
    }
    #[doc = "Bit 23 - 8-bit Interface Support"]
    #[inline(always)]
    pub fn c8bitsup(&mut self) -> C8BITSUP_W {
        C8BITSUP_W { w: self }
    }
    #[doc = "Bit 24 - ADMA2 Mode Support"]
    #[inline(always)]
    pub fn cadma2sup(&mut self) -> CADMA2SUP_W {
        CADMA2SUP_W { w: self }
    }
    #[doc = "Bit 25 - High Speed Mode Support"]
    #[inline(always)]
    pub fn chssup(&mut self) -> CHSSUP_W {
        CHSSUP_W { w: self }
    }
    #[doc = "Bit 26 - SDMA Mode Support"]
    #[inline(always)]
    pub fn csdmasup(&mut self) -> CSDMASUP_W {
        CSDMASUP_W { w: self }
    }
    #[doc = "Bit 27 - Suspend/Resume Support"]
    #[inline(always)]
    pub fn csuspressup(&mut self) -> CSUSPRESSUP_W {
        CSUSPRESSUP_W { w: self }
    }
    #[doc = "Bit 28 - Core 3P3V Support"]
    #[inline(always)]
    pub fn c3p3vsup(&mut self) -> C3P3VSUP_W {
        C3P3VSUP_W { w: self }
    }
    #[doc = "Bit 29 - 3P0V Support"]
    #[inline(always)]
    pub fn c3p0vsup(&mut self) -> C3P0VSUP_W {
        C3P0VSUP_W { w: self }
    }
    #[doc = "Bit 30 - 1P8V Support"]
    #[inline(always)]
    pub fn c1p8vsup(&mut self) -> C1P8VSUP_W {
        C1P8VSUP_W { w: self }
    }
}
