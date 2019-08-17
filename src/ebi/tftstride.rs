#[doc = "Reader of register TFTSTRIDE"]
pub type R = crate::R<u32, super::TFTSTRIDE>;
#[doc = "Writer for register TFTSTRIDE"]
pub type W = crate::W<u32, super::TFTSTRIDE>;
#[doc = "Register TFTSTRIDE `reset()`'s with value 0"]
impl crate::ResetValue for super::TFTSTRIDE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `HSTRIDE`"]
pub type HSTRIDE_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `HSTRIDE`"]
pub struct HSTRIDE_W<'a> {
    w: &'a mut W,
}
impl<'a> HSTRIDE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:11 - Horizontal Stride"]
    #[inline(always)]
    pub fn hstride(&self) -> HSTRIDE_R {
        HSTRIDE_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Horizontal Stride"]
    #[inline(always)]
    pub fn hstride(&mut self) -> HSTRIDE_W {
        HSTRIDE_W { w: self }
    }
}
