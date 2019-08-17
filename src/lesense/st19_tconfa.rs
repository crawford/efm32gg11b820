#[doc = "Reader of register ST19_TCONFA"]
pub type R = crate::R<u32, super::ST19_TCONFA>;
#[doc = "Writer for register ST19_TCONFA"]
pub type W = crate::W<u32, super::ST19_TCONFA>;
#[doc = "Register ST19_TCONFA `reset()`'s with value 0"]
impl crate::ResetValue for super::ST19_TCONFA {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `COMP`"]
pub type COMP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `COMP`"]
pub struct COMP_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `MASK`"]
pub type MASK_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MASK`"]
pub struct MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> MASK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `NEXTSTATE`"]
pub type NEXTSTATE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `NEXTSTATE`"]
pub struct NEXTSTATE_W<'a> {
    w: &'a mut W,
}
impl<'a> NEXTSTATE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | (((value as u32) & 0x1f) << 8);
        self.w
    }
}
#[doc = "Reader of field `CHAIN`"]
pub type CHAIN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CHAIN`"]
pub struct CHAIN_W<'a> {
    w: &'a mut W,
}
impl<'a> CHAIN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `SETIF`"]
pub type SETIF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SETIF`"]
pub struct SETIF_W<'a> {
    w: &'a mut W,
}
impl<'a> SETIF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Reader of field `PRSACT`"]
pub type PRSACT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PRSACT`"]
pub struct PRSACT_W<'a> {
    w: &'a mut W,
}
impl<'a> PRSACT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | (((value as u32) & 0x07) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Sensor Compare Value"]
    #[inline(always)]
    pub fn comp(&self) -> COMP_R {
        COMP_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Sensor Mask"]
    #[inline(always)]
    pub fn mask(&self) -> MASK_R {
        MASK_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:12 - Next State Index"]
    #[inline(always)]
    pub fn nextstate(&self) -> NEXTSTATE_R {
        NEXTSTATE_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bit 14 - Enable State Descriptor Chaining"]
    #[inline(always)]
    pub fn chain(&self) -> CHAIN_R {
        CHAIN_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Set Interrupt Flag Enable"]
    #[inline(always)]
    pub fn setif(&self) -> SETIF_R {
        SETIF_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:18 - Configure Transition Action"]
    #[inline(always)]
    pub fn prsact(&self) -> PRSACT_R {
        PRSACT_R::new(((self.bits >> 16) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Sensor Compare Value"]
    #[inline(always)]
    pub fn comp(&mut self) -> COMP_W {
        COMP_W { w: self }
    }
    #[doc = "Bits 4:7 - Sensor Mask"]
    #[inline(always)]
    pub fn mask(&mut self) -> MASK_W {
        MASK_W { w: self }
    }
    #[doc = "Bits 8:12 - Next State Index"]
    #[inline(always)]
    pub fn nextstate(&mut self) -> NEXTSTATE_W {
        NEXTSTATE_W { w: self }
    }
    #[doc = "Bit 14 - Enable State Descriptor Chaining"]
    #[inline(always)]
    pub fn chain(&mut self) -> CHAIN_W {
        CHAIN_W { w: self }
    }
    #[doc = "Bit 15 - Set Interrupt Flag Enable"]
    #[inline(always)]
    pub fn setif(&mut self) -> SETIF_W {
        SETIF_W { w: self }
    }
    #[doc = "Bits 16:18 - Configure Transition Action"]
    #[inline(always)]
    pub fn prsact(&mut self) -> PRSACT_W {
        PRSACT_W { w: self }
    }
}
