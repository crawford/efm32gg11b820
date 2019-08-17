#[doc = "Reader of register TFTSIZE"]
pub type R = crate::R<u32, super::TFTSIZE>;
#[doc = "Writer for register TFTSIZE"]
pub type W = crate::W<u32, super::TFTSIZE>;
#[doc = "Register TFTSIZE `reset()`'s with value 0"]
impl crate::ResetValue for super::TFTSIZE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `HSZ`"]
pub type HSZ_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `HSZ`"]
pub struct HSZ_W<'a> {
    w: &'a mut W,
}
impl<'a> HSZ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | ((value as u32) & 0x03ff);
        self.w
    }
}
#[doc = "Reader of field `VSZ`"]
pub type VSZ_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `VSZ`"]
pub struct VSZ_W<'a> {
    w: &'a mut W,
}
impl<'a> VSZ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 16)) | (((value as u32) & 0x03ff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:9 - Horizontal Size (excluding Porches)"]
    #[inline(always)]
    pub fn hsz(&self) -> HSZ_R {
        HSZ_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25 - Vertical Size (excluding Porches)"]
    #[inline(always)]
    pub fn vsz(&self) -> VSZ_R {
        VSZ_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Horizontal Size (excluding Porches)"]
    #[inline(always)]
    pub fn hsz(&mut self) -> HSZ_W {
        HSZ_W { w: self }
    }
    #[doc = "Bits 16:25 - Vertical Size (excluding Porches)"]
    #[inline(always)]
    pub fn vsz(&mut self) -> VSZ_W {
        VSZ_W { w: self }
    }
}
