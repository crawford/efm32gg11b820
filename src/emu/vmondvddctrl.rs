#[doc = "Reader of register VMONDVDDCTRL"]
pub type R = crate::R<u32, super::VMONDVDDCTRL>;
#[doc = "Writer for register VMONDVDDCTRL"]
pub type W = crate::W<u32, super::VMONDVDDCTRL>;
#[doc = "Register VMONDVDDCTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::VMONDVDDCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EN`"]
pub type EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EN`"]
pub struct EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EN_W<'a> {
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
#[doc = "Reader of field `RISEWU`"]
pub type RISEWU_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RISEWU`"]
pub struct RISEWU_W<'a> {
    w: &'a mut W,
}
impl<'a> RISEWU_W<'a> {
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
#[doc = "Reader of field `FALLWU`"]
pub type FALLWU_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FALLWU`"]
pub struct FALLWU_W<'a> {
    w: &'a mut W,
}
impl<'a> FALLWU_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `THRESFINE`"]
pub type THRESFINE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `THRESFINE`"]
pub struct THRESFINE_W<'a> {
    w: &'a mut W,
}
impl<'a> THRESFINE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `THRESCOARSE`"]
pub type THRESCOARSE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `THRESCOARSE`"]
pub struct THRESCOARSE_W<'a> {
    w: &'a mut W,
}
impl<'a> THRESCOARSE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Enable"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 2 - Rise Wakeup"]
    #[inline(always)]
    pub fn risewu(&self) -> RISEWU_R {
        RISEWU_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Fall Wakeup"]
    #[inline(always)]
    pub fn fallwu(&self) -> FALLWU_R {
        FALLWU_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 8:11 - Threshold Fine Adjust"]
    #[inline(always)]
    pub fn thresfine(&self) -> THRESFINE_R {
        THRESFINE_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Threshold Coarse Adjust"]
    #[inline(always)]
    pub fn threscoarse(&self) -> THRESCOARSE_R {
        THRESCOARSE_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enable"]
    #[inline(always)]
    pub fn en(&mut self) -> EN_W {
        EN_W { w: self }
    }
    #[doc = "Bit 2 - Rise Wakeup"]
    #[inline(always)]
    pub fn risewu(&mut self) -> RISEWU_W {
        RISEWU_W { w: self }
    }
    #[doc = "Bit 3 - Fall Wakeup"]
    #[inline(always)]
    pub fn fallwu(&mut self) -> FALLWU_W {
        FALLWU_W { w: self }
    }
    #[doc = "Bits 8:11 - Threshold Fine Adjust"]
    #[inline(always)]
    pub fn thresfine(&mut self) -> THRESFINE_W {
        THRESFINE_W { w: self }
    }
    #[doc = "Bits 12:15 - Threshold Coarse Adjust"]
    #[inline(always)]
    pub fn threscoarse(&mut self) -> THRESCOARSE_W {
        THRESCOARSE_W { w: self }
    }
}
