#[doc = "Reader of register TXDOUBLEX"]
pub type R = crate::R<u32, super::TXDOUBLEX>;
#[doc = "Writer for register TXDOUBLEX"]
pub type W = crate::W<u32, super::TXDOUBLEX>;
#[doc = "Register TXDOUBLEX `reset()`'s with value 0"]
impl crate::ResetValue for super::TXDOUBLEX {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TXDATA0`"]
pub type TXDATA0_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `TXDATA0`"]
pub struct TXDATA0_W<'a> {
    w: &'a mut W,
}
impl<'a> TXDATA0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01ff) | ((value as u32) & 0x01ff);
        self.w
    }
}
#[doc = "Reader of field `UBRXAT0`"]
pub type UBRXAT0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UBRXAT0`"]
pub struct UBRXAT0_W<'a> {
    w: &'a mut W,
}
impl<'a> UBRXAT0_W<'a> {
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
#[doc = "Reader of field `TXTRIAT0`"]
pub type TXTRIAT0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXTRIAT0`"]
pub struct TXTRIAT0_W<'a> {
    w: &'a mut W,
}
impl<'a> TXTRIAT0_W<'a> {
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
#[doc = "Reader of field `TXBREAK0`"]
pub type TXBREAK0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXBREAK0`"]
pub struct TXBREAK0_W<'a> {
    w: &'a mut W,
}
impl<'a> TXBREAK0_W<'a> {
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
#[doc = "Reader of field `TXDISAT0`"]
pub type TXDISAT0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXDISAT0`"]
pub struct TXDISAT0_W<'a> {
    w: &'a mut W,
}
impl<'a> TXDISAT0_W<'a> {
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
#[doc = "Reader of field `RXENAT0`"]
pub type RXENAT0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXENAT0`"]
pub struct RXENAT0_W<'a> {
    w: &'a mut W,
}
impl<'a> RXENAT0_W<'a> {
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
#[doc = "Reader of field `TXDATA1`"]
pub type TXDATA1_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `TXDATA1`"]
pub struct TXDATA1_W<'a> {
    w: &'a mut W,
}
impl<'a> TXDATA1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01ff << 16)) | (((value as u32) & 0x01ff) << 16);
        self.w
    }
}
#[doc = "Reader of field `UBRXAT1`"]
pub type UBRXAT1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UBRXAT1`"]
pub struct UBRXAT1_W<'a> {
    w: &'a mut W,
}
impl<'a> UBRXAT1_W<'a> {
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
#[doc = "Reader of field `TXTRIAT1`"]
pub type TXTRIAT1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXTRIAT1`"]
pub struct TXTRIAT1_W<'a> {
    w: &'a mut W,
}
impl<'a> TXTRIAT1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Reader of field `TXBREAK1`"]
pub type TXBREAK1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXBREAK1`"]
pub struct TXBREAK1_W<'a> {
    w: &'a mut W,
}
impl<'a> TXBREAK1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "Reader of field `TXDISAT1`"]
pub type TXDISAT1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXDISAT1`"]
pub struct TXDISAT1_W<'a> {
    w: &'a mut W,
}
impl<'a> TXDISAT1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Reader of field `RXENAT1`"]
pub type RXENAT1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXENAT1`"]
pub struct RXENAT1_W<'a> {
    w: &'a mut W,
}
impl<'a> RXENAT1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:8 - TX Data"]
    #[inline(always)]
    pub fn txdata0(&self) -> TXDATA0_R {
        TXDATA0_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bit 11 - Unblock RX After Transmission"]
    #[inline(always)]
    pub fn ubrxat0(&self) -> UBRXAT0_R {
        UBRXAT0_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Set TXTRI After Transmission"]
    #[inline(always)]
    pub fn txtriat0(&self) -> TXTRIAT0_R {
        TXTRIAT0_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Transmit Data as Break"]
    #[inline(always)]
    pub fn txbreak0(&self) -> TXBREAK0_R {
        TXBREAK0_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Clear TXEN After Transmission"]
    #[inline(always)]
    pub fn txdisat0(&self) -> TXDISAT0_R {
        TXDISAT0_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Enable RX After Transmission"]
    #[inline(always)]
    pub fn rxenat0(&self) -> RXENAT0_R {
        RXENAT0_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:24 - TX Data"]
    #[inline(always)]
    pub fn txdata1(&self) -> TXDATA1_R {
        TXDATA1_R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
    #[doc = "Bit 27 - Unblock RX After Transmission"]
    #[inline(always)]
    pub fn ubrxat1(&self) -> UBRXAT1_R {
        UBRXAT1_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Set TXTRI After Transmission"]
    #[inline(always)]
    pub fn txtriat1(&self) -> TXTRIAT1_R {
        TXTRIAT1_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Transmit Data as Break"]
    #[inline(always)]
    pub fn txbreak1(&self) -> TXBREAK1_R {
        TXBREAK1_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Clear TXEN After Transmission"]
    #[inline(always)]
    pub fn txdisat1(&self) -> TXDISAT1_R {
        TXDISAT1_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Enable RX After Transmission"]
    #[inline(always)]
    pub fn rxenat1(&self) -> RXENAT1_R {
        RXENAT1_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:8 - TX Data"]
    #[inline(always)]
    pub fn txdata0(&mut self) -> TXDATA0_W {
        TXDATA0_W { w: self }
    }
    #[doc = "Bit 11 - Unblock RX After Transmission"]
    #[inline(always)]
    pub fn ubrxat0(&mut self) -> UBRXAT0_W {
        UBRXAT0_W { w: self }
    }
    #[doc = "Bit 12 - Set TXTRI After Transmission"]
    #[inline(always)]
    pub fn txtriat0(&mut self) -> TXTRIAT0_W {
        TXTRIAT0_W { w: self }
    }
    #[doc = "Bit 13 - Transmit Data as Break"]
    #[inline(always)]
    pub fn txbreak0(&mut self) -> TXBREAK0_W {
        TXBREAK0_W { w: self }
    }
    #[doc = "Bit 14 - Clear TXEN After Transmission"]
    #[inline(always)]
    pub fn txdisat0(&mut self) -> TXDISAT0_W {
        TXDISAT0_W { w: self }
    }
    #[doc = "Bit 15 - Enable RX After Transmission"]
    #[inline(always)]
    pub fn rxenat0(&mut self) -> RXENAT0_W {
        RXENAT0_W { w: self }
    }
    #[doc = "Bits 16:24 - TX Data"]
    #[inline(always)]
    pub fn txdata1(&mut self) -> TXDATA1_W {
        TXDATA1_W { w: self }
    }
    #[doc = "Bit 27 - Unblock RX After Transmission"]
    #[inline(always)]
    pub fn ubrxat1(&mut self) -> UBRXAT1_W {
        UBRXAT1_W { w: self }
    }
    #[doc = "Bit 28 - Set TXTRI After Transmission"]
    #[inline(always)]
    pub fn txtriat1(&mut self) -> TXTRIAT1_W {
        TXTRIAT1_W { w: self }
    }
    #[doc = "Bit 29 - Transmit Data as Break"]
    #[inline(always)]
    pub fn txbreak1(&mut self) -> TXBREAK1_W {
        TXBREAK1_W { w: self }
    }
    #[doc = "Bit 30 - Clear TXEN After Transmission"]
    #[inline(always)]
    pub fn txdisat1(&mut self) -> TXDISAT1_W {
        TXDISAT1_W { w: self }
    }
    #[doc = "Bit 31 - Enable RX After Transmission"]
    #[inline(always)]
    pub fn rxenat1(&mut self) -> RXENAT1_W {
        RXENAT1_W { w: self }
    }
}
