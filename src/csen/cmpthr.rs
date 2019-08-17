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
#[doc = "Reader of field `CMPTHR`"]
pub type CMPTHR_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CMPTHR`"]
pub struct CMPTHR_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPTHR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Comparator Threshold."]
    #[inline(always)]
    pub fn cmpthr(&self) -> CMPTHR_R {
        CMPTHR_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Comparator Threshold."]
    #[inline(always)]
    pub fn cmpthr(&mut self) -> CMPTHR_W {
        CMPTHR_W { w: self }
    }
}
