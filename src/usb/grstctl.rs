#[doc = "Reader of register GRSTCTL"]
pub type R = crate::R<u32, super::GRSTCTL>;
#[doc = "Writer for register GRSTCTL"]
pub type W = crate::W<u32, super::GRSTCTL>;
#[doc = "Register GRSTCTL `reset()`'s with value 0x8000_0000"]
impl crate::ResetValue for super::GRSTCTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x8000_0000
    }
}
#[doc = "Reader of field `CSFTRST`"]
pub type CSFTRST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CSFTRST`"]
pub struct CSFTRST_W<'a> {
    w: &'a mut W,
}
impl<'a> CSFTRST_W<'a> {
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
#[doc = "Reader of field `PIUFSSFTRST`"]
pub type PIUFSSFTRST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PIUFSSFTRST`"]
pub struct PIUFSSFTRST_W<'a> {
    w: &'a mut W,
}
impl<'a> PIUFSSFTRST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `FRMCNTRRST`"]
pub type FRMCNTRRST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FRMCNTRRST`"]
pub struct FRMCNTRRST_W<'a> {
    w: &'a mut W,
}
impl<'a> FRMCNTRRST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `RXFFLSH`"]
pub type RXFFLSH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXFFLSH`"]
pub struct RXFFLSH_W<'a> {
    w: &'a mut W,
}
impl<'a> RXFFLSH_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `TXFFLSH`"]
pub type TXFFLSH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXFFLSH`"]
pub struct TXFFLSH_W<'a> {
    w: &'a mut W,
}
impl<'a> TXFFLSH_W<'a> {
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
#[doc = "Reader of field `TXFNUM`"]
pub type TXFNUM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TXFNUM`"]
pub struct TXFNUM_W<'a> {
    w: &'a mut W,
}
impl<'a> TXFNUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 6)) | (((value as u32) & 0x1f) << 6);
        self.w
    }
}
#[doc = "Reader of field `DMAREQ`"]
pub type DMAREQ_R = crate::R<bool, bool>;
#[doc = "Reader of field `AHBIDLE`"]
pub type AHBIDLE_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Core Soft Reset"]
    #[inline(always)]
    pub fn csftrst(&self) -> CSFTRST_R {
        CSFTRST_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - PIU FS Dedicated Controller Soft Reset"]
    #[inline(always)]
    pub fn piufssftrst(&self) -> PIUFSSFTRST_R {
        PIUFSSFTRST_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Host Frame Counter Reset"]
    #[inline(always)]
    pub fn frmcntrrst(&self) -> FRMCNTRRST_R {
        FRMCNTRRST_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4 - RxFIFO Flush"]
    #[inline(always)]
    pub fn rxfflsh(&self) -> RXFFLSH_R {
        RXFFLSH_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - TxFIFO Flush"]
    #[inline(always)]
    pub fn txfflsh(&self) -> TXFFLSH_R {
        TXFFLSH_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 6:10 - TxFIFO Number"]
    #[inline(always)]
    pub fn txfnum(&self) -> TXFNUM_R {
        TXFNUM_R::new(((self.bits >> 6) & 0x1f) as u8)
    }
    #[doc = "Bit 30 - DMA Request Signal"]
    #[inline(always)]
    pub fn dmareq(&self) -> DMAREQ_R {
        DMAREQ_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - AHB Master Idle"]
    #[inline(always)]
    pub fn ahbidle(&self) -> AHBIDLE_R {
        AHBIDLE_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Core Soft Reset"]
    #[inline(always)]
    pub fn csftrst(&mut self) -> CSFTRST_W {
        CSFTRST_W { w: self }
    }
    #[doc = "Bit 1 - PIU FS Dedicated Controller Soft Reset"]
    #[inline(always)]
    pub fn piufssftrst(&mut self) -> PIUFSSFTRST_W {
        PIUFSSFTRST_W { w: self }
    }
    #[doc = "Bit 2 - Host Frame Counter Reset"]
    #[inline(always)]
    pub fn frmcntrrst(&mut self) -> FRMCNTRRST_W {
        FRMCNTRRST_W { w: self }
    }
    #[doc = "Bit 4 - RxFIFO Flush"]
    #[inline(always)]
    pub fn rxfflsh(&mut self) -> RXFFLSH_W {
        RXFFLSH_W { w: self }
    }
    #[doc = "Bit 5 - TxFIFO Flush"]
    #[inline(always)]
    pub fn txfflsh(&mut self) -> TXFFLSH_W {
        TXFFLSH_W { w: self }
    }
    #[doc = "Bits 6:10 - TxFIFO Number"]
    #[inline(always)]
    pub fn txfnum(&mut self) -> TXFNUM_W {
        TXFNUM_W { w: self }
    }
}
