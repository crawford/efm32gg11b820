#[doc = "Reader of register SIGFRAME"]
pub type R = crate::R<u32, super::SIGFRAME>;
#[doc = "Writer for register SIGFRAME"]
pub type W = crate::W<u32, super::SIGFRAME>;
#[doc = "Register SIGFRAME `reset()`'s with value 0"]
impl crate::ResetValue for super::SIGFRAME {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SIGFRAME`"]
pub type SIGFRAME_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `SIGFRAME`"]
pub struct SIGFRAME_W<'a> {
    w: &'a mut W,
}
impl<'a> SIGFRAME_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01ff) | ((value as u32) & 0x01ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:8 - Signal Frame"]
    #[inline(always)]
    pub fn sigframe(&self) -> SIGFRAME_R {
        SIGFRAME_R::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - Signal Frame"]
    #[inline(always)]
    pub fn sigframe(&mut self) -> SIGFRAME_W {
        SIGFRAME_W { w: self }
    }
}
