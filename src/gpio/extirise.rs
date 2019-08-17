#[doc = "Reader of register EXTIRISE"]
pub type R = crate::R<u32, super::EXTIRISE>;
#[doc = "Writer for register EXTIRISE"]
pub type W = crate::W<u32, super::EXTIRISE>;
#[doc = "Register EXTIRISE `reset()`'s with value 0"]
impl crate::ResetValue for super::EXTIRISE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EXTIRISE`"]
pub type EXTIRISE_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `EXTIRISE`"]
pub struct EXTIRISE_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTIRISE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - External Interrupt N Rising Edge Trigger Enable"]
    #[inline(always)]
    pub fn extirise(&self) -> EXTIRISE_R {
        EXTIRISE_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - External Interrupt N Rising Edge Trigger Enable"]
    #[inline(always)]
    pub fn extirise(&mut self) -> EXTIRISE_W {
        EXTIRISE_W { w: self }
    }
}
