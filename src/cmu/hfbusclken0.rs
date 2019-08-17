#[doc = "Reader of register HFBUSCLKEN0"]
pub type R = crate::R<u32, super::HFBUSCLKEN0>;
#[doc = "Writer for register HFBUSCLKEN0"]
pub type W = crate::W<u32, super::HFBUSCLKEN0>;
#[doc = "Register HFBUSCLKEN0 `reset()`'s with value 0"]
impl crate::ResetValue for super::HFBUSCLKEN0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `LE`"]
pub type LE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LE`"]
pub struct LE_W<'a> {
    w: &'a mut W,
}
impl<'a> LE_W<'a> {
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
#[doc = "Reader of field `CRYPTO0`"]
pub type CRYPTO0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CRYPTO0`"]
pub struct CRYPTO0_W<'a> {
    w: &'a mut W,
}
impl<'a> CRYPTO0_W<'a> {
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
#[doc = "Reader of field `EBI`"]
pub type EBI_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EBI`"]
pub struct EBI_W<'a> {
    w: &'a mut W,
}
impl<'a> EBI_W<'a> {
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
#[doc = "Reader of field `ETH`"]
pub type ETH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ETH`"]
pub struct ETH_W<'a> {
    w: &'a mut W,
}
impl<'a> ETH_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `SDIO`"]
pub type SDIO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SDIO`"]
pub struct SDIO_W<'a> {
    w: &'a mut W,
}
impl<'a> SDIO_W<'a> {
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
#[doc = "Reader of field `GPIO`"]
pub type GPIO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO`"]
pub struct GPIO_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_W<'a> {
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
#[doc = "Reader of field `PRS`"]
pub type PRS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PRS`"]
pub struct PRS_W<'a> {
    w: &'a mut W,
}
impl<'a> PRS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `LDMA`"]
pub type LDMA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LDMA`"]
pub struct LDMA_W<'a> {
    w: &'a mut W,
}
impl<'a> LDMA_W<'a> {
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
#[doc = "Reader of field `GPCRC`"]
pub type GPCRC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPCRC`"]
pub struct GPCRC_W<'a> {
    w: &'a mut W,
}
impl<'a> GPCRC_W<'a> {
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
#[doc = "Reader of field `QSPI0`"]
pub type QSPI0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `QSPI0`"]
pub struct QSPI0_W<'a> {
    w: &'a mut W,
}
impl<'a> QSPI0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `USB`"]
pub type USB_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USB`"]
pub struct USB_W<'a> {
    w: &'a mut W,
}
impl<'a> USB_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Low Energy Peripheral Interface Clock Enable"]
    #[inline(always)]
    pub fn le(&self) -> LE_R {
        LE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Advanced Encryption Standard Accelerator Clock Enable"]
    #[inline(always)]
    pub fn crypto0(&self) -> CRYPTO0_R {
        CRYPTO0_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - External Bus Interface Clock Enable"]
    #[inline(always)]
    pub fn ebi(&self) -> EBI_R {
        EBI_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Ethernet Controller Clock Enable"]
    #[inline(always)]
    pub fn eth(&self) -> ETH_R {
        ETH_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - SDIO Controller Clock Enable"]
    #[inline(always)]
    pub fn sdio(&self) -> SDIO_R {
        SDIO_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - General purpose Input/Output Clock Enable"]
    #[inline(always)]
    pub fn gpio(&self) -> GPIO_R {
        GPIO_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Peripheral Reflex System Clock Enable"]
    #[inline(always)]
    pub fn prs(&self) -> PRS_R {
        PRS_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Linked Direct Memory Access Controller Clock Enable"]
    #[inline(always)]
    pub fn ldma(&self) -> LDMA_R {
        LDMA_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - General Purpose CRC Clock Enable"]
    #[inline(always)]
    pub fn gpcrc(&self) -> GPCRC_R {
        GPCRC_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Quad-SPI Clock Enable"]
    #[inline(always)]
    pub fn qspi0(&self) -> QSPI0_R {
        QSPI0_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Universal Serial Bus Interface Clock Enable"]
    #[inline(always)]
    pub fn usb(&self) -> USB_R {
        USB_R::new(((self.bits >> 10) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Low Energy Peripheral Interface Clock Enable"]
    #[inline(always)]
    pub fn le(&mut self) -> LE_W {
        LE_W { w: self }
    }
    #[doc = "Bit 1 - Advanced Encryption Standard Accelerator Clock Enable"]
    #[inline(always)]
    pub fn crypto0(&mut self) -> CRYPTO0_W {
        CRYPTO0_W { w: self }
    }
    #[doc = "Bit 2 - External Bus Interface Clock Enable"]
    #[inline(always)]
    pub fn ebi(&mut self) -> EBI_W {
        EBI_W { w: self }
    }
    #[doc = "Bit 3 - Ethernet Controller Clock Enable"]
    #[inline(always)]
    pub fn eth(&mut self) -> ETH_W {
        ETH_W { w: self }
    }
    #[doc = "Bit 4 - SDIO Controller Clock Enable"]
    #[inline(always)]
    pub fn sdio(&mut self) -> SDIO_W {
        SDIO_W { w: self }
    }
    #[doc = "Bit 5 - General purpose Input/Output Clock Enable"]
    #[inline(always)]
    pub fn gpio(&mut self) -> GPIO_W {
        GPIO_W { w: self }
    }
    #[doc = "Bit 6 - Peripheral Reflex System Clock Enable"]
    #[inline(always)]
    pub fn prs(&mut self) -> PRS_W {
        PRS_W { w: self }
    }
    #[doc = "Bit 7 - Linked Direct Memory Access Controller Clock Enable"]
    #[inline(always)]
    pub fn ldma(&mut self) -> LDMA_W {
        LDMA_W { w: self }
    }
    #[doc = "Bit 8 - General Purpose CRC Clock Enable"]
    #[inline(always)]
    pub fn gpcrc(&mut self) -> GPCRC_W {
        GPCRC_W { w: self }
    }
    #[doc = "Bit 9 - Quad-SPI Clock Enable"]
    #[inline(always)]
    pub fn qspi0(&mut self) -> QSPI0_W {
        QSPI0_W { w: self }
    }
    #[doc = "Bit 10 - Universal Serial Bus Interface Clock Enable"]
    #[inline(always)]
    pub fn usb(&mut self) -> USB_W {
        USB_W { w: self }
    }
}
