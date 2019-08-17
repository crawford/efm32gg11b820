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
#[doc = "Reader of field `FPIOC`"]
pub type FPIOC_R = crate::R<bool, bool>;
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
#[doc = "Reader of field `FPDZC`"]
pub type FPDZC_R = crate::R<bool, bool>;
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
#[doc = "Reader of field `FPUFC`"]
pub type FPUFC_R = crate::R<bool, bool>;
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
#[doc = "Reader of field `FPOFC`"]
pub type FPOFC_R = crate::R<bool, bool>;
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
#[doc = "Reader of field `FPIDC`"]
pub type FPIDC_R = crate::R<bool, bool>;
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
#[doc = "Reader of field `FPIXC`"]
pub type FPIXC_R = crate::R<bool, bool>;
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
impl R {
    #[doc = "Bit 0 - FPIOC Interrupt Enable"]
    #[inline(always)]
    pub fn fpioc(&self) -> FPIOC_R {
        FPIOC_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - FPDZC Interrupt Enable"]
    #[inline(always)]
    pub fn fpdzc(&self) -> FPDZC_R {
        FPDZC_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - FPUFC Interrupt Enable"]
    #[inline(always)]
    pub fn fpufc(&self) -> FPUFC_R {
        FPUFC_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - FPOFC Interrupt Enable"]
    #[inline(always)]
    pub fn fpofc(&self) -> FPOFC_R {
        FPOFC_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - FPIDC Interrupt Enable"]
    #[inline(always)]
    pub fn fpidc(&self) -> FPIDC_R {
        FPIDC_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - FPIXC Interrupt Enable"]
    #[inline(always)]
    pub fn fpixc(&self) -> FPIXC_R {
        FPIXC_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - FPIOC Interrupt Enable"]
    #[inline(always)]
    pub fn fpioc(&mut self) -> FPIOC_W {
        FPIOC_W { w: self }
    }
    #[doc = "Bit 1 - FPDZC Interrupt Enable"]
    #[inline(always)]
    pub fn fpdzc(&mut self) -> FPDZC_W {
        FPDZC_W { w: self }
    }
    #[doc = "Bit 2 - FPUFC Interrupt Enable"]
    #[inline(always)]
    pub fn fpufc(&mut self) -> FPUFC_W {
        FPUFC_W { w: self }
    }
    #[doc = "Bit 3 - FPOFC Interrupt Enable"]
    #[inline(always)]
    pub fn fpofc(&mut self) -> FPOFC_W {
        FPOFC_W { w: self }
    }
    #[doc = "Bit 4 - FPIDC Interrupt Enable"]
    #[inline(always)]
    pub fn fpidc(&mut self) -> FPIDC_W {
        FPIDC_W { w: self }
    }
    #[doc = "Bit 5 - FPIXC Interrupt Enable"]
    #[inline(always)]
    pub fn fpixc(&mut self) -> FPIXC_W {
        FPIXC_W { w: self }
    }
}
