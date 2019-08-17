#[doc = "Reader of register AREGA"]
pub type R = crate::R<u32, super::AREGA>;
#[doc = "Writer for register AREGA"]
pub type W = crate::W<u32, super::AREGA>;
#[doc = "Register AREGA `reset()`'s with value 0"]
impl crate::ResetValue for super::AREGA {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `AREGA`"]
pub type AREGA_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `AREGA`"]
pub struct AREGA_W<'a> {
    w: &'a mut W,
}
impl<'a> AREGA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Animation Register a Data"]
    #[inline(always)]
    pub fn arega(&self) -> AREGA_R {
        AREGA_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Animation Register a Data"]
    #[inline(always)]
    pub fn arega(&mut self) -> AREGA_W {
        AREGA_W { w: self }
    }
}
