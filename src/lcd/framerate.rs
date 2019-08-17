#[doc = "Reader of register FRAMERATE"]
pub type R = crate::R<u32, super::FRAMERATE>;
#[doc = "Writer for register FRAMERATE"]
pub type W = crate::W<u32, super::FRAMERATE>;
#[doc = "Register FRAMERATE `reset()`'s with value 0"]
impl crate::ResetValue for super::FRAMERATE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FRDIV`"]
pub type FRDIV_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `FRDIV`"]
pub struct FRDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> FRDIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01ff) | ((value as u32) & 0x01ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:8 - Frame Rate Divider"]
    #[inline(always)]
    pub fn frdiv(&self) -> FRDIV_R {
        FRDIV_R::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - Frame Rate Divider"]
    #[inline(always)]
    pub fn frdiv(&mut self) -> FRDIV_W {
        FRDIV_W { w: self }
    }
}
