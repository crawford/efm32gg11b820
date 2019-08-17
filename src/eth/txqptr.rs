#[doc = "Reader of register TXQPTR"]
pub type R = crate::R<u32, super::TXQPTR>;
#[doc = "Writer for register TXQPTR"]
pub type W = crate::W<u32, super::TXQPTR>;
#[doc = "Register TXQPTR `reset()`'s with value 0"]
impl crate::ResetValue for super::TXQPTR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DMATXQPTR`"]
pub type DMATXQPTR_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `DMATXQPTR`"]
pub struct DMATXQPTR_W<'a> {
    w: &'a mut W,
}
impl<'a> DMATXQPTR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3fff_ffff << 2)) | (((value as u32) & 0x3fff_ffff) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bits 2:31 - Transmit buffer queue base address"]
    #[inline(always)]
    pub fn dmatxqptr(&self) -> DMATXQPTR_R {
        DMATXQPTR_R::new(((self.bits >> 2) & 0x3fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 2:31 - Transmit buffer queue base address"]
    #[inline(always)]
    pub fn dmatxqptr(&mut self) -> DMATXQPTR_W {
        DMATXQPTR_W { w: self }
    }
}
