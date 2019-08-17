#[doc = "Reader of register HFRCOSS"]
pub type R = crate::R<u32, super::HFRCOSS>;
#[doc = "Writer for register HFRCOSS"]
pub type W = crate::W<u32, super::HFRCOSS>;
#[doc = "Register HFRCOSS `reset()`'s with value 0"]
impl crate::ResetValue for super::HFRCOSS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SSAMP`"]
pub type SSAMP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SSAMP`"]
pub struct SSAMP_W<'a> {
    w: &'a mut W,
}
impl<'a> SSAMP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "Reader of field `SSINV`"]
pub type SSINV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SSINV`"]
pub struct SSINV_W<'a> {
    w: &'a mut W,
}
impl<'a> SSINV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | (((value as u32) & 0x1f) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Spread Spectrum Amplitude"]
    #[inline(always)]
    pub fn ssamp(&self) -> SSAMP_R {
        SSAMP_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 8:12 - Spread Spectrum Update Interval"]
    #[inline(always)]
    pub fn ssinv(&self) -> SSINV_R {
        SSINV_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Spread Spectrum Amplitude"]
    #[inline(always)]
    pub fn ssamp(&mut self) -> SSAMP_W {
        SSAMP_W { w: self }
    }
    #[doc = "Bits 8:12 - Spread Spectrum Update Interval"]
    #[inline(always)]
    pub fn ssinv(&mut self) -> SSINV_W {
        SSINV_W { w: self }
    }
}
