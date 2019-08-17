#[doc = "Reader of register SEGEN2"]
pub type R = crate::R<u32, super::SEGEN2>;
#[doc = "Writer for register SEGEN2"]
pub type W = crate::W<u32, super::SEGEN2>;
#[doc = "Register SEGEN2 `reset()`'s with value 0"]
impl crate::ResetValue for super::SEGEN2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SEGEN2`"]
pub type SEGEN2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SEGEN2`"]
pub struct SEGEN2_W<'a> {
    w: &'a mut W,
}
impl<'a> SEGEN2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Segment Enable (second Group)"]
    #[inline(always)]
    pub fn segen2(&self) -> SEGEN2_R {
        SEGEN2_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Segment Enable (second Group)"]
    #[inline(always)]
    pub fn segen2(&mut self) -> SEGEN2_W {
        SEGEN2_W { w: self }
    }
}
