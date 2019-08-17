#[doc = "Reader of register ETMTSEVR"]
pub type R = crate::R<u32, super::ETMTSEVR>;
#[doc = "Writer for register ETMTSEVR"]
pub type W = crate::W<u32, super::ETMTSEVR>;
#[doc = "Register ETMTSEVR `reset()`'s with value 0"]
impl crate::ResetValue for super::ETMTSEVR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RESAEVT`"]
pub type RESAEVT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RESAEVT`"]
pub struct RESAEVT_W<'a> {
    w: &'a mut W,
}
impl<'a> RESAEVT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u32) & 0x7f);
        self.w
    }
}
#[doc = "Reader of field `RESBEVT`"]
pub type RESBEVT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RESBEVT`"]
pub struct RESBEVT_W<'a> {
    w: &'a mut W,
}
impl<'a> RESBEVT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 7)) | (((value as u32) & 0x7f) << 7);
        self.w
    }
}
#[doc = "Reader of field `ETMFCNEVT`"]
pub type ETMFCNEVT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ETMFCNEVT`"]
pub struct ETMFCNEVT_W<'a> {
    w: &'a mut W,
}
impl<'a> ETMFCNEVT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 14)) | (((value as u32) & 0x07) << 14);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - ETM Resource A Event"]
    #[inline(always)]
    pub fn resaevt(&self) -> RESAEVT_R {
        RESAEVT_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 7:13 - ETM Resource B Event"]
    #[inline(always)]
    pub fn resbevt(&self) -> RESBEVT_R {
        RESBEVT_R::new(((self.bits >> 7) & 0x7f) as u8)
    }
    #[doc = "Bits 14:16 - ETM Function Event"]
    #[inline(always)]
    pub fn etmfcnevt(&self) -> ETMFCNEVT_R {
        ETMFCNEVT_R::new(((self.bits >> 14) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - ETM Resource A Event"]
    #[inline(always)]
    pub fn resaevt(&mut self) -> RESAEVT_W {
        RESAEVT_W { w: self }
    }
    #[doc = "Bits 7:13 - ETM Resource B Event"]
    #[inline(always)]
    pub fn resbevt(&mut self) -> RESBEVT_W {
        RESBEVT_W { w: self }
    }
    #[doc = "Bits 14:16 - ETM Function Event"]
    #[inline(always)]
    pub fn etmfcnevt(&mut self) -> ETMFCNEVT_W {
        ETMFCNEVT_W { w: self }
    }
}
