#[doc = "Writer for register IFC"]
pub type W = crate::W<u32, super::IFC>;
#[doc = "Register IFC `reset()`'s with value 0"]
impl crate::ResetValue for super::IFC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
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
impl W {
    #[doc = "Bit 0 - Clear TXC Interrupt Flag"]
    #[inline(always)]
    pub fn txc(&mut self) -> TXC_W {
        TXC_W { w: self }
    }
    #[doc = "Bit 3 - Clear RXFULL Interrupt Flag"]
    #[inline(always)]
    pub fn rxfull(&mut self) -> RXFULL_W {
        RXFULL_W { w: self }
    }
    #[doc = "Bit 4 - Clear RXOF Interrupt Flag"]
    #[inline(always)]
    pub fn rxof(&mut self) -> RXOF_W {
        RXOF_W { w: self }
    }
    #[doc = "Bit 5 - Clear RXUF Interrupt Flag"]
    #[inline(always)]
    pub fn rxuf(&mut self) -> RXUF_W {
        RXUF_W { w: self }
    }
    #[doc = "Bit 6 - Clear TXOF Interrupt Flag"]
    #[inline(always)]
    pub fn txof(&mut self) -> TXOF_W {
        TXOF_W { w: self }
    }
    #[doc = "Bit 7 - Clear TXUF Interrupt Flag"]
    #[inline(always)]
    pub fn txuf(&mut self) -> TXUF_W {
        TXUF_W { w: self }
    }
    #[doc = "Bit 8 - Clear PERR Interrupt Flag"]
    #[inline(always)]
    pub fn perr(&mut self) -> PERR_W {
        PERR_W { w: self }
    }
    #[doc = "Bit 9 - Clear FERR Interrupt Flag"]
    #[inline(always)]
    pub fn ferr(&mut self) -> FERR_W {
        FERR_W { w: self }
    }
    #[doc = "Bit 10 - Clear MPAF Interrupt Flag"]
    #[inline(always)]
    pub fn mpaf(&mut self) -> MPAF_W {
        MPAF_W { w: self }
    }
    #[doc = "Bit 11 - Clear SSM Interrupt Flag"]
    #[inline(always)]
    pub fn ssm(&mut self) -> SSM_W {
        SSM_W { w: self }
    }
    #[doc = "Bit 12 - Clear CCF Interrupt Flag"]
    #[inline(always)]
    pub fn ccf(&mut self) -> CCF_W {
        CCF_W { w: self }
    }
    #[doc = "Bit 13 - Clear TXIDLE Interrupt Flag"]
    #[inline(always)]
    pub fn txidle(&mut self) -> TXIDLE_W {
        TXIDLE_W { w: self }
    }
    #[doc = "Bit 14 - Clear TCMP0 Interrupt Flag"]
    #[inline(always)]
    pub fn tcmp0(&mut self) -> TCMP0_W {
        TCMP0_W { w: self }
    }
    #[doc = "Bit 15 - Clear TCMP1 Interrupt Flag"]
    #[inline(always)]
    pub fn tcmp1(&mut self) -> TCMP1_W {
        TCMP1_W { w: self }
    }
    #[doc = "Bit 16 - Clear TCMP2 Interrupt Flag"]
    #[inline(always)]
    pub fn tcmp2(&mut self) -> TCMP2_W {
        TCMP2_W { w: self }
    }
}
