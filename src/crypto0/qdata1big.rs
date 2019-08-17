#[doc = "Reader of register QDATA1BIG"]
pub type R = crate::R<u32, super::QDATA1BIG>;
#[doc = "Writer for register QDATA1BIG"]
pub type W = crate::W<u32, super::QDATA1BIG>;
#[doc = "Register QDATA1BIG `reset()`'s with value 0"]
impl crate::ResetValue for super::QDATA1BIG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `QDATA1BIG`"]
pub type QDATA1BIG_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `QDATA1BIG`"]
pub struct QDATA1BIG_W<'a> {
    w: &'a mut W,
}
impl<'a> QDATA1BIG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Quad Data 1 Big Endian Access"]
    #[inline(always)]
    pub fn qdata1big(&self) -> QDATA1BIG_R {
        QDATA1BIG_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Quad Data 1 Big Endian Access"]
    #[inline(always)]
    pub fn qdata1big(&mut self) -> QDATA1BIG_W {
        QDATA1BIG_W { w: self }
    }
}
