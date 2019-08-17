#[doc = "Reader of register CALCNT"]
pub type R = crate::R<u32, super::CALCNT>;
#[doc = "Writer for register CALCNT"]
pub type W = crate::W<u32, super::CALCNT>;
#[doc = "Register CALCNT `reset()`'s with value 0"]
impl crate::ResetValue for super::CALCNT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CALCNT`"]
pub type CALCNT_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `CALCNT`"]
pub struct CALCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> CALCNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x000f_ffff) | ((value as u32) & 0x000f_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:19 - Calibration Counter"]
    #[inline(always)]
    pub fn calcnt(&self) -> CALCNT_R {
        CALCNT_R::new((self.bits & 0x000f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:19 - Calibration Counter"]
    #[inline(always)]
    pub fn calcnt(&mut self) -> CALCNT_W {
        CALCNT_W { w: self }
    }
}
