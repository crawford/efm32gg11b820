#[doc = "Reader of register INPUTDATAHWORD"]
pub type R = crate::R<u32, super::INPUTDATAHWORD>;
#[doc = "Writer for register INPUTDATAHWORD"]
pub type W = crate::W<u32, super::INPUTDATAHWORD>;
#[doc = "Register INPUTDATAHWORD `reset()`'s with value 0"]
impl crate::ResetValue for super::INPUTDATAHWORD {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `INPUTDATAHWORD`"]
pub type INPUTDATAHWORD_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `INPUTDATAHWORD`"]
pub struct INPUTDATAHWORD_W<'a> {
    w: &'a mut W,
}
impl<'a> INPUTDATAHWORD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Input Data for 16-bit"]
    #[inline(always)]
    pub fn inputdatahword(&self) -> INPUTDATAHWORD_R {
        INPUTDATAHWORD_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Input Data for 16-bit"]
    #[inline(always)]
    pub fn inputdatahword(&mut self) -> INPUTDATAHWORD_W {
        INPUTDATAHWORD_W { w: self }
    }
}
