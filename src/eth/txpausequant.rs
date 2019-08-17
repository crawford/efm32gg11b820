#[doc = "Reader of register TXPAUSEQUANT"]
pub type R = crate::R<u32, super::TXPAUSEQUANT>;
#[doc = "Writer for register TXPAUSEQUANT"]
pub type W = crate::W<u32, super::TXPAUSEQUANT>;
#[doc = "Register TXPAUSEQUANT `reset()`'s with value 0xffff_ffff"]
impl crate::ResetValue for super::TXPAUSEQUANT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ffff
    }
}
#[doc = "Reader of field `QUANT`"]
pub type QUANT_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `QUANT`"]
pub struct QUANT_W<'a> {
    w: &'a mut W,
}
impl<'a> QUANT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Reader of field `QUANTP1`"]
pub type QUANTP1_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `QUANTP1`"]
pub struct QUANTP1_W<'a> {
    w: &'a mut W,
}
impl<'a> QUANTP1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Transmit pause quantum"]
    #[inline(always)]
    pub fn quant(&self) -> QUANT_R {
        QUANT_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Transmit pause quantum - written with the pause quantum value for pause frame transmission of priority 1."]
    #[inline(always)]
    pub fn quantp1(&self) -> QUANTP1_R {
        QUANTP1_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Transmit pause quantum"]
    #[inline(always)]
    pub fn quant(&mut self) -> QUANT_W {
        QUANT_W { w: self }
    }
    #[doc = "Bits 16:31 - Transmit pause quantum - written with the pause quantum value for pause frame transmission of priority 1."]
    #[inline(always)]
    pub fn quantp1(&mut self) -> QUANTP1_W {
        QUANTP1_W { w: self }
    }
}
