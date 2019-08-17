#[doc = "Reader of register MIR1_CMDMASK"]
pub type R = crate::R<u32, super::MIR1_CMDMASK>;
#[doc = "Writer for register MIR1_CMDMASK"]
pub type W = crate::W<u32, super::MIR1_CMDMASK>;
#[doc = "Register MIR1_CMDMASK `reset()`'s with value 0"]
impl crate::ResetValue for super::MIR1_CMDMASK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DATAB`"]
pub type DATAB_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DATAB`"]
pub struct DATAB_W<'a> {
    w: &'a mut W,
}
impl<'a> DATAB_W<'a> {
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
#[doc = "Reader of field `DATAA`"]
pub type DATAA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DATAA`"]
pub struct DATAA_W<'a> {
    w: &'a mut W,
}
impl<'a> DATAA_W<'a> {
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
#[doc = "Reader of field `TXRQSTNEWDAT`"]
pub type TXRQSTNEWDAT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXRQSTNEWDAT`"]
pub struct TXRQSTNEWDAT_W<'a> {
    w: &'a mut W,
}
impl<'a> TXRQSTNEWDAT_W<'a> {
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
#[doc = "Reader of field `CLRINTPND`"]
pub type CLRINTPND_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLRINTPND`"]
pub struct CLRINTPND_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRINTPND_W<'a> {
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
#[doc = "Reader of field `CONTROL`"]
pub type CONTROL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CONTROL`"]
pub struct CONTROL_W<'a> {
    w: &'a mut W,
}
impl<'a> CONTROL_W<'a> {
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
#[doc = "Reader of field `ARBACC`"]
pub type ARBACC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ARBACC`"]
pub struct ARBACC_W<'a> {
    w: &'a mut W,
}
impl<'a> ARBACC_W<'a> {
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
#[doc = "Reader of field `MASKACC`"]
pub type MASKACC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MASKACC`"]
pub struct MASKACC_W<'a> {
    w: &'a mut W,
}
impl<'a> MASKACC_W<'a> {
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
#[doc = "Reader of field `WRRD`"]
pub type WRRD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WRRD`"]
pub struct WRRD_W<'a> {
    w: &'a mut W,
}
impl<'a> WRRD_W<'a> {
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
impl R {
    #[doc = "Bit 0 - CC Channel Mode"]
    #[inline(always)]
    pub fn datab(&self) -> DATAB_R {
        DATAB_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Access Data Bytes 0-3"]
    #[inline(always)]
    pub fn dataa(&self) -> DATAA_R {
        DATAA_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Transmission Request Bit/ New Data Bit"]
    #[inline(always)]
    pub fn txrqstnewdat(&self) -> TXRQSTNEWDAT_R {
        TXRQSTNEWDAT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Clear Interrupt Pending Bit"]
    #[inline(always)]
    pub fn clrintpnd(&self) -> CLRINTPND_R {
        CLRINTPND_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Access Control Bits"]
    #[inline(always)]
    pub fn control(&self) -> CONTROL_R {
        CONTROL_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Access Arbitration Bits"]
    #[inline(always)]
    pub fn arbacc(&self) -> ARBACC_R {
        ARBACC_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Access Mask Bits"]
    #[inline(always)]
    pub fn maskacc(&self) -> MASKACC_R {
        MASKACC_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Write/Read RAM"]
    #[inline(always)]
    pub fn wrrd(&self) -> WRRD_R {
        WRRD_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CC Channel Mode"]
    #[inline(always)]
    pub fn datab(&mut self) -> DATAB_W {
        DATAB_W { w: self }
    }
    #[doc = "Bit 1 - Access Data Bytes 0-3"]
    #[inline(always)]
    pub fn dataa(&mut self) -> DATAA_W {
        DATAA_W { w: self }
    }
    #[doc = "Bit 2 - Transmission Request Bit/ New Data Bit"]
    #[inline(always)]
    pub fn txrqstnewdat(&mut self) -> TXRQSTNEWDAT_W {
        TXRQSTNEWDAT_W { w: self }
    }
    #[doc = "Bit 3 - Clear Interrupt Pending Bit"]
    #[inline(always)]
    pub fn clrintpnd(&mut self) -> CLRINTPND_W {
        CLRINTPND_W { w: self }
    }
    #[doc = "Bit 4 - Access Control Bits"]
    #[inline(always)]
    pub fn control(&mut self) -> CONTROL_W {
        CONTROL_W { w: self }
    }
    #[doc = "Bit 5 - Access Arbitration Bits"]
    #[inline(always)]
    pub fn arbacc(&mut self) -> ARBACC_W {
        ARBACC_W { w: self }
    }
    #[doc = "Bit 6 - Access Mask Bits"]
    #[inline(always)]
    pub fn maskacc(&mut self) -> MASKACC_W {
        MASKACC_W { w: self }
    }
    #[doc = "Bit 7 - Write/Read RAM"]
    #[inline(always)]
    pub fn wrrd(&mut self) -> WRRD_W {
        WRRD_W { w: self }
    }
}
