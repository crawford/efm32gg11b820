#[doc = "Reader of register SEQCTRLB"]
pub type R = crate::R<u32, super::SEQCTRLB>;
#[doc = "Writer for register SEQCTRLB"]
pub type W = crate::W<u32, super::SEQCTRLB>;
#[doc = "Register SEQCTRLB `reset()`'s with value 0"]
impl crate::ResetValue for super::SEQCTRLB {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `LENGTHB`"]
pub type LENGTHB_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `LENGTHB`"]
pub struct LENGTHB_W<'a> {
    w: &'a mut W,
}
impl<'a> LENGTHB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3fff) | ((value as u32) & 0x3fff);
        self.w
    }
}
#[doc = "Reader of field `DMA0PRESB`"]
pub type DMA0PRESB_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA0PRESB`"]
pub struct DMA0PRESB_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA0PRESB_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Reader of field `DMA1PRESB`"]
pub type DMA1PRESB_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA1PRESB`"]
pub struct DMA1PRESB_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA1PRESB_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:13 - Buffer Length B in Bytes"]
    #[inline(always)]
    pub fn lengthb(&self) -> LENGTHB_R {
        LENGTHB_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bit 28 - DMA0 Preserve B"]
    #[inline(always)]
    pub fn dma0presb(&self) -> DMA0PRESB_R {
        DMA0PRESB_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - DMA1 Preserve B"]
    #[inline(always)]
    pub fn dma1presb(&self) -> DMA1PRESB_R {
        DMA1PRESB_R::new(((self.bits >> 29) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:13 - Buffer Length B in Bytes"]
    #[inline(always)]
    pub fn lengthb(&mut self) -> LENGTHB_W {
        LENGTHB_W { w: self }
    }
    #[doc = "Bit 28 - DMA0 Preserve B"]
    #[inline(always)]
    pub fn dma0presb(&mut self) -> DMA0PRESB_W {
        DMA0PRESB_W { w: self }
    }
    #[doc = "Bit 29 - DMA1 Preserve B"]
    #[inline(always)]
    pub fn dma1presb(&mut self) -> DMA1PRESB_W {
        DMA1PRESB_W { w: self }
    }
}
