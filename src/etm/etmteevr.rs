#[doc = "Reader of register ETMTEEVR"]
pub type R = crate::R<u32, super::ETMTEEVR>;
#[doc = "Writer for register ETMTEEVR"]
pub type W = crate::W<u32, super::ETMTEEVR>;
#[doc = "Register ETMTEEVR `reset()`'s with value 0"]
impl crate::ResetValue for super::ETMTEEVR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RESA`"]
pub type RESA_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RESA`"]
pub struct RESA_W<'a> {
    w: &'a mut W,
}
impl<'a> RESA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u32) & 0x7f);
        self.w
    }
}
#[doc = "Reader of field `RESB`"]
pub type RESB_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RESB`"]
pub struct RESB_W<'a> {
    w: &'a mut W,
}
impl<'a> RESB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 7)) | (((value as u32) & 0x7f) << 7);
        self.w
    }
}
#[doc = "Reader of field `ETMFCNEN`"]
pub type ETMFCNEN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ETMFCNEN`"]
pub struct ETMFCNEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ETMFCNEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 14)) | (((value as u32) & 0x07) << 14);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - ETM Resource A Trace Enable"]
    #[inline(always)]
    pub fn resa(&self) -> RESA_R {
        RESA_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 7:13 - ETM Resource B Trace Enable"]
    #[inline(always)]
    pub fn resb(&self) -> RESB_R {
        RESB_R::new(((self.bits >> 7) & 0x7f) as u8)
    }
    #[doc = "Bits 14:16 - ETM Function Trace Enable"]
    #[inline(always)]
    pub fn etmfcnen(&self) -> ETMFCNEN_R {
        ETMFCNEN_R::new(((self.bits >> 14) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - ETM Resource A Trace Enable"]
    #[inline(always)]
    pub fn resa(&mut self) -> RESA_W {
        RESA_W { w: self }
    }
    #[doc = "Bits 7:13 - ETM Resource B Trace Enable"]
    #[inline(always)]
    pub fn resb(&mut self) -> RESB_W {
        RESB_W { w: self }
    }
    #[doc = "Bits 14:16 - ETM Function Trace Enable"]
    #[inline(always)]
    pub fn etmfcnen(&mut self) -> ETMFCNEN_W {
        ETMFCNEN_W { w: self }
    }
}
