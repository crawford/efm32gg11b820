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
#[doc = "Reader of field `TXDATAX`"]
pub type TXDATAX_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `TXDATAX`"]
pub struct TXDATAX_W<'a> {
    w: &'a mut W,
}
impl<'a> TXDATAX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01ff) | ((value as u32) & 0x01ff);
        self.w
    }
}
#[doc = "Reader of field `UBRXAT`"]
pub type UBRXAT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UBRXAT`"]
pub struct UBRXAT_W<'a> {
    w: &'a mut W,
}
impl<'a> UBRXAT_W<'a> {
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
#[doc = "Reader of field `TXTRIAT`"]
pub type TXTRIAT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXTRIAT`"]
pub struct TXTRIAT_W<'a> {
    w: &'a mut W,
}
impl<'a> TXTRIAT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
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
    pub fn txdatax(&self) -> TXDATAX_R {
        TXDATAX_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bit 11 - Unblock RX After Transmission"]
    #[inline(always)]
    pub fn ubrxat(&self) -> UBRXAT_R {
        UBRXAT_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Set TXTRI After Transmission"]
    #[inline(always)]
    pub fn txtriat(&self) -> TXTRIAT_R {
        TXTRIAT_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Transmit Data as Break"]
    #[inline(always)]
    pub fn txbreak(&self) -> TXBREAK_R {
        TXBREAK_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Clear TXEN After Transmission"]
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
    pub fn txdatax(&mut self) -> TXDATAX_W {
        TXDATAX_W { w: self }
    }
    #[doc = "Bit 11 - Unblock RX After Transmission"]
    #[inline(always)]
    pub fn ubrxat(&mut self) -> UBRXAT_W {
        UBRXAT_W { w: self }
    }
    #[doc = "Bit 12 - Set TXTRI After Transmission"]
    #[inline(always)]
    pub fn txtriat(&mut self) -> TXTRIAT_W {
        TXTRIAT_W { w: self }
    }
    #[doc = "Bit 13 - Transmit Data as Break"]
    #[inline(always)]
    pub fn txbreak(&mut self) -> TXBREAK_W {
        TXBREAK_W { w: self }
    }
    #[doc = "Bit 14 - Clear TXEN After Transmission"]
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
