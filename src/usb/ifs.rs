#[doc = "Writer for register IFS"]
pub type W = crate::W<u32, super::IFS>;
#[doc = "Register IFS `reset()`'s with value 0"]
impl crate::ResetValue for super::IFS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
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
impl W {
    #[doc = "Bit 0 - Set VBUSDETH Interrupt Flag"]
    #[inline(always)]
    pub fn vbusdeth(&mut self) -> VBUSDETH_W {
        VBUSDETH_W { w: self }
    }
    #[doc = "Bit 1 - Set VBUSDETL Interrupt Flag"]
    #[inline(always)]
    pub fn vbusdetl(&mut self) -> VBUSDETL_W {
        VBUSDETL_W { w: self }
    }
    #[doc = "Bit 8 - Set ERR Interrupt Flag"]
    #[inline(always)]
    pub fn err(&mut self) -> ERR_W {
        ERR_W { w: self }
    }
    #[doc = "Bit 9 - Set DCD Interrupt Flag"]
    #[inline(always)]
    pub fn dcd(&mut self) -> DCD_W {
        DCD_W { w: self }
    }
    #[doc = "Bit 10 - Set PD Interrupt Flag"]
    #[inline(always)]
    pub fn pd(&mut self) -> PD_W {
        PD_W { w: self }
    }
    #[doc = "Bit 11 - Set SD Interrupt Flag"]
    #[inline(always)]
    pub fn sd(&mut self) -> SD_W {
        SD_W { w: self }
    }
}
