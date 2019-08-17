#[doc = "Reader of register ETMCLAIMSET"]
pub type R = crate::R<u32, super::ETMCLAIMSET>;
#[doc = "Writer for register ETMCLAIMSET"]
pub type W = crate::W<u32, super::ETMCLAIMSET>;
#[doc = "Register ETMCLAIMSET `reset()`'s with value 0x0f"]
impl crate::ResetValue for super::ETMCLAIMSET {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0f
    }
}
#[doc = "Reader of field `SETTAG`"]
pub type SETTAG_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SETTAG`"]
pub struct SETTAG_W<'a> {
    w: &'a mut W,
}
impl<'a> SETTAG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Tag Bits"]
    #[inline(always)]
    pub fn settag(&self) -> SETTAG_R {
        SETTAG_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Tag Bits"]
    #[inline(always)]
    pub fn settag(&mut self) -> SETTAG_W {
        SETTAG_W { w: self }
    }
}
