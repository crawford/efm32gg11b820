#[doc = "Reader of register ETMTESSEICR"]
pub type R = crate::R<u32, super::ETMTESSEICR>;
#[doc = "Writer for register ETMTESSEICR"]
pub type W = crate::W<u32, super::ETMTESSEICR>;
#[doc = "Register ETMTESSEICR `reset()`'s with value 0"]
impl crate::ResetValue for super::ETMTESSEICR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `STARTRSEL`"]
pub type STARTRSEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `STARTRSEL`"]
pub struct STARTRSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> STARTRSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `STOPRSEL`"]
pub type STOPRSEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `STOPRSEL`"]
pub struct STOPRSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> STOPRSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Stop Resource Selection"]
    #[inline(always)]
    pub fn startrsel(&self) -> STARTRSEL_R {
        STARTRSEL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Stop Resource Selection"]
    #[inline(always)]
    pub fn stoprsel(&self) -> STOPRSEL_R {
        STOPRSEL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Stop Resource Selection"]
    #[inline(always)]
    pub fn startrsel(&mut self) -> STARTRSEL_W {
        STARTRSEL_W { w: self }
    }
    #[doc = "Bits 16:19 - Stop Resource Selection"]
    #[inline(always)]
    pub fn stoprsel(&mut self) -> STOPRSEL_W {
        STOPRSEL_W { w: self }
    }
}
