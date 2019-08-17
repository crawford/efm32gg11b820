#[doc = "Reader of register PRSCTRL"]
pub type R = crate::R<u32, super::PRSCTRL>;
#[doc = "Writer for register PRSCTRL"]
pub type W = crate::W<u32, super::PRSCTRL>;
#[doc = "Register PRSCTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::PRSCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DECCMPVAL`"]
pub type DECCMPVAL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DECCMPVAL`"]
pub struct DECCMPVAL_W<'a> {
    w: &'a mut W,
}
impl<'a> DECCMPVAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
#[doc = "Reader of field `DECCMPMASK`"]
pub type DECCMPMASK_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DECCMPMASK`"]
pub struct DECCMPMASK_W<'a> {
    w: &'a mut W,
}
impl<'a> DECCMPMASK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | (((value as u32) & 0x1f) << 8);
        self.w
    }
}
#[doc = "Reader of field `DECCMPEN`"]
pub type DECCMPEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DECCMPEN`"]
pub struct DECCMPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DECCMPEN_W<'a> {
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
impl R {
    #[doc = "Bits 0:4 - Decoder State Compare Value"]
    #[inline(always)]
    pub fn deccmpval(&self) -> DECCMPVAL_R {
        DECCMPVAL_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - Decoder State Compare Value Mask"]
    #[inline(always)]
    pub fn deccmpmask(&self) -> DECCMPMASK_R {
        DECCMPMASK_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bit 16 - Enable PRS Output DECCMP"]
    #[inline(always)]
    pub fn deccmpen(&self) -> DECCMPEN_R {
        DECCMPEN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Decoder State Compare Value"]
    #[inline(always)]
    pub fn deccmpval(&mut self) -> DECCMPVAL_W {
        DECCMPVAL_W { w: self }
    }
    #[doc = "Bits 8:12 - Decoder State Compare Value Mask"]
    #[inline(always)]
    pub fn deccmpmask(&mut self) -> DECCMPMASK_W {
        DECCMPMASK_W { w: self }
    }
    #[doc = "Bit 16 - Enable PRS Output DECCMP"]
    #[inline(always)]
    pub fn deccmpen(&mut self) -> DECCMPEN_W {
        DECCMPEN_W { w: self }
    }
}
