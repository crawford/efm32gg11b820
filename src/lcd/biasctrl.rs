#[doc = "Reader of register BIASCTRL"]
pub type R = crate::R<u32, super::BIASCTRL>;
#[doc = "Writer for register BIASCTRL"]
pub type W = crate::W<u32, super::BIASCTRL>;
#[doc = "Register BIASCTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::BIASCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SPEED`"]
pub type SPEED_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SPEED`"]
pub struct SPEED_W<'a> {
    w: &'a mut W,
}
impl<'a> SPEED_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "Reader of field `BUFDRV`"]
pub type BUFDRV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `BUFDRV`"]
pub struct BUFDRV_W<'a> {
    w: &'a mut W,
}
impl<'a> BUFDRV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `BUFBIAS`"]
pub type BUFBIAS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `BUFBIAS`"]
pub struct BUFBIAS_W<'a> {
    w: &'a mut W,
}
impl<'a> BUFBIAS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 10)) | (((value as u32) & 0x07) << 10);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - SPEED Adjustment"]
    #[inline(always)]
    pub fn speed(&self) -> SPEED_R {
        SPEED_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 4:7 - Buffer Drive Strength"]
    #[inline(always)]
    pub fn bufdrv(&self) -> BUFDRV_R {
        BUFDRV_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 10:12 - Buffer Bias Setting"]
    #[inline(always)]
    pub fn bufbias(&self) -> BUFBIAS_R {
        BUFBIAS_R::new(((self.bits >> 10) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - SPEED Adjustment"]
    #[inline(always)]
    pub fn speed(&mut self) -> SPEED_W {
        SPEED_W { w: self }
    }
    #[doc = "Bits 4:7 - Buffer Drive Strength"]
    #[inline(always)]
    pub fn bufdrv(&mut self) -> BUFDRV_W {
        BUFDRV_W { w: self }
    }
    #[doc = "Bits 10:12 - Buffer Bias Setting"]
    #[inline(always)]
    pub fn bufbias(&mut self) -> BUFBIAS_W {
        BUFBIAS_W { w: self }
    }
}
