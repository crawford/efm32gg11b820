#[doc = "Reader of register EVALCTRL"]
pub type R = crate::R<u32, super::EVALCTRL>;
#[doc = "Writer for register EVALCTRL"]
pub type W = crate::W<u32, super::EVALCTRL>;
#[doc = "Register EVALCTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::EVALCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `WINSIZE`"]
pub type WINSIZE_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `WINSIZE`"]
pub struct WINSIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> WINSIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Sliding Window and Step Detection Size"]
    #[inline(always)]
    pub fn winsize(&self) -> WINSIZE_R {
        WINSIZE_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Sliding Window and Step Detection Size"]
    #[inline(always)]
    pub fn winsize(&mut self) -> WINSIZE_W {
        WINSIZE_W { w: self }
    }
}
