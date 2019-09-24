#[doc = "Reader of register CLOCKCTRL"]
pub type R = crate::R<u32, super::CLOCKCTRL>;
#[doc = "Writer for register CLOCKCTRL"]
pub type W = crate::W<u32, super::CLOCKCTRL>;
#[doc = "Register CLOCKCTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::CLOCKCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `INTCLKEN`"]
pub type INTCLKEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INTCLKEN`"]
pub struct INTCLKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> INTCLKEN_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `INTCLKSTABLE`"]
pub type INTCLKSTABLE_R = crate::R<bool, bool>;
#[doc = "Reader of field `SDCLKEN`"]
pub type SDCLKEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SDCLKEN`"]
pub struct SDCLKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SDCLKEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `CLKGENSEL`"]
pub type CLKGENSEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLKGENSEL`"]
pub struct CLKGENSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKGENSEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `UPPSDCLKFRE`"]
pub type UPPSDCLKFRE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `UPPSDCLKFRE`"]
pub struct UPPSDCLKFRE_W<'a> {
    w: &'a mut W,
}
impl<'a> UPPSDCLKFRE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "SD_CLK Frequency Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SDCLKFREQSEL_A {
    #[doc = "0: `0`"]
    NODIVISION,
}
impl From<SDCLKFREQSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: SDCLKFREQSEL_A) -> Self {
        match variant {
            SDCLKFREQSEL_A::NODIVISION => 0,
        }
    }
}
#[doc = "Reader of field `SDCLKFREQSEL`"]
pub type SDCLKFREQSEL_R = crate::R<u8, SDCLKFREQSEL_A>;
impl SDCLKFREQSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SDCLKFREQSEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SDCLKFREQSEL_A::NODIVISION),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `NODIVISION`"]
    #[inline(always)]
    pub fn is_nodivision(&self) -> bool {
        *self == SDCLKFREQSEL_A::NODIVISION
    }
}
#[doc = "Write proxy for field `SDCLKFREQSEL`"]
pub struct SDCLKFREQSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SDCLKFREQSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SDCLKFREQSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn nodivision(self) -> &'a mut W {
        self.variant(SDCLKFREQSEL_A::NODIVISION)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `DATTOUTCNTVAL`"]
pub type DATTOUTCNTVAL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DATTOUTCNTVAL`"]
pub struct DATTOUTCNTVAL_W<'a> {
    w: &'a mut W,
}
impl<'a> DATTOUTCNTVAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Reader of field `SFTRSTA`"]
pub type SFTRSTA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SFTRSTA`"]
pub struct SFTRSTA_W<'a> {
    w: &'a mut W,
}
impl<'a> SFTRSTA_W<'a> {
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
#[doc = "Reader of field `SFTRSTCMD`"]
pub type SFTRSTCMD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SFTRSTCMD`"]
pub struct SFTRSTCMD_W<'a> {
    w: &'a mut W,
}
impl<'a> SFTRSTCMD_W<'a> {
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
#[doc = "Reader of field `SFTRSTDAT`"]
pub type SFTRSTDAT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SFTRSTDAT`"]
pub struct SFTRSTDAT_W<'a> {
    w: &'a mut W,
}
impl<'a> SFTRSTDAT_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Internal Clock Enable"]
    #[inline(always)]
    pub fn intclken(&self) -> INTCLKEN_R {
        INTCLKEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Internal Clock Stable"]
    #[inline(always)]
    pub fn intclkstable(&self) -> INTCLKSTABLE_R {
        INTCLKSTABLE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - SDIO_CLK Pin Clock Enable"]
    #[inline(always)]
    pub fn sdclken(&self) -> SDCLKEN_R {
        SDCLKEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Clock Generator Select"]
    #[inline(always)]
    pub fn clkgensel(&self) -> CLKGENSEL_R {
        CLKGENSEL_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 6:7 - Upper Bits of SD_CLK Frequency Select"]
    #[inline(always)]
    pub fn uppsdclkfre(&self) -> UPPSDCLKFRE_R {
        UPPSDCLKFRE_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:15 - SD_CLK Frequency Select"]
    #[inline(always)]
    pub fn sdclkfreqsel(&self) -> SDCLKFREQSEL_R {
        SDCLKFREQSEL_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:19 - Data Timeout Counter Value"]
    #[inline(always)]
    pub fn dattoutcntval(&self) -> DATTOUTCNTVAL_R {
        DATTOUTCNTVAL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - Software Reset for All"]
    #[inline(always)]
    pub fn sftrsta(&self) -> SFTRSTA_R {
        SFTRSTA_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Software Reset for CMD Line"]
    #[inline(always)]
    pub fn sftrstcmd(&self) -> SFTRSTCMD_R {
        SFTRSTCMD_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Software Reset for DAT Line"]
    #[inline(always)]
    pub fn sftrstdat(&self) -> SFTRSTDAT_R {
        SFTRSTDAT_R::new(((self.bits >> 26) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Internal Clock Enable"]
    #[inline(always)]
    pub fn intclken(&mut self) -> INTCLKEN_W {
        INTCLKEN_W { w: self }
    }
    #[doc = "Bit 2 - SDIO_CLK Pin Clock Enable"]
    #[inline(always)]
    pub fn sdclken(&mut self) -> SDCLKEN_W {
        SDCLKEN_W { w: self }
    }
    #[doc = "Bit 5 - Clock Generator Select"]
    #[inline(always)]
    pub fn clkgensel(&mut self) -> CLKGENSEL_W {
        CLKGENSEL_W { w: self }
    }
    #[doc = "Bits 6:7 - Upper Bits of SD_CLK Frequency Select"]
    #[inline(always)]
    pub fn uppsdclkfre(&mut self) -> UPPSDCLKFRE_W {
        UPPSDCLKFRE_W { w: self }
    }
    #[doc = "Bits 8:15 - SD_CLK Frequency Select"]
    #[inline(always)]
    pub fn sdclkfreqsel(&mut self) -> SDCLKFREQSEL_W {
        SDCLKFREQSEL_W { w: self }
    }
    #[doc = "Bits 16:19 - Data Timeout Counter Value"]
    #[inline(always)]
    pub fn dattoutcntval(&mut self) -> DATTOUTCNTVAL_W {
        DATTOUTCNTVAL_W { w: self }
    }
    #[doc = "Bit 24 - Software Reset for All"]
    #[inline(always)]
    pub fn sftrsta(&mut self) -> SFTRSTA_W {
        SFTRSTA_W { w: self }
    }
    #[doc = "Bit 25 - Software Reset for CMD Line"]
    #[inline(always)]
    pub fn sftrstcmd(&mut self) -> SFTRSTCMD_W {
        SFTRSTCMD_W { w: self }
    }
    #[doc = "Bit 26 - Software Reset for DAT Line"]
    #[inline(always)]
    pub fn sftrstdat(&mut self) -> SFTRSTDAT_W {
        SFTRSTDAT_W { w: self }
    }
}
