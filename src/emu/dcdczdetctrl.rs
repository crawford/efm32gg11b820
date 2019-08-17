#[doc = "Reader of register DCDCZDETCTRL"]
pub type R = crate::R<u32, super::DCDCZDETCTRL>;
#[doc = "Writer for register DCDCZDETCTRL"]
pub type W = crate::W<u32, super::DCDCZDETCTRL>;
#[doc = "Register DCDCZDETCTRL `reset()`'s with value 0x0150"]
impl crate::ResetValue for super::DCDCZDETCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0150
    }
}
#[doc = "Reader of field `ZDETILIMSEL`"]
pub type ZDETILIMSEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ZDETILIMSEL`"]
pub struct ZDETILIMSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> ZDETILIMSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u32) & 0x07) << 4);
        self.w
    }
}
#[doc = "Reader of field `ZDETBLANKDLY`"]
pub type ZDETBLANKDLY_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ZDETBLANKDLY`"]
pub struct ZDETBLANKDLY_W<'a> {
    w: &'a mut W,
}
impl<'a> ZDETBLANKDLY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 4:6 - Reverse Current Limit Level Selection for Zero Detector"]
    #[inline(always)]
    pub fn zdetilimsel(&self) -> ZDETILIMSEL_R {
        ZDETILIMSEL_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bits 8:9 - Reserved for internal use. Do not change."]
    #[inline(always)]
    pub fn zdetblankdly(&self) -> ZDETBLANKDLY_R {
        ZDETBLANKDLY_R::new(((self.bits >> 8) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 4:6 - Reverse Current Limit Level Selection for Zero Detector"]
    #[inline(always)]
    pub fn zdetilimsel(&mut self) -> ZDETILIMSEL_W {
        ZDETILIMSEL_W { w: self }
    }
    #[doc = "Bits 8:9 - Reserved for internal use. Do not change."]
    #[inline(always)]
    pub fn zdetblankdly(&mut self) -> ZDETBLANKDLY_W {
        ZDETBLANKDLY_W { w: self }
    }
}
