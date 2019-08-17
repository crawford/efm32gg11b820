#[doc = "Reader of register TFTFRAMEBASE"]
pub type R = crate::R<u32, super::TFTFRAMEBASE>;
#[doc = "Writer for register TFTFRAMEBASE"]
pub type W = crate::W<u32, super::TFTFRAMEBASE>;
#[doc = "Register TFTFRAMEBASE `reset()`'s with value 0"]
impl crate::ResetValue for super::TFTFRAMEBASE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FRAMEBASE`"]
pub type FRAMEBASE_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `FRAMEBASE`"]
pub struct FRAMEBASE_W<'a> {
    w: &'a mut W,
}
impl<'a> FRAMEBASE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff_ffff) | ((value as u32) & 0x0fff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:27 - Frame Base Address"]
    #[inline(always)]
    pub fn framebase(&self) -> FRAMEBASE_R {
        FRAMEBASE_R::new((self.bits & 0x0fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:27 - Frame Base Address"]
    #[inline(always)]
    pub fn framebase(&mut self) -> FRAMEBASE_W {
        FRAMEBASE_W { w: self }
    }
}
