#[doc = "Reader of register STARTUP"]
pub type R = crate::R<u32, super::STARTUP>;
#[doc = "Writer for register STARTUP"]
pub type W = crate::W<u32, super::STARTUP>;
#[doc = "Register STARTUP `reset()`'s with value 0x1300_1054"]
impl crate::ResetValue for super::STARTUP {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x1300_1054
    }
}
#[doc = "Reader of field `STDLY0`"]
pub type STDLY0_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `STDLY0`"]
pub struct STDLY0_W<'a> {
    w: &'a mut W,
}
impl<'a> STDLY0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | ((value as u32) & 0x03ff);
        self.w
    }
}
#[doc = "Reader of field `STDLY1`"]
pub type STDLY1_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `STDLY1`"]
pub struct STDLY1_W<'a> {
    w: &'a mut W,
}
impl<'a> STDLY1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 12)) | (((value as u32) & 0x03ff) << 12);
        self.w
    }
}
#[doc = "Reader of field `ASTWAIT`"]
pub type ASTWAIT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ASTWAIT`"]
pub struct ASTWAIT_W<'a> {
    w: &'a mut W,
}
impl<'a> ASTWAIT_W<'a> {
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
#[doc = "Reader of field `STWSEN`"]
pub type STWSEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STWSEN`"]
pub struct STWSEN_W<'a> {
    w: &'a mut W,
}
impl<'a> STWSEN_W<'a> {
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
#[doc = "Reader of field `STWSAEN`"]
pub type STWSAEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STWSAEN`"]
pub struct STWSAEN_W<'a> {
    w: &'a mut W,
}
impl<'a> STWSAEN_W<'a> {
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
#[doc = "Reader of field `STWS`"]
pub type STWS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `STWS`"]
pub struct STWS_W<'a> {
    w: &'a mut W,
}
impl<'a> STWS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 28)) | (((value as u32) & 0x07) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:9 - Startup Delay 0"]
    #[inline(always)]
    pub fn stdly0(&self) -> STDLY0_R {
        STDLY0_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 12:21 - Startup Delay 0"]
    #[inline(always)]
    pub fn stdly1(&self) -> STDLY1_R {
        STDLY1_R::new(((self.bits >> 12) & 0x03ff) as u16)
    }
    #[doc = "Bit 24 - Active Startup Wait"]
    #[inline(always)]
    pub fn astwait(&self) -> ASTWAIT_R {
        ASTWAIT_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Startup Waitstates Enable"]
    #[inline(always)]
    pub fn stwsen(&self) -> STWSEN_R {
        STWSEN_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Startup Waitstates Always Enable"]
    #[inline(always)]
    pub fn stwsaen(&self) -> STWSAEN_R {
        STWSAEN_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bits 28:30 - Startup Waitstates"]
    #[inline(always)]
    pub fn stws(&self) -> STWS_R {
        STWS_R::new(((self.bits >> 28) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:9 - Startup Delay 0"]
    #[inline(always)]
    pub fn stdly0(&mut self) -> STDLY0_W {
        STDLY0_W { w: self }
    }
    #[doc = "Bits 12:21 - Startup Delay 0"]
    #[inline(always)]
    pub fn stdly1(&mut self) -> STDLY1_W {
        STDLY1_W { w: self }
    }
    #[doc = "Bit 24 - Active Startup Wait"]
    #[inline(always)]
    pub fn astwait(&mut self) -> ASTWAIT_W {
        ASTWAIT_W { w: self }
    }
    #[doc = "Bit 25 - Startup Waitstates Enable"]
    #[inline(always)]
    pub fn stwsen(&mut self) -> STWSEN_W {
        STWSEN_W { w: self }
    }
    #[doc = "Bit 26 - Startup Waitstates Always Enable"]
    #[inline(always)]
    pub fn stwsaen(&mut self) -> STWSAEN_W {
        STWSAEN_W { w: self }
    }
    #[doc = "Bits 28:30 - Startup Waitstates"]
    #[inline(always)]
    pub fn stws(&mut self) -> STWS_W {
        STWS_W { w: self }
    }
}
