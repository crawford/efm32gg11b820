#[doc = "Reader of register TEST"]
pub type R = crate::R<u32, super::TEST>;
#[doc = "Writer for register TEST"]
pub type W = crate::W<u32, super::TEST>;
#[doc = "Register TEST `reset()`'s with value 0"]
impl crate::ResetValue for super::TEST {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `BASIC`"]
pub type BASIC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BASIC`"]
pub struct BASIC_W<'a> {
    w: &'a mut W,
}
impl<'a> BASIC_W<'a> {
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
#[doc = "Reader of field `SILENT`"]
pub type SILENT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SILENT`"]
pub struct SILENT_W<'a> {
    w: &'a mut W,
}
impl<'a> SILENT_W<'a> {
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
#[doc = "Reader of field `LBACK`"]
pub type LBACK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LBACK`"]
pub struct LBACK_W<'a> {
    w: &'a mut W,
}
impl<'a> LBACK_W<'a> {
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
#[doc = "Control of CAN_TX Pin\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TX_A {
    #[doc = "0: Reset value, CAN_TX is controlled by the CAN Core."]
    CORE,
    #[doc = "1: Sample Point can be monitored at CAN_TX pin."]
    SAMPT,
    #[doc = "2: CAN_TX pin drives a dominant bit (0) value."]
    LOW,
    #[doc = "3: CAN_TX pin drives a recessive bit (1) value."]
    HIGH,
}
impl From<TX_A> for u8 {
    #[inline(always)]
    fn from(variant: TX_A) -> Self {
        match variant {
            TX_A::CORE => 0,
            TX_A::SAMPT => 1,
            TX_A::LOW => 2,
            TX_A::HIGH => 3,
        }
    }
}
#[doc = "Reader of field `TX`"]
pub type TX_R = crate::R<u8, TX_A>;
impl TX_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TX_A {
        match self.bits {
            0 => TX_A::CORE,
            1 => TX_A::SAMPT,
            2 => TX_A::LOW,
            3 => TX_A::HIGH,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CORE`"]
    #[inline(always)]
    pub fn is_core(&self) -> bool {
        *self == TX_A::CORE
    }
    #[doc = "Checks if the value of the field is `SAMPT`"]
    #[inline(always)]
    pub fn is_sampt(&self) -> bool {
        *self == TX_A::SAMPT
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == TX_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == TX_A::HIGH
    }
}
#[doc = "Write proxy for field `TX`"]
pub struct TX_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TX_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Reset value, CAN_TX is controlled by the CAN Core."]
    #[inline(always)]
    pub fn core(self) -> &'a mut W {
        self.variant(TX_A::CORE)
    }
    #[doc = "Sample Point can be monitored at CAN_TX pin."]
    #[inline(always)]
    pub fn sampt(self) -> &'a mut W {
        self.variant(TX_A::SAMPT)
    }
    #[doc = "CAN_TX pin drives a dominant bit (0) value."]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(TX_A::LOW)
    }
    #[doc = "CAN_TX pin drives a recessive bit (1) value."]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(TX_A::HIGH)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 5)) | (((value as u32) & 0x03) << 5);
        self.w
    }
}
#[doc = "Reader of field `RX`"]
pub type RX_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 2 - Basic Mode"]
    #[inline(always)]
    pub fn basic(&self) -> BASIC_R {
        BASIC_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Silent Mode"]
    #[inline(always)]
    pub fn silent(&self) -> SILENT_R {
        SILENT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Loopback Mode"]
    #[inline(always)]
    pub fn lback(&self) -> LBACK_R {
        LBACK_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 5:6 - Control of CAN_TX Pin"]
    #[inline(always)]
    pub fn tx(&self) -> TX_R {
        TX_R::new(((self.bits >> 5) & 0x03) as u8)
    }
    #[doc = "Bit 7 - Monitors the Actual Value of CAN_RX Pin"]
    #[inline(always)]
    pub fn rx(&self) -> RX_R {
        RX_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Basic Mode"]
    #[inline(always)]
    pub fn basic(&mut self) -> BASIC_W {
        BASIC_W { w: self }
    }
    #[doc = "Bit 3 - Silent Mode"]
    #[inline(always)]
    pub fn silent(&mut self) -> SILENT_W {
        SILENT_W { w: self }
    }
    #[doc = "Bit 4 - Loopback Mode"]
    #[inline(always)]
    pub fn lback(&mut self) -> LBACK_W {
        LBACK_W { w: self }
    }
    #[doc = "Bits 5:6 - Control of CAN_TX Pin"]
    #[inline(always)]
    pub fn tx(&mut self) -> TX_W {
        TX_W { w: self }
    }
}
