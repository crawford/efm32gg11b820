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
#[doc = "Write proxy for field `FPIOC`"]
pub struct FPIOC_W<'a> {
    w: &'a mut W,
}
impl<'a> FPIOC_W<'a> {
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
#[doc = "Write proxy for field `FPDZC`"]
pub struct FPDZC_W<'a> {
    w: &'a mut W,
}
impl<'a> FPDZC_W<'a> {
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
#[doc = "Write proxy for field `FPUFC`"]
pub struct FPUFC_W<'a> {
    w: &'a mut W,
}
impl<'a> FPUFC_W<'a> {
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
#[doc = "Write proxy for field `FPOFC`"]
pub struct FPOFC_W<'a> {
    w: &'a mut W,
}
impl<'a> FPOFC_W<'a> {
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
#[doc = "Write proxy for field `FPIDC`"]
pub struct FPIDC_W<'a> {
    w: &'a mut W,
}
impl<'a> FPIDC_W<'a> {
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
#[doc = "Write proxy for field `FPIXC`"]
pub struct FPIXC_W<'a> {
    w: &'a mut W,
}
impl<'a> FPIXC_W<'a> {
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
impl W {
    #[doc = "Bit 0 - Set FPIOC Interrupt Flag"]
    #[inline(always)]
    pub fn fpioc(&mut self) -> FPIOC_W {
        FPIOC_W { w: self }
    }
    #[doc = "Bit 1 - Set FPDZC Interrupt Flag"]
    #[inline(always)]
    pub fn fpdzc(&mut self) -> FPDZC_W {
        FPDZC_W { w: self }
    }
    #[doc = "Bit 2 - Set FPUFC Interrupt Flag"]
    #[inline(always)]
    pub fn fpufc(&mut self) -> FPUFC_W {
        FPUFC_W { w: self }
    }
    #[doc = "Bit 3 - Set FPOFC Interrupt Flag"]
    #[inline(always)]
    pub fn fpofc(&mut self) -> FPOFC_W {
        FPOFC_W { w: self }
    }
    #[doc = "Bit 4 - Set FPIDC Interrupt Flag"]
    #[inline(always)]
    pub fn fpidc(&mut self) -> FPIDC_W {
        FPIDC_W { w: self }
    }
    #[doc = "Bit 5 - Set FPIXC Interrupt Flag"]
    #[inline(always)]
    pub fn fpixc(&mut self) -> FPIXC_W {
        FPIXC_W { w: self }
    }
}
