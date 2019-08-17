#[doc = "Reader of register COMP1"]
pub type R = crate::R<u32, super::COMP1>;
#[doc = "Writer for register COMP1"]
pub type W = crate::W<u32, super::COMP1>;
#[doc = "Register COMP1 `reset()`'s with value 0"]
impl crate::ResetValue for super::COMP1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `COMP1`"]
pub type COMP1_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `COMP1`"]
pub struct COMP1_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Compare Value 1"]
    #[inline(always)]
    pub fn comp1(&self) -> COMP1_R {
        COMP1_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Compare Value 1"]
    #[inline(always)]
    pub fn comp1(&mut self) -> COMP1_W {
        COMP1_W { w: self }
    }
}
