#[doc = "Reader of register ADDRTIMING3"]
pub type R = crate::R<u32, super::ADDRTIMING3>;
#[doc = "Writer for register ADDRTIMING3"]
pub type W = crate::W<u32, super::ADDRTIMING3>;
#[doc = "Register ADDRTIMING3 `reset()`'s with value 0x0707"]
impl crate::ResetValue for super::ADDRTIMING3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0707
    }
}
#[doc = "Reader of field `ADDRSETUP`"]
pub type ADDRSETUP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADDRSETUP`"]
pub struct ADDRSETUP_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDRSETUP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "Reader of field `ADDRHOLD`"]
pub type ADDRHOLD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADDRHOLD`"]
pub struct ADDRHOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDRHOLD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
#[doc = "Reader of field `HALFALE`"]
pub type HALFALE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HALFALE`"]
pub struct HALFALE_W<'a> {
    w: &'a mut W,
}
impl<'a> HALFALE_W<'a> {
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
impl R {
    #[doc = "Bits 0:2 - Address Setup Time"]
    #[inline(always)]
    pub fn addrsetup(&self) -> ADDRSETUP_R {
        ADDRSETUP_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 8:10 - Address Hold Time"]
    #[inline(always)]
    pub fn addrhold(&self) -> ADDRHOLD_R {
        ADDRHOLD_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bit 28 - Half Cycle ALE Strobe Duration Enable"]
    #[inline(always)]
    pub fn halfale(&self) -> HALFALE_R {
        HALFALE_R::new(((self.bits >> 28) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Address Setup Time"]
    #[inline(always)]
    pub fn addrsetup(&mut self) -> ADDRSETUP_W {
        ADDRSETUP_W { w: self }
    }
    #[doc = "Bits 8:10 - Address Hold Time"]
    #[inline(always)]
    pub fn addrhold(&mut self) -> ADDRHOLD_W {
        ADDRHOLD_W { w: self }
    }
    #[doc = "Bit 28 - Half Cycle ALE Strobe Duration Enable"]
    #[inline(always)]
    pub fn halfale(&mut self) -> HALFALE_W {
        HALFALE_W { w: self }
    }
}
