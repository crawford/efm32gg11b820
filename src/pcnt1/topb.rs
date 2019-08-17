#[doc = "Reader of register TOPB"]
pub type R = crate::R<u32, super::TOPB>;
#[doc = "Writer for register TOPB"]
pub type W = crate::W<u32, super::TOPB>;
#[doc = "Register TOPB `reset()`'s with value 0xff"]
impl crate::ResetValue for super::TOPB {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xff
    }
}
#[doc = "Reader of field `TOPB`"]
pub type TOPB_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `TOPB`"]
pub struct TOPB_W<'a> {
    w: &'a mut W,
}
impl<'a> TOPB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Counter Top Buffer"]
    #[inline(always)]
    pub fn topb(&self) -> TOPB_R {
        TOPB_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Counter Top Buffer"]
    #[inline(always)]
    pub fn topb(&mut self) -> TOPB_W {
        TOPB_W { w: self }
    }
}
