#[doc = "Reader of register IEN"]
pub type R = crate::R<u32, super::IEN>;
#[doc = "Writer for register IEN"]
pub type W = crate::W<u32, super::IEN>;
#[doc = "Register IEN `reset()`'s with value 0"]
impl crate::ResetValue for super::IEN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TXC`"]
pub type TXC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXC`"]
pub struct TXC_W<'a> {
    w: &'a mut W,
}
impl<'a> TXC_W<'a> {
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
#[doc = "Reader of field `TXBL`"]
pub type TXBL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXBL`"]
pub struct TXBL_W<'a> {
    w: &'a mut W,
}
impl<'a> TXBL_W<'a> {
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
#[doc = "Reader of field `RXDATAV`"]
pub type RXDATAV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXDATAV`"]
pub struct RXDATAV_W<'a> {
    w: &'a mut W,
}
impl<'a> RXDATAV_W<'a> {
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
#[doc = "Reader of field `RXFULL`"]
pub type RXFULL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXFULL`"]
pub struct RXFULL_W<'a> {
    w: &'a mut W,
}
impl<'a> RXFULL_W<'a> {
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
#[doc = "Reader of field `RXOF`"]
pub type RXOF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXOF`"]
pub struct RXOF_W<'a> {
    w: &'a mut W,
}
impl<'a> RXOF_W<'a> {
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
#[doc = "Reader of field `RXUF`"]
pub type RXUF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXUF`"]
pub struct RXUF_W<'a> {
    w: &'a mut W,
}
impl<'a> RXUF_W<'a> {
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
#[doc = "Reader of field `TXOF`"]
pub type TXOF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXOF`"]
pub struct TXOF_W<'a> {
    w: &'a mut W,
}
impl<'a> TXOF_W<'a> {
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
#[doc = "Reader of field `TXUF`"]
pub type TXUF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXUF`"]
pub struct TXUF_W<'a> {
    w: &'a mut W,
}
impl<'a> TXUF_W<'a> {
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
#[doc = "Reader of field `PERR`"]
pub type PERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PERR`"]
pub struct PERR_W<'a> {
    w: &'a mut W,
}
impl<'a> PERR_W<'a> {
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
#[doc = "Reader of field `FERR`"]
pub type FERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FERR`"]
pub struct FERR_W<'a> {
    w: &'a mut W,
}
impl<'a> FERR_W<'a> {
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
#[doc = "Reader of field `MPAF`"]
pub type MPAF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MPAF`"]
pub struct MPAF_W<'a> {
    w: &'a mut W,
}
impl<'a> MPAF_W<'a> {
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
#[doc = "Reader of field `SSM`"]
pub type SSM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SSM`"]
pub struct SSM_W<'a> {
    w: &'a mut W,
}
impl<'a> SSM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `CCF`"]
pub type CCF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CCF`"]
pub struct CCF_W<'a> {
    w: &'a mut W,
}
impl<'a> CCF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `TXIDLE`"]
pub type TXIDLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXIDLE`"]
pub struct TXIDLE_W<'a> {
    w: &'a mut W,
}
impl<'a> TXIDLE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `TCMP0`"]
pub type TCMP0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TCMP0`"]
pub struct TCMP0_W<'a> {
    w: &'a mut W,
}
impl<'a> TCMP0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `TCMP1`"]
pub type TCMP1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TCMP1`"]
pub struct TCMP1_W<'a> {
    w: &'a mut W,
}
impl<'a> TCMP1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Reader of field `TCMP2`"]
pub type TCMP2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TCMP2`"]
pub struct TCMP2_W<'a> {
    w: &'a mut W,
}
impl<'a> TCMP2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - TXC Interrupt Enable"]
    #[inline(always)]
    pub fn txc(&self) -> TXC_R {
        TXC_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - TXBL Interrupt Enable"]
    #[inline(always)]
    pub fn txbl(&self) -> TXBL_R {
        TXBL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - RXDATAV Interrupt Enable"]
    #[inline(always)]
    pub fn rxdatav(&self) -> RXDATAV_R {
        RXDATAV_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - RXFULL Interrupt Enable"]
    #[inline(always)]
    pub fn rxfull(&self) -> RXFULL_R {
        RXFULL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - RXOF Interrupt Enable"]
    #[inline(always)]
    pub fn rxof(&self) -> RXOF_R {
        RXOF_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - RXUF Interrupt Enable"]
    #[inline(always)]
    pub fn rxuf(&self) -> RXUF_R {
        RXUF_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - TXOF Interrupt Enable"]
    #[inline(always)]
    pub fn txof(&self) -> TXOF_R {
        TXOF_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - TXUF Interrupt Enable"]
    #[inline(always)]
    pub fn txuf(&self) -> TXUF_R {
        TXUF_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - PERR Interrupt Enable"]
    #[inline(always)]
    pub fn perr(&self) -> PERR_R {
        PERR_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - FERR Interrupt Enable"]
    #[inline(always)]
    pub fn ferr(&self) -> FERR_R {
        FERR_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - MPAF Interrupt Enable"]
    #[inline(always)]
    pub fn mpaf(&self) -> MPAF_R {
        MPAF_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - SSM Interrupt Enable"]
    #[inline(always)]
    pub fn ssm(&self) -> SSM_R {
        SSM_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - CCF Interrupt Enable"]
    #[inline(always)]
    pub fn ccf(&self) -> CCF_R {
        CCF_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - TXIDLE Interrupt Enable"]
    #[inline(always)]
    pub fn txidle(&self) -> TXIDLE_R {
        TXIDLE_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - TCMP0 Interrupt Enable"]
    #[inline(always)]
    pub fn tcmp0(&self) -> TCMP0_R {
        TCMP0_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - TCMP1 Interrupt Enable"]
    #[inline(always)]
    pub fn tcmp1(&self) -> TCMP1_R {
        TCMP1_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - TCMP2 Interrupt Enable"]
    #[inline(always)]
    pub fn tcmp2(&self) -> TCMP2_R {
        TCMP2_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TXC Interrupt Enable"]
    #[inline(always)]
    pub fn txc(&mut self) -> TXC_W {
        TXC_W { w: self }
    }
    #[doc = "Bit 1 - TXBL Interrupt Enable"]
    #[inline(always)]
    pub fn txbl(&mut self) -> TXBL_W {
        TXBL_W { w: self }
    }
    #[doc = "Bit 2 - RXDATAV Interrupt Enable"]
    #[inline(always)]
    pub fn rxdatav(&mut self) -> RXDATAV_W {
        RXDATAV_W { w: self }
    }
    #[doc = "Bit 3 - RXFULL Interrupt Enable"]
    #[inline(always)]
    pub fn rxfull(&mut self) -> RXFULL_W {
        RXFULL_W { w: self }
    }
    #[doc = "Bit 4 - RXOF Interrupt Enable"]
    #[inline(always)]
    pub fn rxof(&mut self) -> RXOF_W {
        RXOF_W { w: self }
    }
    #[doc = "Bit 5 - RXUF Interrupt Enable"]
    #[inline(always)]
    pub fn rxuf(&mut self) -> RXUF_W {
        RXUF_W { w: self }
    }
    #[doc = "Bit 6 - TXOF Interrupt Enable"]
    #[inline(always)]
    pub fn txof(&mut self) -> TXOF_W {
        TXOF_W { w: self }
    }
    #[doc = "Bit 7 - TXUF Interrupt Enable"]
    #[inline(always)]
    pub fn txuf(&mut self) -> TXUF_W {
        TXUF_W { w: self }
    }
    #[doc = "Bit 8 - PERR Interrupt Enable"]
    #[inline(always)]
    pub fn perr(&mut self) -> PERR_W {
        PERR_W { w: self }
    }
    #[doc = "Bit 9 - FERR Interrupt Enable"]
    #[inline(always)]
    pub fn ferr(&mut self) -> FERR_W {
        FERR_W { w: self }
    }
    #[doc = "Bit 10 - MPAF Interrupt Enable"]
    #[inline(always)]
    pub fn mpaf(&mut self) -> MPAF_W {
        MPAF_W { w: self }
    }
    #[doc = "Bit 11 - SSM Interrupt Enable"]
    #[inline(always)]
    pub fn ssm(&mut self) -> SSM_W {
        SSM_W { w: self }
    }
    #[doc = "Bit 12 - CCF Interrupt Enable"]
    #[inline(always)]
    pub fn ccf(&mut self) -> CCF_W {
        CCF_W { w: self }
    }
    #[doc = "Bit 13 - TXIDLE Interrupt Enable"]
    #[inline(always)]
    pub fn txidle(&mut self) -> TXIDLE_W {
        TXIDLE_W { w: self }
    }
    #[doc = "Bit 14 - TCMP0 Interrupt Enable"]
    #[inline(always)]
    pub fn tcmp0(&mut self) -> TCMP0_W {
        TCMP0_W { w: self }
    }
    #[doc = "Bit 15 - TCMP1 Interrupt Enable"]
    #[inline(always)]
    pub fn tcmp1(&mut self) -> TCMP1_W {
        TCMP1_W { w: self }
    }
    #[doc = "Bit 16 - TCMP2 Interrupt Enable"]
    #[inline(always)]
    pub fn tcmp2(&mut self) -> TCMP2_W {
        TCMP2_W { w: self }
    }
}
