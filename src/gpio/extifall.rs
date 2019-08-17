#[doc = "Reader of register EXTIFALL"]
pub type R = crate::R<u32, super::EXTIFALL>;
#[doc = "Writer for register EXTIFALL"]
pub type W = crate::W<u32, super::EXTIFALL>;
#[doc = "Register EXTIFALL `reset()`'s with value 0"]
impl crate::ResetValue for super::EXTIFALL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EXTIFALL`"]
pub type EXTIFALL_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `EXTIFALL`"]
pub struct EXTIFALL_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTIFALL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - External Interrupt N Falling Edge Trigger Enable"]
    #[inline(always)]
    pub fn extifall(&self) -> EXTIFALL_R {
        EXTIFALL_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - External Interrupt N Falling Edge Trigger Enable"]
    #[inline(always)]
    pub fn extifall(&mut self) -> EXTIFALL_W {
        EXTIFALL_W { w: self }
    }
}
