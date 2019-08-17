#[doc = "Reader of register DECSTATE"]
pub type R = crate::R<u32, super::DECSTATE>;
#[doc = "Writer for register DECSTATE"]
pub type W = crate::W<u32, super::DECSTATE>;
#[doc = "Register DECSTATE `reset()`'s with value 0"]
impl crate::ResetValue for super::DECSTATE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DECSTATE`"]
pub type DECSTATE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DECSTATE`"]
pub struct DECSTATE_W<'a> {
    w: &'a mut W,
}
impl<'a> DECSTATE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - Current Decoder State"]
    #[inline(always)]
    pub fn decstate(&self) -> DECSTATE_R {
        DECSTATE_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Current Decoder State"]
    #[inline(always)]
    pub fn decstate(&mut self) -> DECSTATE_W {
        DECSTATE_W { w: self }
    }
}
