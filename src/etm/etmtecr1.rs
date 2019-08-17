#[doc = "Reader of register ETMTECR1"]
pub type R = crate::R<u32, super::ETMTECR1>;
#[doc = "Writer for register ETMTECR1"]
pub type W = crate::W<u32, super::ETMTECR1>;
#[doc = "Register ETMTECR1 `reset()`'s with value 0"]
impl crate::ResetValue for super::ETMTECR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADRCMP`"]
pub type ADRCMP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADRCMP`"]
pub struct ADRCMP_W<'a> {
    w: &'a mut W,
}
impl<'a> ADRCMP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `MEMMAP`"]
pub type MEMMAP_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `MEMMAP`"]
pub struct MEMMAP_W<'a> {
    w: &'a mut W,
}
impl<'a> MEMMAP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 8)) | (((value as u32) & 0xffff) << 8);
        self.w
    }
}
#[doc = "Reader of field `INCEXCTL`"]
pub type INCEXCTL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INCEXCTL`"]
pub struct INCEXCTL_W<'a> {
    w: &'a mut W,
}
impl<'a> INCEXCTL_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Reader of field `TCE`"]
pub type TCE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TCE`"]
pub struct TCE_W<'a> {
    w: &'a mut W,
}
impl<'a> TCE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Address Comparator"]
    #[inline(always)]
    pub fn adrcmp(&self) -> ADRCMP_R {
        ADRCMP_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:23 - Memmap"]
    #[inline(always)]
    pub fn memmap(&self) -> MEMMAP_R {
        MEMMAP_R::new(((self.bits >> 8) & 0xffff) as u16)
    }
    #[doc = "Bit 24 - Trace Include/Exclude Flag"]
    #[inline(always)]
    pub fn incexctl(&self) -> INCEXCTL_R {
        INCEXCTL_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Trace Control Enable"]
    #[inline(always)]
    pub fn tce(&self) -> TCE_R {
        TCE_R::new(((self.bits >> 25) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Address Comparator"]
    #[inline(always)]
    pub fn adrcmp(&mut self) -> ADRCMP_W {
        ADRCMP_W { w: self }
    }
    #[doc = "Bits 8:23 - Memmap"]
    #[inline(always)]
    pub fn memmap(&mut self) -> MEMMAP_W {
        MEMMAP_W { w: self }
    }
    #[doc = "Bit 24 - Trace Include/Exclude Flag"]
    #[inline(always)]
    pub fn incexctl(&mut self) -> INCEXCTL_W {
        INCEXCTL_W { w: self }
    }
    #[doc = "Bit 25 - Trace Control Enable"]
    #[inline(always)]
    pub fn tce(&mut self) -> TCE_W {
        TCE_W { w: self }
    }
}
