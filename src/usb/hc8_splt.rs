#[doc = "Reader of register HC8_SPLT"]
pub type R = crate::R<u32, super::HC8_SPLT>;
#[doc = "Writer for register HC8_SPLT"]
pub type W = crate::W<u32, super::HC8_SPLT>;
#[doc = "Register HC8_SPLT `reset()`'s with value 0"]
impl crate::ResetValue for super::HC8_SPLT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PRTADDR`"]
pub type PRTADDR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PRTADDR`"]
pub struct PRTADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> PRTADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u32) & 0x7f);
        self.w
    }
}
#[doc = "Reader of field `HUBADDR`"]
pub type HUBADDR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HUBADDR`"]
pub struct HUBADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> HUBADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 7)) | (((value as u32) & 0x7f) << 7);
        self.w
    }
}
#[doc = "Reader of field `XACTPOS`"]
pub type XACTPOS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `XACTPOS`"]
pub struct XACTPOS_W<'a> {
    w: &'a mut W,
}
impl<'a> XACTPOS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
        self.w
    }
}
#[doc = "Reader of field `COMPSPLT`"]
pub type COMPSPLT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `COMPSPLT`"]
pub struct COMPSPLT_W<'a> {
    w: &'a mut W,
}
impl<'a> COMPSPLT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `SPLTENA`"]
pub type SPLTENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPLTENA`"]
pub struct SPLTENA_W<'a> {
    w: &'a mut W,
}
impl<'a> SPLTENA_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - Port Address"]
    #[inline(always)]
    pub fn prtaddr(&self) -> PRTADDR_R {
        PRTADDR_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 7:13 - Hub Address"]
    #[inline(always)]
    pub fn hubaddr(&self) -> HUBADDR_R {
        HUBADDR_R::new(((self.bits >> 7) & 0x7f) as u8)
    }
    #[doc = "Bits 14:15 - Transaction Position"]
    #[inline(always)]
    pub fn xactpos(&self) -> XACTPOS_R {
        XACTPOS_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bit 16 - Do Complete Split"]
    #[inline(always)]
    pub fn compsplt(&self) -> COMPSPLT_R {
        COMPSPLT_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Split Enable"]
    #[inline(always)]
    pub fn spltena(&self) -> SPLTENA_R {
        SPLTENA_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - Port Address"]
    #[inline(always)]
    pub fn prtaddr(&mut self) -> PRTADDR_W {
        PRTADDR_W { w: self }
    }
    #[doc = "Bits 7:13 - Hub Address"]
    #[inline(always)]
    pub fn hubaddr(&mut self) -> HUBADDR_W {
        HUBADDR_W { w: self }
    }
    #[doc = "Bits 14:15 - Transaction Position"]
    #[inline(always)]
    pub fn xactpos(&mut self) -> XACTPOS_W {
        XACTPOS_W { w: self }
    }
    #[doc = "Bit 16 - Do Complete Split"]
    #[inline(always)]
    pub fn compsplt(&mut self) -> COMPSPLT_W {
        COMPSPLT_W { w: self }
    }
    #[doc = "Bit 31 - Split Enable"]
    #[inline(always)]
    pub fn spltena(&mut self) -> SPLTENA_W {
        SPLTENA_W { w: self }
    }
}
