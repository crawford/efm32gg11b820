#[doc = "Reader of register INDIRECTTRIGGERADDRRANGE"]
pub type R = crate::R<u32, super::INDIRECTTRIGGERADDRRANGE>;
#[doc = "Writer for register INDIRECTTRIGGERADDRRANGE"]
pub type W = crate::W<u32, super::INDIRECTTRIGGERADDRRANGE>;
#[doc = "Register INDIRECTTRIGGERADDRRANGE `reset()`'s with value 0x04"]
impl crate::ResetValue for super::INDIRECTTRIGGERADDRRANGE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x04
    }
}
#[doc = "Reader of field `INDRANGEWIDTH`"]
pub type INDRANGEWIDTH_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `INDRANGEWIDTH`"]
pub struct INDRANGEWIDTH_W<'a> {
    w: &'a mut W,
}
impl<'a> INDRANGEWIDTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Indirect Trigger Address Width"]
    #[inline(always)]
    pub fn indrangewidth(&self) -> INDRANGEWIDTH_R {
        INDRANGEWIDTH_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Indirect Trigger Address Width"]
    #[inline(always)]
    pub fn indrangewidth(&mut self) -> INDRANGEWIDTH_W {
        INDRANGEWIDTH_W { w: self }
    }
}
