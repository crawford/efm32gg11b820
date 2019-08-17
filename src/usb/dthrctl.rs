#[doc = "Reader of register DTHRCTL"]
pub type R = crate::R<u32, super::DTHRCTL>;
#[doc = "Writer for register DTHRCTL"]
pub type W = crate::W<u32, super::DTHRCTL>;
#[doc = "Register DTHRCTL `reset()`'s with value 0x0810_0020"]
impl crate::ResetValue for super::DTHRCTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0810_0020
    }
}
#[doc = "Reader of field `NONISOTHREN`"]
pub type NONISOTHREN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NONISOTHREN`"]
pub struct NONISOTHREN_W<'a> {
    w: &'a mut W,
}
impl<'a> NONISOTHREN_W<'a> {
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
#[doc = "Reader of field `ISOTHREN`"]
pub type ISOTHREN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ISOTHREN`"]
pub struct ISOTHREN_W<'a> {
    w: &'a mut W,
}
impl<'a> ISOTHREN_W<'a> {
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
#[doc = "Reader of field `TXTHRLEN`"]
pub type TXTHRLEN_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `TXTHRLEN`"]
pub struct TXTHRLEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TXTHRLEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01ff << 2)) | (((value as u32) & 0x01ff) << 2);
        self.w
    }
}
#[doc = "Reader of field `AHBTHRRATIO`"]
pub type AHBTHRRATIO_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `AHBTHRRATIO`"]
pub struct AHBTHRRATIO_W<'a> {
    w: &'a mut W,
}
impl<'a> AHBTHRRATIO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 11)) | (((value as u32) & 0x03) << 11);
        self.w
    }
}
#[doc = "Reader of field `RXTHREN`"]
pub type RXTHREN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXTHREN`"]
pub struct RXTHREN_W<'a> {
    w: &'a mut W,
}
impl<'a> RXTHREN_W<'a> {
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
#[doc = "Reader of field `RXTHRLEN`"]
pub type RXTHRLEN_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `RXTHRLEN`"]
pub struct RXTHRLEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RXTHRLEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01ff << 17)) | (((value as u32) & 0x01ff) << 17);
        self.w
    }
}
#[doc = "Reader of field `ARBPRKEN`"]
pub type ARBPRKEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ARBPRKEN`"]
pub struct ARBPRKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ARBPRKEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Non-ISO IN Endpoints Threshold Enable"]
    #[inline(always)]
    pub fn nonisothren(&self) -> NONISOTHREN_R {
        NONISOTHREN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - ISO IN Endpoints Threshold Enable"]
    #[inline(always)]
    pub fn isothren(&self) -> ISOTHREN_R {
        ISOTHREN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 2:10 - Transmit Threshold Length"]
    #[inline(always)]
    pub fn txthrlen(&self) -> TXTHRLEN_R {
        TXTHRLEN_R::new(((self.bits >> 2) & 0x01ff) as u16)
    }
    #[doc = "Bits 11:12 - AHB Threshold Ratio"]
    #[inline(always)]
    pub fn ahbthrratio(&self) -> AHBTHRRATIO_R {
        AHBTHRRATIO_R::new(((self.bits >> 11) & 0x03) as u8)
    }
    #[doc = "Bit 16 - Receive Threshold Enable"]
    #[inline(always)]
    pub fn rxthren(&self) -> RXTHREN_R {
        RXTHREN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 17:25 - Receive Threshold Length"]
    #[inline(always)]
    pub fn rxthrlen(&self) -> RXTHRLEN_R {
        RXTHRLEN_R::new(((self.bits >> 17) & 0x01ff) as u16)
    }
    #[doc = "Bit 27 - Arbiter Parking Enable"]
    #[inline(always)]
    pub fn arbprken(&self) -> ARBPRKEN_R {
        ARBPRKEN_R::new(((self.bits >> 27) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Non-ISO IN Endpoints Threshold Enable"]
    #[inline(always)]
    pub fn nonisothren(&mut self) -> NONISOTHREN_W {
        NONISOTHREN_W { w: self }
    }
    #[doc = "Bit 1 - ISO IN Endpoints Threshold Enable"]
    #[inline(always)]
    pub fn isothren(&mut self) -> ISOTHREN_W {
        ISOTHREN_W { w: self }
    }
    #[doc = "Bits 2:10 - Transmit Threshold Length"]
    #[inline(always)]
    pub fn txthrlen(&mut self) -> TXTHRLEN_W {
        TXTHRLEN_W { w: self }
    }
    #[doc = "Bits 11:12 - AHB Threshold Ratio"]
    #[inline(always)]
    pub fn ahbthrratio(&mut self) -> AHBTHRRATIO_W {
        AHBTHRRATIO_W { w: self }
    }
    #[doc = "Bit 16 - Receive Threshold Enable"]
    #[inline(always)]
    pub fn rxthren(&mut self) -> RXTHREN_W {
        RXTHREN_W { w: self }
    }
    #[doc = "Bits 17:25 - Receive Threshold Length"]
    #[inline(always)]
    pub fn rxthrlen(&mut self) -> RXTHRLEN_W {
        RXTHRLEN_W { w: self }
    }
    #[doc = "Bit 27 - Arbiter Parking Enable"]
    #[inline(always)]
    pub fn arbprken(&mut self) -> ARBPRKEN_W {
        ARBPRKEN_W { w: self }
    }
}
