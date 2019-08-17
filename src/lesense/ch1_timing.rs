#[doc = "Reader of register CH1_TIMING"]
pub type R = crate::R<u32, super::CH1_TIMING>;
#[doc = "Writer for register CH1_TIMING"]
pub type W = crate::W<u32, super::CH1_TIMING>;
#[doc = "Register CH1_TIMING `reset()`'s with value 0"]
impl crate::ResetValue for super::CH1_TIMING {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EXTIME`"]
pub type EXTIME_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EXTIME`"]
pub struct EXTIME_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTIME_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
#[doc = "Reader of field `SAMPLEDLY`"]
pub type SAMPLEDLY_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SAMPLEDLY`"]
pub struct SAMPLEDLY_W<'a> {
    w: &'a mut W,
}
impl<'a> SAMPLEDLY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 6)) | (((value as u32) & 0xff) << 6);
        self.w
    }
}
#[doc = "Reader of field `MEASUREDLY`"]
pub type MEASUREDLY_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `MEASUREDLY`"]
pub struct MEASUREDLY_W<'a> {
    w: &'a mut W,
}
impl<'a> MEASUREDLY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 14)) | (((value as u32) & 0x03ff) << 14);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - Set Excitation Time"]
    #[inline(always)]
    pub fn extime(&self) -> EXTIME_R {
        EXTIME_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:13 - Set Sample Delay"]
    #[inline(always)]
    pub fn sampledly(&self) -> SAMPLEDLY_R {
        SAMPLEDLY_R::new(((self.bits >> 6) & 0xff) as u8)
    }
    #[doc = "Bits 14:23 - Set Measure Delay"]
    #[inline(always)]
    pub fn measuredly(&self) -> MEASUREDLY_R {
        MEASUREDLY_R::new(((self.bits >> 14) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:5 - Set Excitation Time"]
    #[inline(always)]
    pub fn extime(&mut self) -> EXTIME_W {
        EXTIME_W { w: self }
    }
    #[doc = "Bits 6:13 - Set Sample Delay"]
    #[inline(always)]
    pub fn sampledly(&mut self) -> SAMPLEDLY_W {
        SAMPLEDLY_W { w: self }
    }
    #[doc = "Bits 14:23 - Set Measure Delay"]
    #[inline(always)]
    pub fn measuredly(&mut self) -> MEASUREDLY_W {
        MEASUREDLY_W { w: self }
    }
}
