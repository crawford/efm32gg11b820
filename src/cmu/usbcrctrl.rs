#[doc = "Reader of register USBCRCTRL"]
pub type R = crate::R<u32, super::USBCRCTRL>;
#[doc = "Writer for register USBCRCTRL"]
pub type W = crate::W<u32, super::USBCRCTRL>;
#[doc = "Register USBCRCTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::USBCRCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `USBCREN`"]
pub type USBCREN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USBCREN`"]
pub struct USBCREN_W<'a> {
    w: &'a mut W,
}
impl<'a> USBCREN_W<'a> {
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
#[doc = "Reader of field `USBLSCRMD`"]
pub type USBLSCRMD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USBLSCRMD`"]
pub struct USBLSCRMD_W<'a> {
    w: &'a mut W,
}
impl<'a> USBLSCRMD_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Clock Recovery Enable"]
    #[inline(always)]
    pub fn usbcren(&self) -> USBCREN_R {
        USBCREN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Low Speed Clock Recovery Mode"]
    #[inline(always)]
    pub fn usblscrmd(&self) -> USBLSCRMD_R {
        USBLSCRMD_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Clock Recovery Enable"]
    #[inline(always)]
    pub fn usbcren(&mut self) -> USBCREN_W {
        USBCREN_W { w: self }
    }
    #[doc = "Bit 1 - Low Speed Clock Recovery Mode"]
    #[inline(always)]
    pub fn usblscrmd(&mut self) -> USBLSCRMD_W {
        USBLSCRMD_W { w: self }
    }
}
