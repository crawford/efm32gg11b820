#[doc = "Reader of register GAHBCFG"]
pub type R = crate::R<u32, super::GAHBCFG>;
#[doc = "Writer for register GAHBCFG"]
pub type W = crate::W<u32, super::GAHBCFG>;
#[doc = "Register GAHBCFG `reset()`'s with value 0"]
impl crate::ResetValue for super::GAHBCFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `GLBLINTRMSK`"]
pub type GLBLINTRMSK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GLBLINTRMSK`"]
pub struct GLBLINTRMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> GLBLINTRMSK_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `HBSTLEN`"]
pub type HBSTLEN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HBSTLEN`"]
pub struct HBSTLEN_W<'a> {
    w: &'a mut W,
}
impl<'a> HBSTLEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 1)) | (((value as u32) & 0x0f) << 1);
        self.w
    }
}
#[doc = "Reader of field `DMAEN`"]
pub type DMAEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMAEN`"]
pub struct DMAEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `NPTXFEMPLVL`"]
pub type NPTXFEMPLVL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NPTXFEMPLVL`"]
pub struct NPTXFEMPLVL_W<'a> {
    w: &'a mut W,
}
impl<'a> NPTXFEMPLVL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `PTXFEMPLVL`"]
pub type PTXFEMPLVL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PTXFEMPLVL`"]
pub struct PTXFEMPLVL_W<'a> {
    w: &'a mut W,
}
impl<'a> PTXFEMPLVL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `REMMEMSUPP`"]
pub type REMMEMSUPP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `REMMEMSUPP`"]
pub struct REMMEMSUPP_W<'a> {
    w: &'a mut W,
}
impl<'a> REMMEMSUPP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Reader of field `NOTIALLDMAWRIT`"]
pub type NOTIALLDMAWRIT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NOTIALLDMAWRIT`"]
pub struct NOTIALLDMAWRIT_W<'a> {
    w: &'a mut W,
}
impl<'a> NOTIALLDMAWRIT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Reader of field `AHBSINGLE`"]
pub type AHBSINGLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AHBSINGLE`"]
pub struct AHBSINGLE_W<'a> {
    w: &'a mut W,
}
impl<'a> AHBSINGLE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Global Interrupt Mask"]
    #[inline(always)]
    pub fn glblintrmsk(&self) -> GLBLINTRMSK_R {
        GLBLINTRMSK_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:4 - Burst Length/Type"]
    #[inline(always)]
    pub fn hbstlen(&self) -> HBSTLEN_R {
        HBSTLEN_R::new(((self.bits >> 1) & 0x0f) as u8)
    }
    #[doc = "Bit 5 - DMA Enable"]
    #[inline(always)]
    pub fn dmaen(&self) -> DMAEN_R {
        DMAEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Non-Periodic TxFIFO Empty Level"]
    #[inline(always)]
    pub fn nptxfemplvl(&self) -> NPTXFEMPLVL_R {
        NPTXFEMPLVL_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Periodic TxFIFO Empty Level"]
    #[inline(always)]
    pub fn ptxfemplvl(&self) -> PTXFEMPLVL_R {
        PTXFEMPLVL_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Remote Memory Support"]
    #[inline(always)]
    pub fn remmemsupp(&self) -> REMMEMSUPP_R {
        REMMEMSUPP_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Notify All Dma Write Transactions"]
    #[inline(always)]
    pub fn notialldmawrit(&self) -> NOTIALLDMAWRIT_R {
        NOTIALLDMAWRIT_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - AHB Single Support"]
    #[inline(always)]
    pub fn ahbsingle(&self) -> AHBSINGLE_R {
        AHBSINGLE_R::new(((self.bits >> 23) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Global Interrupt Mask"]
    #[inline(always)]
    pub fn glblintrmsk(&mut self) -> GLBLINTRMSK_W {
        GLBLINTRMSK_W { w: self }
    }
    #[doc = "Bits 1:4 - Burst Length/Type"]
    #[inline(always)]
    pub fn hbstlen(&mut self) -> HBSTLEN_W {
        HBSTLEN_W { w: self }
    }
    #[doc = "Bit 5 - DMA Enable"]
    #[inline(always)]
    pub fn dmaen(&mut self) -> DMAEN_W {
        DMAEN_W { w: self }
    }
    #[doc = "Bit 7 - Non-Periodic TxFIFO Empty Level"]
    #[inline(always)]
    pub fn nptxfemplvl(&mut self) -> NPTXFEMPLVL_W {
        NPTXFEMPLVL_W { w: self }
    }
    #[doc = "Bit 8 - Periodic TxFIFO Empty Level"]
    #[inline(always)]
    pub fn ptxfemplvl(&mut self) -> PTXFEMPLVL_W {
        PTXFEMPLVL_W { w: self }
    }
    #[doc = "Bit 21 - Remote Memory Support"]
    #[inline(always)]
    pub fn remmemsupp(&mut self) -> REMMEMSUPP_W {
        REMMEMSUPP_W { w: self }
    }
    #[doc = "Bit 22 - Notify All Dma Write Transactions"]
    #[inline(always)]
    pub fn notialldmawrit(&mut self) -> NOTIALLDMAWRIT_W {
        NOTIALLDMAWRIT_W { w: self }
    }
    #[doc = "Bit 23 - AHB Single Support"]
    #[inline(always)]
    pub fn ahbsingle(&mut self) -> AHBSINGLE_W {
        AHBSINGLE_W { w: self }
    }
}
