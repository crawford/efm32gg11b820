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
#[doc = "Reader of field `VBUSDETH`"]
pub type VBUSDETH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VBUSDETH`"]
pub struct VBUSDETH_W<'a> {
    w: &'a mut W,
}
impl<'a> VBUSDETH_W<'a> {
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
#[doc = "Reader of field `VBUSDETL`"]
pub type VBUSDETL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VBUSDETL`"]
pub struct VBUSDETL_W<'a> {
    w: &'a mut W,
}
impl<'a> VBUSDETL_W<'a> {
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
#[doc = "Reader of field `ERR`"]
pub type ERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ERR`"]
pub struct ERR_W<'a> {
    w: &'a mut W,
}
impl<'a> ERR_W<'a> {
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
#[doc = "Reader of field `DCD`"]
pub type DCD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DCD`"]
pub struct DCD_W<'a> {
    w: &'a mut W,
}
impl<'a> DCD_W<'a> {
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
#[doc = "Reader of field `PD`"]
pub type PD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PD`"]
pub struct PD_W<'a> {
    w: &'a mut W,
}
impl<'a> PD_W<'a> {
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
#[doc = "Reader of field `SD`"]
pub type SD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SD`"]
pub struct SD_W<'a> {
    w: &'a mut W,
}
impl<'a> SD_W<'a> {
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
impl R {
    #[doc = "Bit 0 - VBUSDETH Interrupt Enable"]
    #[inline(always)]
    pub fn vbusdeth(&self) -> VBUSDETH_R {
        VBUSDETH_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - VBUSDETL Interrupt Enable"]
    #[inline(always)]
    pub fn vbusdetl(&self) -> VBUSDETL_R {
        VBUSDETL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 8 - ERR Interrupt Enable"]
    #[inline(always)]
    pub fn err(&self) -> ERR_R {
        ERR_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - DCD Interrupt Enable"]
    #[inline(always)]
    pub fn dcd(&self) -> DCD_R {
        DCD_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - PD Interrupt Enable"]
    #[inline(always)]
    pub fn pd(&self) -> PD_R {
        PD_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - SD Interrupt Enable"]
    #[inline(always)]
    pub fn sd(&self) -> SD_R {
        SD_R::new(((self.bits >> 11) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - VBUSDETH Interrupt Enable"]
    #[inline(always)]
    pub fn vbusdeth(&mut self) -> VBUSDETH_W {
        VBUSDETH_W { w: self }
    }
    #[doc = "Bit 1 - VBUSDETL Interrupt Enable"]
    #[inline(always)]
    pub fn vbusdetl(&mut self) -> VBUSDETL_W {
        VBUSDETL_W { w: self }
    }
    #[doc = "Bit 8 - ERR Interrupt Enable"]
    #[inline(always)]
    pub fn err(&mut self) -> ERR_W {
        ERR_W { w: self }
    }
    #[doc = "Bit 9 - DCD Interrupt Enable"]
    #[inline(always)]
    pub fn dcd(&mut self) -> DCD_W {
        DCD_W { w: self }
    }
    #[doc = "Bit 10 - PD Interrupt Enable"]
    #[inline(always)]
    pub fn pd(&mut self) -> PD_W {
        PD_W { w: self }
    }
    #[doc = "Bit 11 - SD Interrupt Enable"]
    #[inline(always)]
    pub fn sd(&mut self) -> SD_W {
        SD_W { w: self }
    }
}
