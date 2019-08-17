#[doc = "Reader of register IMOD"]
pub type R = crate::R<u32, super::IMOD>;
#[doc = "Writer for register IMOD"]
pub type W = crate::W<u32, super::IMOD>;
#[doc = "Register IMOD `reset()`'s with value 0"]
impl crate::ResetValue for super::IMOD {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RXINTMOD`"]
pub type RXINTMOD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RXINTMOD`"]
pub struct RXINTMOD_W<'a> {
    w: &'a mut W,
}
impl<'a> RXINTMOD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `TXINTMOD`"]
pub type TXINTMOD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TXINTMOD`"]
pub struct TXINTMOD_W<'a> {
    w: &'a mut W,
}
impl<'a> TXINTMOD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Count of 800ns periods before bit 1 is set in the interrupt status register after a frame is received"]
    #[inline(always)]
    pub fn rxintmod(&self) -> RXINTMOD_R {
        RXINTMOD_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Count of 800ns periods before bit 7 is set in the interrupt status register after a frame is transmitted"]
    #[inline(always)]
    pub fn txintmod(&self) -> TXINTMOD_R {
        TXINTMOD_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Count of 800ns periods before bit 1 is set in the interrupt status register after a frame is received"]
    #[inline(always)]
    pub fn rxintmod(&mut self) -> RXINTMOD_W {
        RXINTMOD_W { w: self }
    }
    #[doc = "Bits 16:23 - Count of 800ns periods before bit 7 is set in the interrupt status register after a frame is transmitted"]
    #[inline(always)]
    pub fn txintmod(&mut self) -> TXINTMOD_W {
        TXINTMOD_W { w: self }
    }
}
