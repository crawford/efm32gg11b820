#[doc = "Reader of register I2SCTRL"]
pub type R = crate::R<u32, super::I2SCTRL>;
#[doc = "Writer for register I2SCTRL"]
pub type W = crate::W<u32, super::I2SCTRL>;
#[doc = "Register I2SCTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::I2SCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EN`"]
pub type EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EN`"]
pub struct EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EN_W<'a> {
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
#[doc = "Reader of field `MONO`"]
pub type MONO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MONO`"]
pub struct MONO_W<'a> {
    w: &'a mut W,
}
impl<'a> MONO_W<'a> {
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
#[doc = "Reader of field `JUSTIFY`"]
pub type JUSTIFY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `JUSTIFY`"]
pub struct JUSTIFY_W<'a> {
    w: &'a mut W,
}
impl<'a> JUSTIFY_W<'a> {
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
#[doc = "Reader of field `DMASPLIT`"]
pub type DMASPLIT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMASPLIT`"]
pub struct DMASPLIT_W<'a> {
    w: &'a mut W,
}
impl<'a> DMASPLIT_W<'a> {
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
#[doc = "Reader of field `DELAY`"]
pub type DELAY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DELAY`"]
pub struct DELAY_W<'a> {
    w: &'a mut W,
}
impl<'a> DELAY_W<'a> {
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
#[doc = "I2S Word Format\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FORMAT_A {
    #[doc = "0: 32-bit word, 32-bit data"]
    W32D32,
    #[doc = "1: 32-bit word, 32-bit data with 8 lsb masked"]
    W32D24M,
    #[doc = "2: 32-bit word, 24-bit data"]
    W32D24,
    #[doc = "3: 32-bit word, 16-bit data"]
    W32D16,
    #[doc = "4: 32-bit word, 8-bit data"]
    W32D8,
    #[doc = "5: 16-bit word, 16-bit data"]
    W16D16,
    #[doc = "6: 16-bit word, 8-bit data"]
    W16D8,
    #[doc = "7: 8-bit word, 8-bit data"]
    W8D8,
}
impl From<FORMAT_A> for u8 {
    #[inline(always)]
    fn from(variant: FORMAT_A) -> Self {
        match variant {
            FORMAT_A::W32D32 => 0,
            FORMAT_A::W32D24M => 1,
            FORMAT_A::W32D24 => 2,
            FORMAT_A::W32D16 => 3,
            FORMAT_A::W32D8 => 4,
            FORMAT_A::W16D16 => 5,
            FORMAT_A::W16D8 => 6,
            FORMAT_A::W8D8 => 7,
        }
    }
}
#[doc = "Reader of field `FORMAT`"]
pub type FORMAT_R = crate::R<u8, FORMAT_A>;
impl FORMAT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FORMAT_A {
        match self.bits {
            0 => FORMAT_A::W32D32,
            1 => FORMAT_A::W32D24M,
            2 => FORMAT_A::W32D24,
            3 => FORMAT_A::W32D16,
            4 => FORMAT_A::W32D8,
            5 => FORMAT_A::W16D16,
            6 => FORMAT_A::W16D8,
            7 => FORMAT_A::W8D8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `W32D32`"]
    #[inline(always)]
    pub fn is_w32d32(&self) -> bool {
        *self == FORMAT_A::W32D32
    }
    #[doc = "Checks if the value of the field is `W32D24M`"]
    #[inline(always)]
    pub fn is_w32d24m(&self) -> bool {
        *self == FORMAT_A::W32D24M
    }
    #[doc = "Checks if the value of the field is `W32D24`"]
    #[inline(always)]
    pub fn is_w32d24(&self) -> bool {
        *self == FORMAT_A::W32D24
    }
    #[doc = "Checks if the value of the field is `W32D16`"]
    #[inline(always)]
    pub fn is_w32d16(&self) -> bool {
        *self == FORMAT_A::W32D16
    }
    #[doc = "Checks if the value of the field is `W32D8`"]
    #[inline(always)]
    pub fn is_w32d8(&self) -> bool {
        *self == FORMAT_A::W32D8
    }
    #[doc = "Checks if the value of the field is `W16D16`"]
    #[inline(always)]
    pub fn is_w16d16(&self) -> bool {
        *self == FORMAT_A::W16D16
    }
    #[doc = "Checks if the value of the field is `W16D8`"]
    #[inline(always)]
    pub fn is_w16d8(&self) -> bool {
        *self == FORMAT_A::W16D8
    }
    #[doc = "Checks if the value of the field is `W8D8`"]
    #[inline(always)]
    pub fn is_w8d8(&self) -> bool {
        *self == FORMAT_A::W8D8
    }
}
#[doc = "Write proxy for field `FORMAT`"]
pub struct FORMAT_W<'a> {
    w: &'a mut W,
}
impl<'a> FORMAT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FORMAT_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "32-bit word, 32-bit data"]
    #[inline(always)]
    pub fn w32d32(self) -> &'a mut W {
        self.variant(FORMAT_A::W32D32)
    }
    #[doc = "32-bit word, 32-bit data with 8 lsb masked"]
    #[inline(always)]
    pub fn w32d24m(self) -> &'a mut W {
        self.variant(FORMAT_A::W32D24M)
    }
    #[doc = "32-bit word, 24-bit data"]
    #[inline(always)]
    pub fn w32d24(self) -> &'a mut W {
        self.variant(FORMAT_A::W32D24)
    }
    #[doc = "32-bit word, 16-bit data"]
    #[inline(always)]
    pub fn w32d16(self) -> &'a mut W {
        self.variant(FORMAT_A::W32D16)
    }
    #[doc = "32-bit word, 8-bit data"]
    #[inline(always)]
    pub fn w32d8(self) -> &'a mut W {
        self.variant(FORMAT_A::W32D8)
    }
    #[doc = "16-bit word, 16-bit data"]
    #[inline(always)]
    pub fn w16d16(self) -> &'a mut W {
        self.variant(FORMAT_A::W16D16)
    }
    #[doc = "16-bit word, 8-bit data"]
    #[inline(always)]
    pub fn w16d8(self) -> &'a mut W {
        self.variant(FORMAT_A::W16D8)
    }
    #[doc = "8-bit word, 8-bit data"]
    #[inline(always)]
    pub fn w8d8(self) -> &'a mut W {
        self.variant(FORMAT_A::W8D8)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Enable I2S Mode"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Stero or Mono"]
    #[inline(always)]
    pub fn mono(&self) -> MONO_R {
        MONO_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Justification of I2S Data"]
    #[inline(always)]
    pub fn justify(&self) -> JUSTIFY_R {
        JUSTIFY_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Separate DMA Request for Left/Right Data"]
    #[inline(always)]
    pub fn dmasplit(&self) -> DMASPLIT_R {
        DMASPLIT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Delay on I2S Data"]
    #[inline(always)]
    pub fn delay(&self) -> DELAY_R {
        DELAY_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 8:10 - I2S Word Format"]
    #[inline(always)]
    pub fn format(&self) -> FORMAT_R {
        FORMAT_R::new(((self.bits >> 8) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enable I2S Mode"]
    #[inline(always)]
    pub fn en(&mut self) -> EN_W {
        EN_W { w: self }
    }
    #[doc = "Bit 1 - Stero or Mono"]
    #[inline(always)]
    pub fn mono(&mut self) -> MONO_W {
        MONO_W { w: self }
    }
    #[doc = "Bit 2 - Justification of I2S Data"]
    #[inline(always)]
    pub fn justify(&mut self) -> JUSTIFY_W {
        JUSTIFY_W { w: self }
    }
    #[doc = "Bit 3 - Separate DMA Request for Left/Right Data"]
    #[inline(always)]
    pub fn dmasplit(&mut self) -> DMASPLIT_W {
        DMASPLIT_W { w: self }
    }
    #[doc = "Bit 4 - Delay on I2S Data"]
    #[inline(always)]
    pub fn delay(&mut self) -> DELAY_W {
        DELAY_W { w: self }
    }
    #[doc = "Bits 8:10 - I2S Word Format"]
    #[inline(always)]
    pub fn format(&mut self) -> FORMAT_W {
        FORMAT_W { w: self }
    }
}
