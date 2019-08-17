#[doc = "Reader of register R5VOUTLEVEL"]
pub type R = crate::R<u32, super::R5VOUTLEVEL>;
#[doc = "Writer for register R5VOUTLEVEL"]
pub type W = crate::W<u32, super::R5VOUTLEVEL>;
#[doc = "Register R5VOUTLEVEL `reset()`'s with value 0x01"]
impl crate::ResetValue for super::R5VOUTLEVEL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x01
    }
}
#[doc = "Reader of field `OUTLEVEL`"]
pub type OUTLEVEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `OUTLEVEL`"]
pub struct OUTLEVEL_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTLEVEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - 5V Regulator Voltage"]
    #[inline(always)]
    pub fn outlevel(&self) -> OUTLEVEL_R {
        OUTLEVEL_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 5V Regulator Voltage"]
    #[inline(always)]
    pub fn outlevel(&mut self) -> OUTLEVEL_W {
        OUTLEVEL_W { w: self }
    }
}
