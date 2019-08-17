#[doc = "Reader of register VMONAVDDCTRL"]
pub type R = crate::R<u32, super::VMONAVDDCTRL>;
#[doc = "Writer for register VMONAVDDCTRL"]
pub type W = crate::W<u32, super::VMONAVDDCTRL>;
#[doc = "Register VMONAVDDCTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::VMONAVDDCTRL {
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
#[doc = "Reader of field `FALLTHRESFINE`"]
pub type FALLTHRESFINE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FALLTHRESFINE`"]
pub struct FALLTHRESFINE_W<'a> {
    w: &'a mut W,
}
impl<'a> FALLTHRESFINE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `FALLTHRESCOARSE`"]
pub type FALLTHRESCOARSE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FALLTHRESCOARSE`"]
pub struct FALLTHRESCOARSE_W<'a> {
    w: &'a mut W,
}
impl<'a> FALLTHRESCOARSE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
#[doc = "Reader of field `RISETHRESFINE`"]
pub type RISETHRESFINE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RISETHRESFINE`"]
pub struct RISETHRESFINE_W<'a> {
    w: &'a mut W,
}
impl<'a> RISETHRESFINE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Reader of field `RISETHRESCOARSE`"]
pub type RISETHRESCOARSE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RISETHRESCOARSE`"]
pub struct RISETHRESCOARSE_W<'a> {
    w: &'a mut W,
}
impl<'a> RISETHRESCOARSE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | (((value as u32) & 0x0f) << 20);
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
    #[doc = "Bits 8:11 - Falling Threshold Fine Adjust"]
    #[inline(always)]
    pub fn fallthresfine(&self) -> FALLTHRESFINE_R {
        FALLTHRESFINE_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Falling Threshold Coarse Adjust"]
    #[inline(always)]
    pub fn fallthrescoarse(&self) -> FALLTHRESCOARSE_R {
        FALLTHRESCOARSE_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Rising Threshold Fine Adjust"]
    #[inline(always)]
    pub fn risethresfine(&self) -> RISETHRESFINE_R {
        RISETHRESFINE_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Rising Threshold Coarse Adjust"]
    #[inline(always)]
    pub fn risethrescoarse(&self) -> RISETHRESCOARSE_R {
        RISETHRESCOARSE_R::new(((self.bits >> 20) & 0x0f) as u8)
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
    #[doc = "Bits 8:11 - Falling Threshold Fine Adjust"]
    #[inline(always)]
    pub fn fallthresfine(&mut self) -> FALLTHRESFINE_W {
        FALLTHRESFINE_W { w: self }
    }
    #[doc = "Bits 12:15 - Falling Threshold Coarse Adjust"]
    #[inline(always)]
    pub fn fallthrescoarse(&mut self) -> FALLTHRESCOARSE_W {
        FALLTHRESCOARSE_W { w: self }
    }
    #[doc = "Bits 16:19 - Rising Threshold Fine Adjust"]
    #[inline(always)]
    pub fn risethresfine(&mut self) -> RISETHRESFINE_W {
        RISETHRESFINE_W { w: self }
    }
    #[doc = "Bits 20:23 - Rising Threshold Coarse Adjust"]
    #[inline(always)]
    pub fn risethrescoarse(&mut self) -> RISETHRESCOARSE_W {
        RISETHRESCOARSE_W { w: self }
    }
}
