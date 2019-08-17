#[doc = "Reader of register R5VDETCTRL"]
pub type R = crate::R<u32, super::R5VDETCTRL>;
#[doc = "Writer for register R5VDETCTRL"]
pub type W = crate::W<u32, super::R5VDETCTRL>;
#[doc = "Register R5VDETCTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::R5VDETCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `VREGIDETDIS`"]
pub type VREGIDETDIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VREGIDETDIS`"]
pub struct VREGIDETDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> VREGIDETDIS_W<'a> {
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
#[doc = "Reader of field `VBUSDETDIS`"]
pub type VBUSDETDIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VBUSDETDIS`"]
pub struct VBUSDETDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> VBUSDETDIS_W<'a> {
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
#[doc = "Reader of field `VREGODETDIS`"]
pub type VREGODETDIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VREGODETDIS`"]
pub struct VREGODETDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> VREGODETDIS_W<'a> {
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
impl R {
    #[doc = "Bit 0 - VREGI Detector Disable"]
    #[inline(always)]
    pub fn vregidetdis(&self) -> VREGIDETDIS_R {
        VREGIDETDIS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - VBUS Detector Disable"]
    #[inline(always)]
    pub fn vbusdetdis(&self) -> VBUSDETDIS_R {
        VBUSDETDIS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - VREGO Detector Disable"]
    #[inline(always)]
    pub fn vregodetdis(&self) -> VREGODETDIS_R {
        VREGODETDIS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - VREGI Detector Disable"]
    #[inline(always)]
    pub fn vregidetdis(&mut self) -> VREGIDETDIS_W {
        VREGIDETDIS_W { w: self }
    }
    #[doc = "Bit 1 - VBUS Detector Disable"]
    #[inline(always)]
    pub fn vbusdetdis(&mut self) -> VBUSDETDIS_W {
        VBUSDETDIS_W { w: self }
    }
    #[doc = "Bit 2 - VREGO Detector Disable"]
    #[inline(always)]
    pub fn vregodetdis(&mut self) -> VREGODETDIS_W {
        VREGODETDIS_W { w: self }
    }
}
