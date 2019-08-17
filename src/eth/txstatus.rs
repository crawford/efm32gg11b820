#[doc = "Reader of register TXSTATUS"]
pub type R = crate::R<u32, super::TXSTATUS>;
#[doc = "Writer for register TXSTATUS"]
pub type W = crate::W<u32, super::TXSTATUS>;
#[doc = "Register TXSTATUS `reset()`'s with value 0"]
impl crate::ResetValue for super::TXSTATUS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `USEDBITREAD`"]
pub type USEDBITREAD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USEDBITREAD`"]
pub struct USEDBITREAD_W<'a> {
    w: &'a mut W,
}
impl<'a> USEDBITREAD_W<'a> {
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
#[doc = "Reader of field `COLOCCRD`"]
pub type COLOCCRD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `COLOCCRD`"]
pub struct COLOCCRD_W<'a> {
    w: &'a mut W,
}
impl<'a> COLOCCRD_W<'a> {
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
#[doc = "Reader of field `RETRYLMTEXCD`"]
pub type RETRYLMTEXCD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RETRYLMTEXCD`"]
pub struct RETRYLMTEXCD_W<'a> {
    w: &'a mut W,
}
impl<'a> RETRYLMTEXCD_W<'a> {
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
#[doc = "Reader of field `TXGO`"]
pub type TXGO_R = crate::R<bool, bool>;
#[doc = "Reader of field `AMBAERR`"]
pub type AMBAERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AMBAERR`"]
pub struct AMBAERR_W<'a> {
    w: &'a mut W,
}
impl<'a> AMBAERR_W<'a> {
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
#[doc = "Reader of field `TXCMPLT`"]
pub type TXCMPLT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXCMPLT`"]
pub struct TXCMPLT_W<'a> {
    w: &'a mut W,
}
impl<'a> TXCMPLT_W<'a> {
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
#[doc = "Reader of field `TXUNDERRUN`"]
pub type TXUNDERRUN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXUNDERRUN`"]
pub struct TXUNDERRUN_W<'a> {
    w: &'a mut W,
}
impl<'a> TXUNDERRUN_W<'a> {
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
#[doc = "Reader of field `LATECOLOCCRD`"]
pub type LATECOLOCCRD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LATECOLOCCRD`"]
pub struct LATECOLOCCRD_W<'a> {
    w: &'a mut W,
}
impl<'a> LATECOLOCCRD_W<'a> {
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
#[doc = "Reader of field `RESPNOTOK`"]
pub type RESPNOTOK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RESPNOTOK`"]
pub struct RESPNOTOK_W<'a> {
    w: &'a mut W,
}
impl<'a> RESPNOTOK_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Used bit read"]
    #[inline(always)]
    pub fn usedbitread(&self) -> USEDBITREAD_R {
        USEDBITREAD_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Collision occurred"]
    #[inline(always)]
    pub fn coloccrd(&self) -> COLOCCRD_R {
        COLOCCRD_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Retry limit exceeded"]
    #[inline(always)]
    pub fn retrylmtexcd(&self) -> RETRYLMTEXCD_R {
        RETRYLMTEXCD_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Transmit go"]
    #[inline(always)]
    pub fn txgo(&self) -> TXGO_R {
        TXGO_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Transmit frame corruption due to AMBA (AHB) errors."]
    #[inline(always)]
    pub fn ambaerr(&self) -> AMBAERR_R {
        AMBAERR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Transmit complete"]
    #[inline(always)]
    pub fn txcmplt(&self) -> TXCMPLT_R {
        TXCMPLT_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Transmit under run"]
    #[inline(always)]
    pub fn txunderrun(&self) -> TXUNDERRUN_R {
        TXUNDERRUN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Late collision occurred"]
    #[inline(always)]
    pub fn latecoloccrd(&self) -> LATECOLOCCRD_R {
        LATECOLOCCRD_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - bresp/hresp not OK"]
    #[inline(always)]
    pub fn respnotok(&self) -> RESPNOTOK_R {
        RESPNOTOK_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Used bit read"]
    #[inline(always)]
    pub fn usedbitread(&mut self) -> USEDBITREAD_W {
        USEDBITREAD_W { w: self }
    }
    #[doc = "Bit 1 - Collision occurred"]
    #[inline(always)]
    pub fn coloccrd(&mut self) -> COLOCCRD_W {
        COLOCCRD_W { w: self }
    }
    #[doc = "Bit 2 - Retry limit exceeded"]
    #[inline(always)]
    pub fn retrylmtexcd(&mut self) -> RETRYLMTEXCD_W {
        RETRYLMTEXCD_W { w: self }
    }
    #[doc = "Bit 4 - Transmit frame corruption due to AMBA (AHB) errors."]
    #[inline(always)]
    pub fn ambaerr(&mut self) -> AMBAERR_W {
        AMBAERR_W { w: self }
    }
    #[doc = "Bit 5 - Transmit complete"]
    #[inline(always)]
    pub fn txcmplt(&mut self) -> TXCMPLT_W {
        TXCMPLT_W { w: self }
    }
    #[doc = "Bit 6 - Transmit under run"]
    #[inline(always)]
    pub fn txunderrun(&mut self) -> TXUNDERRUN_W {
        TXUNDERRUN_W { w: self }
    }
    #[doc = "Bit 7 - Late collision occurred"]
    #[inline(always)]
    pub fn latecoloccrd(&mut self) -> LATECOLOCCRD_W {
        LATECOLOCCRD_W { w: self }
    }
    #[doc = "Bit 8 - bresp/hresp not OK"]
    #[inline(always)]
    pub fn respnotok(&mut self) -> RESPNOTOK_W {
        RESPNOTOK_W { w: self }
    }
}
