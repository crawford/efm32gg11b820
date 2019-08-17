#[doc = "Reader of register AREGB"]
pub type R = crate::R<u32, super::AREGB>;
#[doc = "Writer for register AREGB"]
pub type W = crate::W<u32, super::AREGB>;
#[doc = "Register AREGB `reset()`'s with value 0"]
impl crate::ResetValue for super::AREGB {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `AREGB`"]
pub type AREGB_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `AREGB`"]
pub struct AREGB_W<'a> {
    w: &'a mut W,
}
impl<'a> AREGB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Animation Register B Data"]
    #[inline(always)]
    pub fn aregb(&self) -> AREGB_R {
        AREGB_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Animation Register B Data"]
    #[inline(always)]
    pub fn aregb(&mut self) -> AREGB_W {
        AREGB_W { w: self }
    }
}
