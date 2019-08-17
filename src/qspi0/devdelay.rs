#[doc = "Reader of register DEVDELAY"]
pub type R = crate::R<u32, super::DEVDELAY>;
#[doc = "Writer for register DEVDELAY"]
pub type W = crate::W<u32, super::DEVDELAY>;
#[doc = "Register DEVDELAY `reset()`'s with value 0"]
impl crate::ResetValue for super::DEVDELAY {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DINIT`"]
pub type DINIT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DINIT`"]
pub struct DINIT_W<'a> {
    w: &'a mut W,
}
impl<'a> DINIT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `DAFTER`"]
pub type DAFTER_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DAFTER`"]
pub struct DAFTER_W<'a> {
    w: &'a mut W,
}
impl<'a> DAFTER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `DBTWN`"]
pub type DBTWN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DBTWN`"]
pub struct DBTWN_W<'a> {
    w: &'a mut W,
}
impl<'a> DBTWN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `DNSS`"]
pub type DNSS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DNSS`"]
pub struct DNSS_W<'a> {
    w: &'a mut W,
}
impl<'a> DNSS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Clock Delay for CS"]
    #[inline(always)]
    pub fn dinit(&self) -> DINIT_R {
        DINIT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Clock Delay for Last Transaction Bit"]
    #[inline(always)]
    pub fn dafter(&self) -> DAFTER_R {
        DAFTER_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Clock Delay Between Two Chip Selects"]
    #[inline(always)]
    pub fn dbtwn(&self) -> DBTWN_R {
        DBTWN_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Clock Delay for Chip Select Deassert"]
    #[inline(always)]
    pub fn dnss(&self) -> DNSS_R {
        DNSS_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Clock Delay for CS"]
    #[inline(always)]
    pub fn dinit(&mut self) -> DINIT_W {
        DINIT_W { w: self }
    }
    #[doc = "Bits 8:15 - Clock Delay for Last Transaction Bit"]
    #[inline(always)]
    pub fn dafter(&mut self) -> DAFTER_W {
        DAFTER_W { w: self }
    }
    #[doc = "Bits 16:23 - Clock Delay Between Two Chip Selects"]
    #[inline(always)]
    pub fn dbtwn(&mut self) -> DBTWN_W {
        DBTWN_W { w: self }
    }
    #[doc = "Bits 24:31 - Clock Delay for Chip Select Deassert"]
    #[inline(always)]
    pub fn dnss(&mut self) -> DNSS_W {
        DNSS_W { w: self }
    }
}
