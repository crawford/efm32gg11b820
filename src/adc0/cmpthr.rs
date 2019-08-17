#[doc = "Reader of register CMPTHR"]
pub type R = crate::R<u32, super::CMPTHR>;
#[doc = "Writer for register CMPTHR"]
pub type W = crate::W<u32, super::CMPTHR>;
#[doc = "Register CMPTHR `reset()`'s with value 0"]
impl crate::ResetValue for super::CMPTHR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADLT`"]
pub type ADLT_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `ADLT`"]
pub struct ADLT_W<'a> {
    w: &'a mut W,
}
impl<'a> ADLT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Reader of field `ADGT`"]
pub type ADGT_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `ADGT`"]
pub struct ADGT_W<'a> {
    w: &'a mut W,
}
impl<'a> ADGT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Less Than Compare Threshold"]
    #[inline(always)]
    pub fn adlt(&self) -> ADLT_R {
        ADLT_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Greater Than Compare Threshold"]
    #[inline(always)]
    pub fn adgt(&self) -> ADGT_R {
        ADGT_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Less Than Compare Threshold"]
    #[inline(always)]
    pub fn adlt(&mut self) -> ADLT_W {
        ADLT_W { w: self }
    }
    #[doc = "Bits 16:31 - Greater Than Compare Threshold"]
    #[inline(always)]
    pub fn adgt(&mut self) -> ADGT_W {
        ADGT_W { w: self }
    }
}
