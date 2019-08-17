#[doc = "Reader of register TXDATAX"]
pub type R = crate::R<u32, super::TXDATAX>;
#[doc = "Writer for register TXDATAX"]
pub type W = crate::W<u32, super::TXDATAX>;
#[doc = "Register TXDATAX `reset()`'s with value 0"]
impl crate::ResetValue for super::TXDATAX {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TXDATA`"]
pub type TXDATA_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `TXDATA`"]
pub struct TXDATA_W<'a> {
    w: &'a mut W,
}
impl<'a> TXDATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01ff) | ((value as u32) & 0x01ff);
        self.w
    }
}
#[doc = "Reader of field `TXBREAK`"]
pub type TXBREAK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXBREAK`"]
pub struct TXBREAK_W<'a> {
    w: &'a mut W,
}
impl<'a> TXBREAK_W<'a> {
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
#[doc = "Reader of field `TXDISAT`"]
pub type TXDISAT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXDISAT`"]
pub struct TXDISAT_W<'a> {
    w: &'a mut W,
}
impl<'a> TXDISAT_W<'a> {
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
#[doc = "Reader of field `RXENAT`"]
pub type RXENAT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXENAT`"]
pub struct RXENAT_W<'a> {
    w: &'a mut W,
}
impl<'a> RXENAT_W<'a> {
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
impl R {
    #[doc = "Bits 0:8 - TX Data"]
    #[inline(always)]
    pub fn txdata(&self) -> TXDATA_R {
        TXDATA_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bit 13 - Transmit Data as Break"]
    #[inline(always)]
    pub fn txbreak(&self) -> TXBREAK_R {
        TXBREAK_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Disable TX After Transmission"]
    #[inline(always)]
    pub fn txdisat(&self) -> TXDISAT_R {
        TXDISAT_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Enable RX After Transmission"]
    #[inline(always)]
    pub fn rxenat(&self) -> RXENAT_R {
        RXENAT_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:8 - TX Data"]
    #[inline(always)]
    pub fn txdata(&mut self) -> TXDATA_W {
        TXDATA_W { w: self }
    }
    #[doc = "Bit 13 - Transmit Data as Break"]
    #[inline(always)]
    pub fn txbreak(&mut self) -> TXBREAK_W {
        TXBREAK_W { w: self }
    }
    #[doc = "Bit 14 - Disable TX After Transmission"]
    #[inline(always)]
    pub fn txdisat(&mut self) -> TXDISAT_W {
        TXDISAT_W { w: self }
    }
    #[doc = "Bit 15 - Enable RX After Transmission"]
    #[inline(always)]
    pub fn rxenat(&mut self) -> RXENAT_W {
        RXENAT_W { w: self }
    }
}
