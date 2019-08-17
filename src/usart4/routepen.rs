#[doc = "Reader of register ROUTEPEN"]
pub type R = crate::R<u32, super::ROUTEPEN>;
#[doc = "Writer for register ROUTEPEN"]
pub type W = crate::W<u32, super::ROUTEPEN>;
#[doc = "Register ROUTEPEN `reset()`'s with value 0"]
impl crate::ResetValue for super::ROUTEPEN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RXPEN`"]
pub type RXPEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXPEN`"]
pub struct RXPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RXPEN_W<'a> {
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
#[doc = "Reader of field `TXPEN`"]
pub type TXPEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXPEN`"]
pub struct TXPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TXPEN_W<'a> {
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
#[doc = "Reader of field `CSPEN`"]
pub type CSPEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CSPEN`"]
pub struct CSPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CSPEN_W<'a> {
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
#[doc = "Reader of field `CLKPEN`"]
pub type CLKPEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLKPEN`"]
pub struct CLKPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKPEN_W<'a> {
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
#[doc = "Reader of field `CTSPEN`"]
pub type CTSPEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CTSPEN`"]
pub struct CTSPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CTSPEN_W<'a> {
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
#[doc = "Reader of field `RTSPEN`"]
pub type RTSPEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTSPEN`"]
pub struct RTSPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RTSPEN_W<'a> {
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
    #[doc = "Bit 0 - RX Pin Enable"]
    #[inline(always)]
    pub fn rxpen(&self) -> RXPEN_R {
        RXPEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - TX Pin Enable"]
    #[inline(always)]
    pub fn txpen(&self) -> TXPEN_R {
        TXPEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - CS Pin Enable"]
    #[inline(always)]
    pub fn cspen(&self) -> CSPEN_R {
        CSPEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - CLK Pin Enable"]
    #[inline(always)]
    pub fn clkpen(&self) -> CLKPEN_R {
        CLKPEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - CTS Pin Enable"]
    #[inline(always)]
    pub fn ctspen(&self) -> CTSPEN_R {
        CTSPEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - RTS Pin Enable"]
    #[inline(always)]
    pub fn rtspen(&self) -> RTSPEN_R {
        RTSPEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RX Pin Enable"]
    #[inline(always)]
    pub fn rxpen(&mut self) -> RXPEN_W {
        RXPEN_W { w: self }
    }
    #[doc = "Bit 1 - TX Pin Enable"]
    #[inline(always)]
    pub fn txpen(&mut self) -> TXPEN_W {
        TXPEN_W { w: self }
    }
    #[doc = "Bit 2 - CS Pin Enable"]
    #[inline(always)]
    pub fn cspen(&mut self) -> CSPEN_W {
        CSPEN_W { w: self }
    }
    #[doc = "Bit 3 - CLK Pin Enable"]
    #[inline(always)]
    pub fn clkpen(&mut self) -> CLKPEN_W {
        CLKPEN_W { w: self }
    }
    #[doc = "Bit 4 - CTS Pin Enable"]
    #[inline(always)]
    pub fn ctspen(&mut self) -> CTSPEN_W {
        CTSPEN_W { w: self }
    }
    #[doc = "Bit 5 - RTS Pin Enable"]
    #[inline(always)]
    pub fn rtspen(&mut self) -> RTSPEN_W {
        RTSPEN_W { w: self }
    }
}
