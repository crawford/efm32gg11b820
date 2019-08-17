#[doc = "Reader of register TFTPOLARITY"]
pub type R = crate::R<u32, super::TFTPOLARITY>;
#[doc = "Writer for register TFTPOLARITY"]
pub type W = crate::W<u32, super::TFTPOLARITY>;
#[doc = "Register TFTPOLARITY `reset()`'s with value 0"]
impl crate::ResetValue for super::TFTPOLARITY {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CSPOL`"]
pub type CSPOL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CSPOL`"]
pub struct CSPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> CSPOL_W<'a> {
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
#[doc = "Reader of field `DCLKPOL`"]
pub type DCLKPOL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DCLKPOL`"]
pub struct DCLKPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> DCLKPOL_W<'a> {
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
#[doc = "Reader of field `DATAENPOL`"]
pub type DATAENPOL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DATAENPOL`"]
pub struct DATAENPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> DATAENPOL_W<'a> {
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
#[doc = "Reader of field `HSYNCPOL`"]
pub type HSYNCPOL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HSYNCPOL`"]
pub struct HSYNCPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> HSYNCPOL_W<'a> {
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
#[doc = "Reader of field `VSYNCPOL`"]
pub type VSYNCPOL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VSYNCPOL`"]
pub struct VSYNCPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> VSYNCPOL_W<'a> {
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
impl R {
    #[doc = "Bit 0 - TFT Chip Select Polarity"]
    #[inline(always)]
    pub fn cspol(&self) -> CSPOL_R {
        CSPOL_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - TFT DCLK Polarity"]
    #[inline(always)]
    pub fn dclkpol(&self) -> DCLKPOL_R {
        DCLKPOL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - TFT DATAEN Polarity"]
    #[inline(always)]
    pub fn dataenpol(&self) -> DATAENPOL_R {
        DATAENPOL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Address Latch Polarity"]
    #[inline(always)]
    pub fn hsyncpol(&self) -> HSYNCPOL_R {
        HSYNCPOL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - VSYNC Polarity"]
    #[inline(always)]
    pub fn vsyncpol(&self) -> VSYNCPOL_R {
        VSYNCPOL_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TFT Chip Select Polarity"]
    #[inline(always)]
    pub fn cspol(&mut self) -> CSPOL_W {
        CSPOL_W { w: self }
    }
    #[doc = "Bit 1 - TFT DCLK Polarity"]
    #[inline(always)]
    pub fn dclkpol(&mut self) -> DCLKPOL_W {
        DCLKPOL_W { w: self }
    }
    #[doc = "Bit 2 - TFT DATAEN Polarity"]
    #[inline(always)]
    pub fn dataenpol(&mut self) -> DATAENPOL_W {
        DATAENPOL_W { w: self }
    }
    #[doc = "Bit 3 - Address Latch Polarity"]
    #[inline(always)]
    pub fn hsyncpol(&mut self) -> HSYNCPOL_W {
        HSYNCPOL_W { w: self }
    }
    #[doc = "Bit 4 - VSYNC Polarity"]
    #[inline(always)]
    pub fn vsyncpol(&mut self) -> VSYNCPOL_W {
        VSYNCPOL_W { w: self }
    }
}
