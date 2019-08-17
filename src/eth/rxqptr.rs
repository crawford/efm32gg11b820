#[doc = "Reader of register RXQPTR"]
pub type R = crate::R<u32, super::RXQPTR>;
#[doc = "Writer for register RXQPTR"]
pub type W = crate::W<u32, super::RXQPTR>;
#[doc = "Register RXQPTR `reset()`'s with value 0"]
impl crate::ResetValue for super::RXQPTR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DMARXQPTR`"]
pub type DMARXQPTR_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `DMARXQPTR`"]
pub struct DMARXQPTR_W<'a> {
    w: &'a mut W,
}
impl<'a> DMARXQPTR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3fff_ffff << 2)) | (((value as u32) & 0x3fff_ffff) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bits 2:31 - Receive buffer queue base address"]
    #[inline(always)]
    pub fn dmarxqptr(&self) -> DMARXQPTR_R {
        DMARXQPTR_R::new(((self.bits >> 2) & 0x3fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 2:31 - Receive buffer queue base address"]
    #[inline(always)]
    pub fn dmarxqptr(&mut self) -> DMARXQPTR_W {
        DMARXQPTR_W { w: self }
    }
}
