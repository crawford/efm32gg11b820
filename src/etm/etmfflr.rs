#[doc = "Reader of register ETMFFLR"]
pub type R = crate::R<u32, super::ETMFFLR>;
#[doc = "Writer for register ETMFFLR"]
pub type W = crate::W<u32, super::ETMFFLR>;
#[doc = "Register ETMFFLR `reset()`'s with value 0"]
impl crate::ResetValue for super::ETMFFLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `BYTENUM`"]
pub type BYTENUM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `BYTENUM`"]
pub struct BYTENUM_W<'a> {
    w: &'a mut W,
}
impl<'a> BYTENUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Bytes left in FIFO"]
    #[inline(always)]
    pub fn bytenum(&self) -> BYTENUM_R {
        BYTENUM_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Bytes left in FIFO"]
    #[inline(always)]
    pub fn bytenum(&mut self) -> BYTENUM_W {
        BYTENUM_W { w: self }
    }
}
