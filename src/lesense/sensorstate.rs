#[doc = "Reader of register SENSORSTATE"]
pub type R = crate::R<u32, super::SENSORSTATE>;
#[doc = "Writer for register SENSORSTATE"]
pub type W = crate::W<u32, super::SENSORSTATE>;
#[doc = "Register SENSORSTATE `reset()`'s with value 0"]
impl crate::ResetValue for super::SENSORSTATE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SENSORSTATE`"]
pub type SENSORSTATE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SENSORSTATE`"]
pub struct SENSORSTATE_W<'a> {
    w: &'a mut W,
}
impl<'a> SENSORSTATE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Decoder Input Register"]
    #[inline(always)]
    pub fn sensorstate(&self) -> SENSORSTATE_R {
        SENSORSTATE_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Decoder Input Register"]
    #[inline(always)]
    pub fn sensorstate(&mut self) -> SENSORSTATE_W {
        SENSORSTATE_W { w: self }
    }
}
