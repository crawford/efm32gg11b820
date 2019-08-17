#[doc = "Reader of register TFTMASK"]
pub type R = crate::R<u32, super::TFTMASK>;
#[doc = "Writer for register TFTMASK"]
pub type W = crate::W<u32, super::TFTMASK>;
#[doc = "Register TFTMASK `reset()`'s with value 0"]
impl crate::ResetValue for super::TFTMASK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TFTMASK`"]
pub type TFTMASK_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `TFTMASK`"]
pub struct TFTMASK_W<'a> {
    w: &'a mut W,
}
impl<'a> TFTMASK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x00ff_ffff) | ((value as u32) & 0x00ff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:23 - TFT Mask Value"]
    #[inline(always)]
    pub fn tftmask(&self) -> TFTMASK_R {
        TFTMASK_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:23 - TFT Mask Value"]
    #[inline(always)]
    pub fn tftmask(&mut self) -> TFTMASK_W {
        TFTMASK_W { w: self }
    }
}
