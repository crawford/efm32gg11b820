#[doc = "Reader of register STATUS"]
pub type R = crate::R<u32, super::STATUS>;
#[doc = "Writer for register STATUS"]
pub type W = crate::W<u32, super::STATUS>;
#[doc = "Register STATUS `reset()`'s with value 0"]
impl crate::ResetValue for super::STATUS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Last Error Code\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LEC_A {
    #[doc = "0:  No error occurred during last CAN bus event."]
    NONE,
    #[doc = "1: More than 5 equal bits in a sequence have occurred in a part of a received message where this is not allowed."]
    STUFF,
    #[doc = "2: A fixed format part of a received frame has the wrong format."]
    FORM,
    #[doc = "3: The message this CAN Core transmitted was not acknowledged by another node."]
    ACK,
    #[doc = "4: During the transmission of a message (with the exception of the arbitration field), the device wanted to send a recessive level (bit of logical value 1), but the monitored bus value was dominant."]
    BIT1,
    #[doc = "5: During the transmission of a message (or acknowledge bit, or active error flag, or overload flag), the device wanted to send a dominant level (data or identifier bit logical value 0), but the monitored Bus value was recessive. During Bus Off recovery this status is set each time a sequence of 11 recessive bits has been monitored. This enables the CPU to monitor the proceeding of the Bus Off recovery sequence (indicating the bus is not stuck at dominant or continuously disturbed)."]
    BIT0,
    #[doc = "6: The CRC check sum was incorrect in the message received; the CRC received for an incoming message does not match with the calculated CRC for the received data."]
    CRC,
    #[doc = "7: When the LEC shows the value '7', no CAN bus event was detected since the CPU wrote this value to the LEC."]
    UNUSED,
}
impl From<LEC_A> for u8 {
    #[inline(always)]
    fn from(variant: LEC_A) -> Self {
        match variant {
            LEC_A::NONE => 0,
            LEC_A::STUFF => 1,
            LEC_A::FORM => 2,
            LEC_A::ACK => 3,
            LEC_A::BIT1 => 4,
            LEC_A::BIT0 => 5,
            LEC_A::CRC => 6,
            LEC_A::UNUSED => 7,
        }
    }
}
#[doc = "Reader of field `LEC`"]
pub type LEC_R = crate::R<u8, LEC_A>;
impl LEC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LEC_A {
        match self.bits {
            0 => LEC_A::NONE,
            1 => LEC_A::STUFF,
            2 => LEC_A::FORM,
            3 => LEC_A::ACK,
            4 => LEC_A::BIT1,
            5 => LEC_A::BIT0,
            6 => LEC_A::CRC,
            7 => LEC_A::UNUSED,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == LEC_A::NONE
    }
    #[doc = "Checks if the value of the field is `STUFF`"]
    #[inline(always)]
    pub fn is_stuff(&self) -> bool {
        *self == LEC_A::STUFF
    }
    #[doc = "Checks if the value of the field is `FORM`"]
    #[inline(always)]
    pub fn is_form(&self) -> bool {
        *self == LEC_A::FORM
    }
    #[doc = "Checks if the value of the field is `ACK`"]
    #[inline(always)]
    pub fn is_ack(&self) -> bool {
        *self == LEC_A::ACK
    }
    #[doc = "Checks if the value of the field is `BIT1`"]
    #[inline(always)]
    pub fn is_bit1(&self) -> bool {
        *self == LEC_A::BIT1
    }
    #[doc = "Checks if the value of the field is `BIT0`"]
    #[inline(always)]
    pub fn is_bit0(&self) -> bool {
        *self == LEC_A::BIT0
    }
    #[doc = "Checks if the value of the field is `CRC`"]
    #[inline(always)]
    pub fn is_crc(&self) -> bool {
        *self == LEC_A::CRC
    }
    #[doc = "Checks if the value of the field is `UNUSED`"]
    #[inline(always)]
    pub fn is_unused(&self) -> bool {
        *self == LEC_A::UNUSED
    }
}
#[doc = "Write proxy for field `LEC`"]
pub struct LEC_W<'a> {
    w: &'a mut W,
}
impl<'a> LEC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LEC_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "No error occurred during last CAN bus event."]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(LEC_A::NONE)
    }
    #[doc = "More than 5 equal bits in a sequence have occurred in a part of a received message where this is not allowed."]
    #[inline(always)]
    pub fn stuff(self) -> &'a mut W {
        self.variant(LEC_A::STUFF)
    }
    #[doc = "A fixed format part of a received frame has the wrong format."]
    #[inline(always)]
    pub fn form(self) -> &'a mut W {
        self.variant(LEC_A::FORM)
    }
    #[doc = "The message this CAN Core transmitted was not acknowledged by another node."]
    #[inline(always)]
    pub fn ack(self) -> &'a mut W {
        self.variant(LEC_A::ACK)
    }
    #[doc = "During the transmission of a message (with the exception of the arbitration field), the device wanted to send a recessive level (bit of logical value 1), but the monitored bus value was dominant."]
    #[inline(always)]
    pub fn bit1(self) -> &'a mut W {
        self.variant(LEC_A::BIT1)
    }
    #[doc = "During the transmission of a message (or acknowledge bit, or active error flag, or overload flag), the device wanted to send a dominant level (data or identifier bit logical value 0), but the monitored Bus value was recessive. During Bus Off recovery this status is set each time a sequence of 11 recessive bits has been monitored. This enables the CPU to monitor the proceeding of the Bus Off recovery sequence (indicating the bus is not stuck at dominant or continuously disturbed)."]
    #[inline(always)]
    pub fn bit0(self) -> &'a mut W {
        self.variant(LEC_A::BIT0)
    }
    #[doc = "The CRC check sum was incorrect in the message received; the CRC received for an incoming message does not match with the calculated CRC for the received data."]
    #[inline(always)]
    pub fn crc(self) -> &'a mut W {
        self.variant(LEC_A::CRC)
    }
    #[doc = "When the LEC shows the value '7', no CAN bus event was detected since the CPU wrote this value to the LEC."]
    #[inline(always)]
    pub fn unused(self) -> &'a mut W {
        self.variant(LEC_A::UNUSED)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "Reader of field `TXOK`"]
pub type TXOK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXOK`"]
pub struct TXOK_W<'a> {
    w: &'a mut W,
}
impl<'a> TXOK_W<'a> {
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
#[doc = "Reader of field `RXOK`"]
pub type RXOK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXOK`"]
pub struct RXOK_W<'a> {
    w: &'a mut W,
}
impl<'a> RXOK_W<'a> {
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
#[doc = "Reader of field `EPASS`"]
pub type EPASS_R = crate::R<bool, bool>;
#[doc = "Reader of field `EWARN`"]
pub type EWARN_R = crate::R<bool, bool>;
#[doc = "Reader of field `BOFF`"]
pub type BOFF_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 0:2 - Last Error Code"]
    #[inline(always)]
    pub fn lec(&self) -> LEC_R {
        LEC_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 3 - Transmitted a Message Successfully"]
    #[inline(always)]
    pub fn txok(&self) -> TXOK_R {
        TXOK_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Received a Message Successfully"]
    #[inline(always)]
    pub fn rxok(&self) -> RXOK_R {
        RXOK_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Error Passive"]
    #[inline(always)]
    pub fn epass(&self) -> EPASS_R {
        EPASS_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Warning Status"]
    #[inline(always)]
    pub fn ewarn(&self) -> EWARN_R {
        EWARN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Bus Off Status"]
    #[inline(always)]
    pub fn boff(&self) -> BOFF_R {
        BOFF_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Last Error Code"]
    #[inline(always)]
    pub fn lec(&mut self) -> LEC_W {
        LEC_W { w: self }
    }
    #[doc = "Bit 3 - Transmitted a Message Successfully"]
    #[inline(always)]
    pub fn txok(&mut self) -> TXOK_W {
        TXOK_W { w: self }
    }
    #[doc = "Bit 4 - Received a Message Successfully"]
    #[inline(always)]
    pub fn rxok(&mut self) -> RXOK_W {
        RXOK_W { w: self }
    }
}
